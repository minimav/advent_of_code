use std::collections::HashMap;
use std::time::Instant;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    PARAMETER,
    IMMEDIATE,
}

#[derive(Debug)]
struct ThrusterSignal {
    phase_settings: Vec<i64>,
    signal: i64,
}

struct IntCode {
    memory: Vec<i64>,
}

impl IntCode {
    fn parse_opcode(raw_code: i64) -> (i64, HashMap<usize, Mode>) {
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

    fn get_value_at_index(&self, index: usize, mode: &Mode) -> i64 {
        match mode {
            Mode::IMMEDIATE => self.memory[index],
            Mode::PARAMETER => {
                let value_index = self.memory[index];
                self.memory[value_index as usize]
            }
        }
    }

    fn get_parameters(
        &self,
        index: usize,
        num_parameters: usize,
        parameter_modes: HashMap<usize, Mode>,
    ) -> Vec<i64> {
        (1..=num_parameters)
            .map(|x| {
                self.get_value_at_index(
                    index + x,
                    parameter_modes.get(&x).unwrap_or(&Mode::PARAMETER),
                )
            })
            .collect()
    }

    fn opcode_1(&mut self, index: &mut usize, mut parameter_modes: HashMap<usize, Mode>) {
        let num_parameters = 3;
        parameter_modes.entry(3).or_insert(Mode::IMMEDIATE);
        let parameters = self.get_parameters(*index, num_parameters, parameter_modes);
        self.memory[parameters[2] as usize] = parameters[0] + parameters[1];
        *index += num_parameters + 1;
    }

    fn opcode_2(&mut self, index: &mut usize, mut parameter_modes: HashMap<usize, Mode>) {
        let num_parameters = 3;
        parameter_modes.entry(3).or_insert(Mode::IMMEDIATE);
        let parameters = self.get_parameters(*index, num_parameters, parameter_modes);
        self.memory[parameters[2] as usize] = parameters[0] * parameters[1];
        *index += num_parameters + 1;
    }

    fn opcode_3(&mut self, index: &mut usize, input: i64) {
        let change_index = self.memory[*index + 1];
        self.memory[change_index as usize] = input;
        *index += 2
    }

    fn opcode_4(&mut self, index: &mut usize, parameter_modes: HashMap<usize, Mode>) -> i64 {
        let output = self.get_value_at_index(
            *index + 1,
            parameter_modes.get(&1).unwrap_or(&Mode::PARAMETER),
        );
        *index += 2;
        output
    }

    fn opcode_5(&mut self, index: &mut usize, parameter_modes: HashMap<usize, Mode>) {
        let num_parameters = 2;
        let parameters = self.get_parameters(*index, num_parameters, parameter_modes);
        if parameters[0] > 0 {
            *index = parameters[1] as usize;
        } else {
            *index += num_parameters + 1;
        }
    }

    fn opcode_6(&mut self, index: &mut usize, parameter_modes: HashMap<usize, Mode>) {
        let num_parameters = 2;
        let parameters = self.get_parameters(*index, num_parameters, parameter_modes);
        if parameters[0] == 0 {
            *index = parameters[1] as usize;
        } else {
            *index += num_parameters + 1;
        }
    }

    fn opcode_7(&mut self, index: &mut usize, mut parameter_modes: HashMap<usize, Mode>) {
        let num_parameters = 3;
        parameter_modes.entry(3).or_insert(Mode::IMMEDIATE);
        let parameters = self.get_parameters(*index, num_parameters, parameter_modes);
        self.memory[parameters[2] as usize] = (parameters[0] < parameters[1]) as i64;
        *index += num_parameters + 1;
    }

    fn opcode_8(&mut self, index: &mut usize, mut parameter_modes: HashMap<usize, Mode>) {
        let num_parameters = 3;
        parameter_modes.entry(3).or_insert(Mode::IMMEDIATE);
        let parameters = self.get_parameters(*index, num_parameters, parameter_modes);
        self.memory[parameters[2] as usize] = (parameters[0] == parameters[1]) as i64;
        *index += num_parameters + 1;
    }

    fn max_thruster_signal(memory: Vec<i64>) -> ThrusterSignal {
        let mut max_thruster_signal = ThrusterSignal {
            phase_settings: vec![],
            signal: i64::MIN,
        };

        for phase_settings in (0..=4).permutations(5) {
            let mut output = 0;
            for phase in phase_settings.iter() {
                let mut intcode = IntCode {
                    memory: memory.clone(),
                };
                let inputs = vec![*phase, output];
                output = intcode.process(inputs).unwrap();
            }

            if output > max_thruster_signal.signal {
                max_thruster_signal = ThrusterSignal {
                    phase_settings,
                    signal: output,
                };
            }
        }
        max_thruster_signal
    }

    fn max_thruster_signal_with_feedback(memory: Vec<i64>) -> ThrusterSignal {
        ThrusterSignal {
            phase_settings: vec![],
            signal: i64::MIN,
        }
    }

    fn process(&mut self, inputs: Vec<i64>) -> Option<i64> {
        let mut index: usize = 0;
        let mut input_index: usize = 0;
        let mut last_output: Option<i64> = None;
        loop {
            let raw_code = self.memory[index];
            let (opcode, parameter_modes) = IntCode::parse_opcode(raw_code);
            if opcode == 99 {
                return last_output;
            } else if opcode == 1 {
                self.opcode_1(&mut index, parameter_modes);
            } else if opcode == 2 {
                self.opcode_2(&mut index, parameter_modes);
            } else if opcode == 3 {
                self.opcode_3(&mut index, inputs[input_index]);
                input_index += 1;
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

fn part_1(contents: &str) -> i64 {
    let input: Vec<i64> = contents
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let max_thruster_signal = IntCode::max_thruster_signal(input);
    max_thruster_signal.signal
}

fn part_2(contents: &str) -> i64 {
    let input: Vec<i64> = contents
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let max_thruster_signal = IntCode::max_thruster_signal(input);
    max_thruster_signal.signal
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
    fn test_opcodes_1_and_2(#[case] memory: Vec<i64>, #[case] expected: Vec<i64>) {
        let mut intcode = IntCode { memory };
        intcode.process(vec![1]);
        assert_eq!(intcode.memory, expected);
    }

    #[rstest]
    #[case(99, (99, HashMap::new()))]
    #[case(101, (1, [(1, Mode::IMMEDIATE)].into_iter().collect()))]
    #[case(1003, (3, [(1, Mode::PARAMETER), (2, Mode::IMMEDIATE)].into_iter().collect()))]
    #[case(10104, (4, [(1, Mode::IMMEDIATE), (2, Mode::PARAMETER), (3, Mode::IMMEDIATE)].into_iter().collect()))]
    fn test_parse_opcodes(#[case] raw_code: i64, #[case] expected: (i64, HashMap<usize, Mode>)) {
        assert_eq!(IntCode::parse_opcode(raw_code), expected);
    }

    #[test]
    fn test_process_after_parsing_opcode() {
        let mut intcode = IntCode {
            memory: vec![1002, 4, 3, 4, 33],
        };
        intcode.process(vec![1]);
        assert_eq!(intcode.memory, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_negative_entry() {
        let mut intcode = IntCode {
            memory: vec![1101, 100, -1, 4, 0],
        };
        intcode.process(vec![1]);
        assert_eq!(intcode.memory, vec![1101, 100, -1, 4, 99]);
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
    fn test_opcodes_7_and_8(#[case] memory: Vec<i64>, #[case] input: i64, #[case] expected: i64) {
        let mut intcode = IntCode { memory };
        assert_eq!(intcode.process(vec![input]).unwrap(), expected);
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
    fn test_opcodes_5_and_6(#[case] memory: Vec<i64>, #[case] input: i64, #[case] expected: i64) {
        let mut intcode = IntCode { memory };
        assert_eq!(intcode.process(vec![input]).unwrap(), expected);
    }

    #[rstest]
    #[case(8, 1000)]
    #[case(6, 999)]
    #[case(9, 1001)]
    fn test_larger_example(#[case] input: i64, #[case] expected: i64) {
        let memory = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut intcode = IntCode {
            memory: memory.clone(),
        };
        assert_eq!(intcode.process(vec![input]).unwrap(), expected);
    }

    #[rstest]
    #[case(
        vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0],
        vec![4, 3, 2, 1, 0],
        43210
    )]
    #[case(
        vec![
            3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,
            99,0,0
        ],
        vec![0, 1, 2, 3, 4],
        54321
    )]
    #[case(
        vec![
            3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,
            33,31,31,1,32,31,31,4,31,99,0,0,0
        ],
        vec![1, 0, 4, 3, 2],
        65210
    )]
    fn test_max_thruster_signal(
        #[case] memory: Vec<i64>,
        #[case] phase_settings: Vec<i64>,
        #[case] signal: i64,
    ) {
        let max_thruster_signal = IntCode::max_thruster_signal(memory);
        assert_eq!(max_thruster_signal.phase_settings, phase_settings);
        assert_eq!(max_thruster_signal.signal, signal);
    }

    #[rstest]
    #[case(
        vec![
            3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26, 27,4,27,1001,28,-1,
            28,1005,28,6,99,0,0,5
        ],
        vec![9, 8, 7, 6, 5],
        139629729
    )]
    #[case(
        vec![
            3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
            -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
            53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10
        ],
        vec![9, 7, 8, 5, 6],
        18216
    )]
    fn test_max_thruster_signal_with_feedback(
        #[case] memory: Vec<i64>,
        #[case] phase_settings: Vec<i64>,
        #[case] signal: i64,
    ) {
        let max_thruster_signal = IntCode::max_thruster_signal_with_feedback(memory);
        assert_eq!(max_thruster_signal.phase_settings, phase_settings);
        assert_eq!(max_thruster_signal.signal, signal);
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
