use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
enum ParameterMode {
    POSITION,
    IMMEDIATE,
}

struct IntCode {
    positions: Vec<i32>,
}

impl IntCode {
    fn parse_opcode(raw_code: i32) -> (i32, HashMap<usize, ParameterMode>) {
        let mut parameter_modes: HashMap<usize, ParameterMode> = HashMap::new();
        let opcode = raw_code % 100;

        let mut mode_digits = (raw_code - opcode) / 100;
        let mut offset: usize = 1;
        while mode_digits > 0 {
            let digit = mode_digits % 10;
            mode_digits = (mode_digits - digit) / 10;
            let mode = match digit {
                0 => ParameterMode::POSITION,
                1 => ParameterMode::IMMEDIATE,
                _ => panic!("ParameterMode digit should only be 0 or 1"),
            };
            parameter_modes.insert(offset, mode);
            offset += 1
        }
        (opcode, parameter_modes)
    }

    fn get_value_at_index(&self, index: usize, mode: &ParameterMode) -> i32 {
        match mode {
            ParameterMode::IMMEDIATE => self.positions[index],
            ParameterMode::POSITION => {
                let value_index = self.positions[index];
                self.positions[value_index as usize]
            }
        }
    }

    fn get_parameters(
        &self,
        index: usize,
        num_parameters: usize,
        parameter_modes: HashMap<usize, ParameterMode>,
    ) -> Vec<i32> {
        (1..=num_parameters)
            .map(|x| {
                self.get_value_at_index(
                    index + x,
                    parameter_modes.get(&x).unwrap_or(&ParameterMode::POSITION),
                )
            })
            .collect()
    }

    fn opcode_1(&mut self, index: &mut usize, mut parameter_modes: HashMap<usize, ParameterMode>) {
        let num_parameters = 3;
        parameter_modes.entry(3).or_insert(ParameterMode::IMMEDIATE);
        let parameters = self.get_parameters(*index, num_parameters, parameter_modes);
        self.positions[parameters[2] as usize] = parameters[0] + parameters[1];
        *index += num_parameters + 1;
    }

    fn opcode_2(&mut self, index: &mut usize, mut parameter_modes: HashMap<usize, ParameterMode>) {
        let num_parameters = 3;
        parameter_modes.entry(3).or_insert(ParameterMode::IMMEDIATE);
        let parameters = self.get_parameters(*index, num_parameters, parameter_modes);
        self.positions[parameters[2] as usize] = parameters[0] * parameters[1];
        *index += num_parameters + 1;
    }

    fn opcode_3(&mut self, index: &mut usize, input: i32) {
        let change_index = self.positions[*index + 1];
        self.positions[change_index as usize] = input;
        *index += 2
    }

    fn opcode_4(
        &mut self,
        index: &mut usize,
        parameter_modes: HashMap<usize, ParameterMode>,
    ) -> i32 {
        let output = self.get_value_at_index(
            *index + 1,
            parameter_modes.get(&1).unwrap_or(&ParameterMode::POSITION),
        );
        *index += 2;
        output
    }

    fn opcode_5(&mut self, index: &mut usize, parameter_modes: HashMap<usize, ParameterMode>) {
        let num_parameters = 2;
        let parameters = self.get_parameters(*index, num_parameters, parameter_modes);
        if parameters[0] > 0 {
            *index = parameters[1] as usize;
        } else {
            *index += num_parameters + 1;
        }
    }

    fn opcode_6(&mut self, index: &mut usize, parameter_modes: HashMap<usize, ParameterMode>) {
        let num_parameters = 2;
        let parameters = self.get_parameters(*index, num_parameters, parameter_modes);
        if parameters[0] == 0 {
            *index = parameters[1] as usize;
        } else {
            *index += num_parameters + 1;
        }
    }

    fn opcode_7(&mut self, index: &mut usize, mut parameter_modes: HashMap<usize, ParameterMode>) {
        let num_parameters = 3;
        parameter_modes.entry(3).or_insert(ParameterMode::IMMEDIATE);
        let parameters = self.get_parameters(*index, num_parameters, parameter_modes);
        self.positions[parameters[2] as usize] = (parameters[0] < parameters[1]) as i32;
        *index += num_parameters + 1;
    }

    fn opcode_8(&mut self, index: &mut usize, mut parameter_modes: HashMap<usize, ParameterMode>) {
        let num_parameters = 3;
        parameter_modes.entry(3).or_insert(ParameterMode::IMMEDIATE);
        let parameters = self.get_parameters(*index, num_parameters, parameter_modes);
        self.positions[parameters[2] as usize] = (parameters[0] == parameters[1]) as i32;
        *index += num_parameters + 1;
    }

    fn process(&mut self, input: i32) -> Option<i32> {
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
                self.opcode_3(&mut index, input);
            } else if opcode == 4 {
                let next_output = self.opcode_4(&mut index, parameter_modes);
                if last_output.is_some() && last_output.unwrap() > 0 {
                    panic!("Diagnostic test failed")
                } else {
                    last_output = Some(next_output)
                }
            } else if opcode == 5 {
                self.opcode_5(&mut index, parameter_modes);
            } else if opcode == 6 {
                self.opcode_6(&mut index, parameter_modes);
            } else if opcode == 7 {
                self.opcode_7(&mut index, parameter_modes);
            } else if opcode == 8 {
                self.opcode_8(&mut index, parameter_modes);
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
    intcode.process(1).unwrap()
}

fn part_2(contents: &str) -> i32 {
    let input: Vec<i32> = contents
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut intcode = IntCode { positions: input };
    intcode.process(5).unwrap()
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
    fn test_opcodes_1_and_2(#[case] positions: Vec<i32>, #[case] expected: Vec<i32>) {
        let mut intcode = IntCode { positions };
        intcode.process(1);
        assert_eq!(intcode.positions, expected);
    }

    #[rstest]
    #[case(99, (99, HashMap::new()))]
    #[case(101, (1, [(1, ParameterMode::IMMEDIATE)].into_iter().collect()))]
    #[case(1003, (3, [(1, ParameterMode::POSITION), (2, ParameterMode::IMMEDIATE)].into_iter().collect()))]
    #[case(10104, (4, [(1, ParameterMode::IMMEDIATE), (2, ParameterMode::POSITION), (3, ParameterMode::IMMEDIATE)].into_iter().collect()))]
    fn test_parse_opcodes(
        #[case] raw_code: i32,
        #[case] expected: (i32, HashMap<usize, ParameterMode>),
    ) {
        assert_eq!(IntCode::parse_opcode(raw_code), expected);
    }

    #[test]
    fn test_process_after_parsing_opcode() {
        let mut intcode = IntCode {
            positions: vec![1002, 4, 3, 4, 33],
        };
        intcode.process(1);
        assert_eq!(intcode.positions, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_negative_entry() {
        let mut intcode = IntCode {
            positions: vec![1101, 100, -1, 4, 0],
        };
        intcode.process(1);
        assert_eq!(intcode.positions, vec![1101, 100, -1, 4, 99]);
    }

    #[rstest]
    #[case(
        vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
        8,
        1
    )]
    #[case(
        vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
        7,
        0
    )]
    #[case(
        vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
        6,
        1
    )]
    #[case(
        vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
        9,
        0
    )]
    #[case(
        vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
        8,
        1
    )]
    #[case(
        vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
        100,
        0
    )]
    #[case(
        vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
        2,
        1
    )]
    #[case(
        vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
        340,
        0
    )]
    fn test_opcodes_7_and_8(
        #[case] positions: Vec<i32>,
        #[case] input: i32,
        #[case] expected: i32,
    ) {
        let mut intcode = IntCode { positions };
        assert_eq!(intcode.process(input).unwrap(), expected);
    }

    #[rstest]
    #[case(
        vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
        0,
        0
    )]
    #[case(
        vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
        1,
        1
    )]
    #[case(
        vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
        0,
        0
    )]
    #[case(
        vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
        34,
        1
    )]
    fn test_opcodes_5_and_6(
        #[case] positions: Vec<i32>,
        #[case] input: i32,
        #[case] expected: i32,
    ) {
        let mut intcode = IntCode { positions };
        assert_eq!(intcode.process(input).unwrap(), expected);
    }

    #[rstest]
    #[case(8, 1000)]
    #[case(6, 999)]
    #[case(9, 1001)]
    fn test_larger_example(#[case] input: i32, #[case] expected: i32) {
        let positions = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut intcode = IntCode {
            positions: positions.clone(),
        };
        assert_eq!(intcode.process(input).unwrap(), expected);
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
