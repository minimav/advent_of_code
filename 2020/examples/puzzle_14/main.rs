use itertools::Itertools;
use std::collections::HashMap;
use std::time::Instant;

static MASK_LENGTH: u32 = 36;
static VALID_CHARS: &str = "X01";

#[derive(PartialEq, Eq, Debug)]
struct BitMaskV1 {
    zeros_mask: u128,
    ones_mask: u128,
}

impl BitMaskV1 {
    fn new(raw_mask: &str) -> Result<Self, ()> {
        let all_valid_chars = raw_mask.chars().all(|x| VALID_CHARS.contains(x));
        if raw_mask.len() as u32 != MASK_LENGTH || !all_valid_chars {
            return Err(());
        }
        let mut zeros_mask: u128 = 0;
        let mut ones_mask: u128 = 0;
        let mut index: u32 = 0;
        for char in raw_mask.chars() {
            match char.to_digit(10) {
                Some(b) => {
                    let power = 2_u128.pow(MASK_LENGTH - index - 1) as u128;
                    if b > 0 {
                        ones_mask += power
                    } else {
                        zeros_mask += power
                    }
                }
                None => {}
            }
            index += 1
        }
        Ok(BitMaskV1 {
            zeros_mask,
            ones_mask,
        })
    }

    fn transform(&self, input: u128) -> u128 {
        (input | self.ones_mask) - (input & self.zeros_mask)
    }
}

impl Default for BitMaskV1 {
    fn default() -> BitMaskV1 {
        BitMaskV1 {
            zeros_mask: 0,
            ones_mask: 0,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct BitMaskV2 {
    xs_mask: u128,
    ones_mask: u128,
}

impl BitMaskV2 {
    fn new(raw_mask: &str) -> Result<Self, ()> {
        let all_valid_chars = raw_mask.chars().all(|x| VALID_CHARS.contains(x));
        if raw_mask.len() as u32 != MASK_LENGTH || !all_valid_chars {
            return Err(());
        }
        let mut xs_mask: u128 = 0;
        let mut ones_mask: u128 = 0;
        let mut index: u32 = 0;
        for char in raw_mask.chars() {
            let power = 2_u128.pow(MASK_LENGTH - index - 1) as u128;
            match char.to_digit(10) {
                Some(b) => {
                    if b > 0 {
                        ones_mask += power
                    }
                }
                None => xs_mask += power,
            }
            index += 1
        }
        Ok(BitMaskV2 { xs_mask, ones_mask })
    }

    fn locations(&self, input: u128) -> Vec<u128> {
        // apply mask of 1s and set floating bits to 0 before we
        // iterate over all possibilities
        let input_pre_floats = (input | self.ones_mask) - (input & self.xs_mask);

        let mut components: Vec<u128> = Vec::new();
        for exponent in 0..MASK_LENGTH {
            let power = 2_u128.pow(exponent) as u128;
            if (power & self.xs_mask) != 0 {
                components.push(power)
            }
        }

        let mut transformed_locations: Vec<u128> = Vec::new();
        for selected in components.iter().powerset() {
            let number_to_add: u128 = selected.into_iter().sum();
            transformed_locations.push(input_pre_floats + number_to_add);
        }

        transformed_locations
    }
}

impl Default for BitMaskV2 {
    fn default() -> BitMaskV2 {
        BitMaskV2 {
            xs_mask: 0,
            ones_mask: 0,
        }
    }
}

fn parse_memory_line(line: &str) -> (u128, u128) {
    // line looks like mem[<int>] = <int> where we want the <int>s
    let split = line.split(" = ").collect::<Vec<&str>>();
    // remove mem[ from start and ] from end
    let memory_location = split[0][4..split[0].len() - 1].parse::<u128>().unwrap();
    let memory_value = split[1].parse::<u128>().unwrap();
    (memory_location, memory_value)
}

fn part_1(contents: &str) -> u128 {
    let mut memory: HashMap<u128, u128> = HashMap::new();
    let mut mask = BitMaskV1::default();
    for line in contents.lines() {
        if line.starts_with("mask") {
            let raw_mask = line.strip_prefix("mask = ").unwrap();
            match BitMaskV1::new(raw_mask) {
                Ok(m) => mask = m,
                Err(_) => panic!("Could not parse bit mask"),
            };
        } else {
            let (memory_location, initial_value) = parse_memory_line(line);
            memory.insert(memory_location, mask.transform(initial_value));
        }
    }
    memory.values().sum()
}

fn part_2(contents: &str) -> u128 {
    let mut memory: HashMap<u128, u128> = HashMap::new();
    let mut mask = BitMaskV2::default();
    for line in contents.lines() {
        if line.starts_with("mask") {
            let raw_mask = line.strip_prefix("mask = ").unwrap();
            match BitMaskV2::new(raw_mask) {
                Ok(m) => mask = m,
                Err(_) => panic!("Could not parse bit mask"),
            };
        } else {
            let (memory_location, memory_value) = parse_memory_line(line);
            for transformed_location in mask.locations(memory_location).into_iter() {
                memory.insert(transformed_location, memory_value);
            }
        }
    }
    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 2, 64)]
    #[case("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX11", 0, 3)]
    #[case("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX010", 5, 2)]
    fn test_bit_mask_to_int(
        #[case] raw_mask: &str,
        #[case] expected_zeros_mask: u128,
        #[case] expected_ones_mask: u128,
    ) {
        let loaded = BitMaskV1::new(raw_mask);
        assert_eq!(
            loaded.unwrap(),
            BitMaskV1 {
                zeros_mask: expected_zeros_mask,
                ones_mask: expected_ones_mask
            }
        );
    }

    #[rstest]
    #[case("XXX0101")]
    #[case("X1")]
    #[case("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0XXXXXXXXXXX")]
    fn test_error_for_invalid_length(#[case] raw_mask: &str) {
        let result = match BitMaskV1::new(raw_mask) {
            Ok(_) => false,
            Err(_) => true,
        };
        assert_eq!(result, true);
    }

    #[rstest]
    #[case("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX02")]
    #[case("XXXXAXXXBXXXXXXXXXXXXXXXXXXXX1XXXX00")]
    fn test_error_for_invalid_characters(#[case] raw_mask: &str) {
        let result = match BitMaskV1::new(raw_mask) {
            Ok(_) => false,
            Err(_) => true,
        };
        assert_eq!(result, true);
    }

    #[rstest]
    #[case("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 11, 73)]
    fn test_applying_bit_mask(
        #[case] raw_mask: &str,
        #[case] input: u128,
        #[case] expected_output: u128,
    ) {
        let bit_mask = BitMaskV1::new(raw_mask).unwrap();
        assert_eq!(bit_mask.transform(input), expected_output);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example_1.txt")), 165);
    }

    #[test]
    fn test_part_2_example_1() {
        assert_eq!(part_2(include_str!("./example_2.txt")), 208);
    }

    #[test]
    fn test_part_2_example_2() {
        let input = "mask = 0XX000X1111001010X10XX1101XX00X00100
mem[50596] = 1000
mask = 0X000001111001010X1011100100001X0X0X
mem[45713] = 1";
        assert_eq!(part_2(input), 508032);
    }

    #[test]
    fn test_part_2_example_3() {
        let input = "mask = 000000000000000000000000000000000XXX
mem[8] = 4
mask = XX0000000000000000000000000000000000
mem[0] = 5";
        assert_eq!(part_2(input), 52);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let part_1_answer = part_1(contents);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer = part_2(contents);
    println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
