use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    PARAMETER,
    IMMEDIATE,
}

struct IntCode {
    positions: Vec<i32>,
}

impl IntCode {
    fn parse_opcode(raw_code: i32) -> (i32, HashMap<usize, Mode>) {
        let mut parameter_modes: HashMap<usize, Mode> = HashMap::new();
        let opcode = raw_code % 100;

        let mut mode_digits = (raw_code - opcode) / 100;
        let mut offset: usize = 1;
        while mode_digits > 0 {
            let digit = mode_digits % 10;
            mode_digits = (mode_digits - digit) / 10;
            let mode = match digit {
                0 => Mode::PARAMETER,
                1 => Mode::IMMEDIATE,
                _ => panic!("Mode digit should only be 0 or 1"),
            };
            parameter_modes.insert(offset, mode);
            offset += 1
        }
        (opcode, parameter_modes)
    }

    fn get_value(&self, index: usize, mode: &Mode) -> i32 {
        match mode {
            Mode::IMMEDIATE => self.positions[index],
            Mode::PARAMETER => {
                let value_index = self.positions[index];
                self.positions[value_index as usize]
            }
        }
    }

    fn opcode_1(&mut self, index: &mut usize, parameter_modes: HashMap<usize, Mode>) {
        let value_1 = self.get_value(
            *index + 1,
            parameter_modes.get(&1).unwrap_or(&Mode::PARAMETER),
        );
        let value_2 = self.get_value(
            *index + 2,
            parameter_modes.get(&2).unwrap_or(&Mode::PARAMETER),
        );
        let change_index = self.positions[*index + 3];
        self.positions[change_index as usize] = value_1 + value_2;
        *index += 4;
    }

    fn opcode_2(&mut self, index: &mut usize, parameter_modes: HashMap<usize, Mode>) {
        let value_1 = self.get_value(
            *index + 1,
            parameter_modes.get(&1).unwrap_or(&Mode::PARAMETER),
        );
        let value_2 = self.get_value(
            *index + 2,
            parameter_modes.get(&2).unwrap_or(&Mode::PARAMETER),
        );
        let change_index = self.positions[*index + 3];
        self.positions[change_index as usize] = value_1 * value_2;
        *index += 4;
    }

    fn opcode_3(&mut self, index: &mut usize, input: i32) {
        let change_index = self.positions[*index + 1];
        self.positions[change_index as usize] = input;
        *index += 2
    }

    fn opcode_4(&mut self, index: &mut usize, parameter_modes: HashMap<usize, Mode>) -> i32 {
        let output = self.get_value(
            *index + 1,
            parameter_modes.get(&1).unwrap_or(&Mode::PARAMETER),
        );
        *index += 2;
        output
    }

    fn process(&mut self) -> Option<i32> {
        let mut index: usize = 0;
        let mut last_output: Option<i32> = None;
        loop {
            let raw_code = self.positions[index];
            let (opcode, parameter_modes) = IntCode::parse_opcode(raw_code);

            if opcode == 99 {
                return last_output;
            } else if opcode == 1 {
                self.opcode_1(&mut index, parameter_modes);
            } else if opcode == 2 {
                self.opcode_2(&mut index, parameter_modes);
            } else if opcode == 3 {
                self.opcode_3(&mut index, 1);
            } else if opcode == 4 {
                let next_output = self.opcode_4(&mut index, parameter_modes);
                if last_output.is_some() && last_output.unwrap() > 0 {
                    panic!("Diagnostic test failed")
                } else {
                    last_output = Some(next_output)
                }
            }
        }
    }
}

fn part_1(contents: &str) -> i32 {
    let input: Vec<i32> = contents
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut intcode = IntCode { positions: input };
    intcode.process().unwrap()
}

fn part_2(contents: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
        vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    )]
    #[case(
        vec![2, 3, 0, 3, 99],
        vec![2, 3, 0, 6, 99]
    )]
    #[case(
        vec![2, 4, 4, 5, 99, 0],
        vec![2, 4, 4, 5, 99, 9801]
    )]
    #[case(
        vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
    )]
    fn test_opcodes_1_and_2(#[case] input: Vec<i32>, #[case] expected: Vec<i32>) {
        let mut intcode = IntCode { positions: input };
        intcode.process();
        assert_eq!(intcode.positions, expected);
    }

    #[rstest]
    #[case(99, (99, HashMap::new()))]
    #[case(101, (1, [(1, Mode::IMMEDIATE)].into_iter().collect()))]
    #[case(1003, (3, [(1, Mode::PARAMETER), (2, Mode::IMMEDIATE)].into_iter().collect()))]
    #[case(10104, (4, [(1, Mode::IMMEDIATE), (2, Mode::PARAMETER), (3, Mode::IMMEDIATE)].into_iter().collect()))]
    fn test_parse_opcodes(#[case] raw_code: i32, #[case] expected: (i32, HashMap<usize, Mode>)) {
        assert_eq!(IntCode::parse_opcode(raw_code), expected);
    }

    #[test]
    fn test_process_after_parsing_opcode() {
        let mut intcode = IntCode {
            positions: vec![1002, 4, 3, 4, 33],
        };
        intcode.process();
        assert_eq!(intcode.positions, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_negative_entry() {
        let mut intcode = IntCode {
            positions: vec![1101, 100, -1, 4, 0],
        };
        intcode.process();
        assert_eq!(intcode.positions, vec![1101, 100, -1, 4, 99]);
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
