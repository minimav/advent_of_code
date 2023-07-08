use std::collections::{HashMap, VecDeque};
use std::time::Instant;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum ParameterMode {
    POSITION,
    IMMEDIATE,
}

#[derive(Debug, PartialEq, Eq)]
enum Status {
    CONTINUE,
    TERMINATE,
}

#[derive(Debug)]
struct ThrusterSignal {
    phase_settings: Vec<i64>,
    signal: i64,
}

#[derive(Debug)]
struct Signal {
    output: Option<i64>,
    status: Status,
}

struct IntCode {
    memory: Vec<i64>,
    index: usize,
}

impl IntCode {
    fn from_memory(memory: Vec<i64>) -> Self {
        IntCode { memory, index: 0 }
    }
    fn parse_opcode(raw_code: i64) -> (i64, HashMap<usize, ParameterMode>) {
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

    fn read_memory(&self, mode: &ParameterMode, index: Option<usize>) -> i64 {
        let memory_index = match index {
            Some(i) => i,
            None => self.index,
        };
        match mode {
            ParameterMode::IMMEDIATE => self.memory[memory_index],
            ParameterMode::POSITION => {
                let value_index = self.memory[memory_index];
                self.memory[value_index as usize]
            }
        }
    }

    fn get_parameters(
        &self,
        num_parameters: usize,
        parameter_modes: HashMap<usize, ParameterMode>,
    ) -> Vec<i64> {
        (1..=num_parameters)
            .map(|x| {
                self.read_memory(
                    parameter_modes.get(&x).unwrap_or(&ParameterMode::POSITION),
                    Some(self.index + x),
                )
            })
            .collect()
    }

    fn opcode_1(&mut self, mut parameter_modes: HashMap<usize, ParameterMode>) {
        let num_parameters = 3;
        parameter_modes.entry(3).or_insert(ParameterMode::IMMEDIATE);
        let parameters = self.get_parameters(num_parameters, parameter_modes);
        self.memory[parameters[2] as usize] = parameters[0] + parameters[1];
        self.index += num_parameters + 1;
    }

    fn opcode_2(&mut self, mut parameter_modes: HashMap<usize, ParameterMode>) {
        let num_parameters = 3;
        parameter_modes.entry(3).or_insert(ParameterMode::IMMEDIATE);
        let parameters = self.get_parameters(num_parameters, parameter_modes);
        self.memory[parameters[2] as usize] = parameters[0] * parameters[1];
        self.index += num_parameters + 1;
    }

    fn opcode_3(&mut self, input: i64) {
        let change_index = self.memory[self.index + 1];
        self.memory[change_index as usize] = input;
        self.index += 2
    }

    fn opcode_4(&mut self, parameter_modes: HashMap<usize, ParameterMode>) -> i64 {
        let output = self.read_memory(
            parameter_modes.get(&1).unwrap_or(&ParameterMode::POSITION),
            Some(self.index + 1),
        );
        self.index += 2;
        output
    }

    fn opcode_5(&mut self, parameter_modes: HashMap<usize, ParameterMode>) {
        let num_parameters = 2;
        let parameters = self.get_parameters(num_parameters, parameter_modes);
        if parameters[0] > 0 {
            self.index = parameters[1] as usize;
        } else {
            self.index += num_parameters + 1;
        }
    }

    fn opcode_6(&mut self, parameter_modes: HashMap<usize, ParameterMode>) {
        let num_parameters = 2;
        let parameters = self.get_parameters(num_parameters, parameter_modes);
        if parameters[0] == 0 {
            self.index = parameters[1] as usize;
        } else {
            self.index += num_parameters + 1;
        }
    }

    fn opcode_7(&mut self, mut parameter_modes: HashMap<usize, ParameterMode>) {
        let num_parameters = 3;
        parameter_modes.entry(3).or_insert(ParameterMode::IMMEDIATE);
        let parameters = self.get_parameters(num_parameters, parameter_modes);
        self.memory[parameters[2] as usize] = (parameters[0] < parameters[1]) as i64;
        self.index += num_parameters + 1;
    }

    fn opcode_8(&mut self, mut parameter_modes: HashMap<usize, ParameterMode>) {
        let num_parameters = 3;
        parameter_modes.entry(3).or_insert(ParameterMode::IMMEDIATE);
        let parameters = self.get_parameters(num_parameters, parameter_modes);
        self.memory[parameters[2] as usize] = (parameters[0] == parameters[1]) as i64;
        self.index += num_parameters + 1;
    }

    fn max_thruster_signal(memory: Vec<i64>) -> ThrusterSignal {
        let mut max_thruster_signal = ThrusterSignal {
            phase_settings: vec![],
            signal: i64::MIN,
        };

        for phase_settings in (0..=4).permutations(5) {
            let mut output = 0;
            let mut inputs: VecDeque<i64> = VecDeque::new();
            for phase in phase_settings.iter() {
                let mut intcode = IntCode {
                    memory: memory.clone(),
                    index: 0,
                };
                inputs.push_back(*phase);
                inputs.push_back(output);
                output = intcode.process(&mut inputs);
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
        let mut max_thruster_signal = ThrusterSignal {
            phase_settings: vec![],
            signal: i64::MIN,
        };
        let num_amplifiers: i64 = 5;
        for phase_settings in (5..5 + num_amplifiers).permutations(num_amplifiers as usize) {
            let mut amplifiers: Vec<IntCode> = (0..num_amplifiers)
                .map(|_| IntCode::from_memory(memory.clone()))
                .collect();

            let mut amplifier_index: usize = 0;
            let mut inputs: Vec<VecDeque<i64>> = vec![VecDeque::new(); num_amplifiers as usize];

            // prime the inputs
            for (index, phase) in phase_settings.iter().enumerate() {
                inputs[index].push_back(*phase);
            }
            inputs[0].push_back(0);

            let mut output_signal = 0;
            loop {
                let signal = amplifiers[amplifier_index as usize]
                    .process_to_output_or_terminate(&mut inputs[amplifier_index as usize]);

                // update the output if there was some
                amplifier_index = (amplifier_index + 1) % (num_amplifiers as usize);
                match signal.output {
                    Some(output) => {
                        output_signal = output;
                        inputs[amplifier_index as usize].push_back(output);
                    }
                    _ => {}
                }

                // exit gracefully if the final amplifier had a terminate status
                // we've already changed the amplifier index, so check takes
                // that into account
                if amplifier_index == 0 as usize {
                    match signal.status {
                        Status::TERMINATE => break,
                        _ => {}
                    }
                }
            }

            // override maximum signal if these phase settings are better
            if output_signal > max_thruster_signal.signal {
                max_thruster_signal = ThrusterSignal {
                    phase_settings,
                    signal: output_signal,
                };
            }
        }
        max_thruster_signal
    }

    fn process_op(&mut self, inputs: &mut VecDeque<i64>) -> Signal {
        let raw_code = self.memory[self.index];
        let (opcode, parameter_modes) = IntCode::parse_opcode(raw_code);
        let mut output = None;
        let mut status = Status::CONTINUE;
        if opcode == 99 {
            status = Status::TERMINATE;
        } else if opcode == 1 {
            self.opcode_1(parameter_modes);
        } else if opcode == 2 {
            self.opcode_2(parameter_modes);
        } else if opcode == 3 {
            let input = inputs.pop_front().unwrap();
            self.opcode_3(input);
        } else if opcode == 4 {
            output = Some(self.opcode_4(parameter_modes));
        } else if opcode == 5 {
            self.opcode_5(parameter_modes);
        } else if opcode == 6 {
            self.opcode_6(parameter_modes);
        } else if opcode == 7 {
            self.opcode_7(parameter_modes);
        } else if opcode == 8 {
            self.opcode_8(parameter_modes);
        }
        Signal { output, status }
    }

    fn process_to_output_or_terminate(&mut self, inputs: &mut VecDeque<i64>) -> Signal {
        loop {
            let signal = self.process_op(inputs);
            match signal {
                Signal {
                    status: Status::TERMINATE,
                    ..
                }
                | Signal {
                    output: Some(_), ..
                } => return signal,
                _ => {}
            }
        }
    }

    fn process(&mut self, inputs: &mut VecDeque<i64>) -> i64 {
        let mut output = 0;
        loop {
            let signal = self.process_op(inputs);
            match signal.status {
                Status::TERMINATE => return output,
                Status::CONTINUE => match signal.output {
                    Some(new_output) => {
                        if output > 0 {
                            panic!("Diagnostic test failed")
                        }
                        output = new_output;
                    }
                    None => {}
                },
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

    let max_thruster_signal = IntCode::max_thruster_signal_with_feedback(input);
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
        let mut intcode = IntCode { memory, index: 0 };
        intcode.process(&mut VecDeque::from(vec![1]));
        assert_eq!(intcode.memory, expected);
    }

    #[rstest]
    #[case(99, (99, HashMap::new()))]
    #[case(101, (1, [(1, ParameterMode::IMMEDIATE)].into_iter().collect()))]
    #[case(1003, (3, [(1, ParameterMode::POSITION), (2, ParameterMode::IMMEDIATE)].into_iter().collect()))]
    #[case(10104, (4, [(1, ParameterMode::IMMEDIATE), (2, ParameterMode::POSITION), (3, ParameterMode::IMMEDIATE)].into_iter().collect()))]
    fn test_parse_opcodes(
        #[case] raw_code: i64,
        #[case] expected: (i64, HashMap<usize, ParameterMode>),
    ) {
        assert_eq!(IntCode::parse_opcode(raw_code), expected);
    }

    #[test]
    fn test_process_after_parsing_opcode() {
        let mut intcode = IntCode {
            memory: vec![1002, 4, 3, 4, 33],
            index: 0,
        };
        intcode.process(&mut VecDeque::from(vec![1]));
        assert_eq!(intcode.memory, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_negative_entry() {
        let mut intcode = IntCode {
            memory: vec![1101, 100, -1, 4, 0],
            index: 0,
        };
        intcode.process(&mut VecDeque::from(vec![1]));
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
        let mut intcode = IntCode { memory, index: 0 };
        let mut inputs = VecDeque::from(vec![input]);
        assert_eq!(intcode.process(&mut inputs), expected);
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
        let mut intcode = IntCode { memory, index: 0 };
        let mut inputs = VecDeque::from(vec![input]);
        assert_eq!(intcode.process(&mut inputs), expected);
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
            index: 0,
        };
        let mut inputs = VecDeque::from(vec![input]);
        assert_eq!(intcode.process(&mut inputs), expected);
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
            3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,
            1005,28,6,99,0,0,5
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
