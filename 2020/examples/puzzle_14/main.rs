use std::collections::HashMap;
use std::time::Instant;

static MASK_LENGTH: u32 = 36;
static VALID_CHARS: &str = "X01";

#[derive(PartialEq, Eq, Debug)]
struct BitMask {
    zeros_mask: u64,
    ones_mask: u64,
}

impl BitMask {
    fn new(raw_mask: &str) -> Result<Self, ()> {
        let all_valid_chars = raw_mask.chars().all(|x| VALID_CHARS.contains(x));
        if raw_mask.len() as u32 != MASK_LENGTH || !all_valid_chars {
            return Err(());
        }
        let mut zeros_mask: u64 = 0;
        let mut ones_mask: u64 = 0;
        let mut index: u32 = 0;
        for char in raw_mask.chars() {
            match char.to_digit(10) {
                Some(b) => {
                    let power = 2_u64.pow(MASK_LENGTH - index - 1) as u64;
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
        Ok(BitMask {
            zeros_mask,
            ones_mask,
        })
    }

    fn transform(&self, input: u64) -> u64 {
        (input | self.ones_mask) - (input & self.zeros_mask)
    }
}

impl Default for BitMask {
    fn default() -> BitMask {
        BitMask {
            zeros_mask: 0,
            ones_mask: 0,
        }
    }
}

fn part_1(contents: &str) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = BitMask::default();
    for line in contents.lines() {
        if line.starts_with("mask") {
            let raw_mask = line.strip_prefix("mask = ").unwrap();
            match BitMask::new(raw_mask) {
                Ok(m) => mask = m,
                Err(_) => panic!("Could not parse bit mask"),
            };
        } else {
            // line looks like mem[<int>] = <int> where we want the <int>s
            let split = line.split(" = ").collect::<Vec<&str>>();
            // remove mem[ from start and ] from end
            let memory_location = split[0][4..split[0].len() - 1].parse::<u64>().unwrap();
            let initial_value = split[1].parse::<u64>().unwrap();

            memory.insert(memory_location, mask.transform(initial_value));
        }
    }
    memory.values().sum()
}

fn part_2(contents: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 64, 2)]
    #[case("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX11", 3, 0)]
    #[case("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX010", 2, 5)]
    fn test_bit_mask_to_int(
        #[case] raw_mask: &str,
        #[case] expected_zeros_mask: u64,
        #[case] expected_ones_mask: u64,
    ) {
        let loaded = BitMask::new(raw_mask);
        assert_eq!(
            loaded.unwrap(),
            BitMask {
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
        let result = match BitMask::new(raw_mask) {
            Ok(_) => false,
            Err(_) => true,
        };
        assert_eq!(result, true);
    }

    #[rstest]
    #[case("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX02")]
    #[case("XXXXAXXXBXXXXXXXXXXXXXXXXXXXX1XXXX00")]
    fn test_error_for_invalid_characters(#[case] raw_mask: &str) {
        let result = match BitMask::new(raw_mask) {
            Ok(_) => false,
            Err(_) => true,
        };
        assert_eq!(result, true);
    }

    #[rstest]
    #[case("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 11, 73)]
    fn test_applying_bit_mask(
        #[case] raw_mask: &str,
        #[case] input: u64,
        #[case] expected_output: u64,
    ) {
        let bit_mask = BitMask::new(raw_mask).unwrap();
        assert_eq!(bit_mask.transform(input), expected_output);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 165);
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
