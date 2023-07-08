use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
enum ParameterMode {
    POSITION,
    IMMEDIATE,
    RELATIVE,
}

struct IntCode {
    memory: HashMap<i64, i64>,
}

impl IntCode {
    fn parse_opcode(raw_code: i64) -> (i64, HashMap<i64, ParameterMode>) {
        let mut parameter_modes: HashMap<i64, ParameterMode> = HashMap::new();
        let opcode = raw_code % 100;

        let mut mode_digits = (raw_code - opcode) / 100;
        let mut offset: i64 = 1;
        while mode_digits > 0 {
            let digit = mode_digits % 10;
            mode_digits = (mode_digits - digit) / 10;
            let mode = match digit {
                0 => ParameterMode::POSITION,
                1 => ParameterMode::IMMEDIATE,
                2 => ParameterMode::RELATIVE,
                _ => panic!("ParameterMode digit should only be 0, 1 or 2"),
            };
            parameter_modes.insert(offset, mode);
            offset += 1
        }
        (opcode, parameter_modes)
    }

    fn get_value_at_index(&self, index: i64, relative_base: i64, mode: &ParameterMode) -> i64 {
        match mode {
            ParameterMode::IMMEDIATE => *self.memory.get(&index).unwrap_or(&0),
            ParameterMode::POSITION => {
                let value_index = self.memory.get(&index).unwrap_or(&0);
                *self.memory.get(value_index).unwrap_or(&0)
            }
            ParameterMode::RELATIVE => {
                let value_index = self.memory.get(&index).unwrap_or(&0);
                *self
                    .memory
                    .get(&(value_index + relative_base))
                    .unwrap_or(&0)
            }
        }
    }

    fn get_parameters(
        &self,
        index: i64,
        relative_base: i64,
        num_parameters: usize,
        parameter_modes: HashMap<i64, ParameterMode>,
    ) -> Vec<i64> {
        (1..=num_parameters)
            .map(|x| {
                self.get_value_at_index(
                    index + (x as i64),
                    relative_base,
                    parameter_modes
                        .get(&(x as i64))
                        .unwrap_or(&ParameterMode::POSITION),
                )
            })
            .collect()
    }

    fn opcode_1(
        &mut self,
        index: &mut i64,
        relative_base: &mut i64,
        mut parameter_modes: HashMap<i64, ParameterMode>,
    ) {
        let num_parameters = 3;
        parameter_modes.entry(3).or_insert(ParameterMode::IMMEDIATE);
        let parameters =
            self.get_parameters(*index, *relative_base, num_parameters, parameter_modes);
        self.memory
            .insert(parameters[2], parameters[0] + parameters[1]);
        *index += (num_parameters + 1) as i64;
    }

    fn opcode_2(
        &mut self,
        index: &mut i64,
        relative_base: &mut i64,
        mut parameter_modes: HashMap<i64, ParameterMode>,
    ) {
        let num_parameters = 3;
        parameter_modes.entry(3).or_insert(ParameterMode::IMMEDIATE);
        let parameters =
            self.get_parameters(*index, *relative_base, num_parameters, parameter_modes);
        self.memory
            .insert(parameters[2], parameters[0] * parameters[1]);
        *index += (num_parameters + 1) as i64;
    }

    fn opcode_3(
        &mut self,
        index: &mut i64,
        relative_base: &mut i64,
        input: i64,
        parameter_modes: HashMap<i64, ParameterMode>,
    ) {
        let raw_index = self.memory.get(&(*index + 1)).unwrap_or(&0);
        let relative_index = raw_index + relative_base.clone();
        let change_index = match parameter_modes.get(&1).unwrap_or(&ParameterMode::POSITION) {
            ParameterMode::POSITION => raw_index,
            ParameterMode::RELATIVE => &relative_index,
            _ => panic!("ParameterMode should only be POSITION or RELATIVE"),
        };
        self.memory.insert(*change_index, input);
        *index += 2
    }

    fn opcode_4(
        &mut self,
        index: &mut i64,
        relative_base: &mut i64,
        parameter_modes: HashMap<i64, ParameterMode>,
    ) -> i64 {
        let output = self.get_value_at_index(
            *index + 1,
            *relative_base,
            parameter_modes.get(&1).unwrap_or(&ParameterMode::POSITION),
        );
        *index += 2;
        output
    }

    fn opcode_5(
        &mut self,
        index: &mut i64,
        relative_base: &mut i64,
        parameter_modes: HashMap<i64, ParameterMode>,
    ) {
        let num_parameters = 2;
        let parameters =
            self.get_parameters(*index, *relative_base, num_parameters, parameter_modes);
        if parameters[0] > 0 {
            *index = parameters[1];
        } else {
            *index += (num_parameters + 1) as i64;
        }
    }

    fn opcode_6(
        &mut self,
        index: &mut i64,
        relative_base: &mut i64,
        parameter_modes: HashMap<i64, ParameterMode>,
    ) {
        let num_parameters = 2;
        let parameters =
            self.get_parameters(*index, *relative_base, num_parameters, parameter_modes);
        if parameters[0] == 0 {
            *index = parameters[1];
        } else {
            *index += (num_parameters + 1) as i64;
        }
    }

    fn opcode_7(
        &mut self,
        index: &mut i64,
        relative_base: &mut i64,
        mut parameter_modes: HashMap<i64, ParameterMode>,
    ) {
        let num_parameters = 3;
        parameter_modes.entry(3).or_insert(ParameterMode::IMMEDIATE);
        let parameters =
            self.get_parameters(*index, *relative_base, num_parameters, parameter_modes);
        self.memory
            .insert(parameters[2], (parameters[0] < parameters[1]) as i64);
        *index += (num_parameters + 1) as i64;
    }

    fn opcode_8(
        &mut self,
        index: &mut i64,
        relative_base: &mut i64,
        mut parameter_modes: HashMap<i64, ParameterMode>,
    ) {
        let num_parameters = 3;
        parameter_modes.entry(3).or_insert(ParameterMode::IMMEDIATE);
        let parameters =
            self.get_parameters(*index, *relative_base, num_parameters, parameter_modes);
        self.memory
            .insert(parameters[2], (parameters[0] == parameters[1]) as i64);
        *index += (num_parameters + 1) as i64;
    }

    fn opcode_9(
        &mut self,
        index: &mut i64,
        relative_base: &mut i64,
        parameter_modes: HashMap<i64, ParameterMode>,
    ) {
        let output = self.get_value_at_index(
            *index + 1,
            *relative_base,
            parameter_modes.get(&1).unwrap_or(&ParameterMode::POSITION),
        );
        *relative_base += output;
        *index += 2;
    }

    fn process(&mut self, input: i64, relative_base: &mut i64) -> Option<i64> {
        let mut index: i64 = 0;
        let mut last_output: Option<i64> = None;
        loop {
            let raw_code = self.memory.get(&index).unwrap_or(&0);
            let (opcode, parameter_modes) = IntCode::parse_opcode(*raw_code);
            //println!("index={index} raw_code={raw_code} relative_base={relative_base} -> opcode={opcode} parameter_modes={parameter_modes:?}");
            //println!("{:?}", self.memory);
            if opcode == 99 {
                return last_output;
            } else if opcode == 1 {
                self.opcode_1(&mut index, relative_base, parameter_modes);
            } else if opcode == 2 {
                self.opcode_2(&mut index, relative_base, parameter_modes);
            } else if opcode == 3 {
                self.opcode_3(&mut index, relative_base, input, parameter_modes);
            } else if opcode == 4 {
                let next_output = self.opcode_4(&mut index, relative_base, parameter_modes);
                if last_output.is_some() && last_output.unwrap() > 0 {
                    panic!("Diagnostic test failed, output={}", last_output.unwrap())
                } else {
                    last_output = Some(next_output)
                }
            } else if opcode == 5 {
                self.opcode_5(&mut index, relative_base, parameter_modes);
            } else if opcode == 6 {
                self.opcode_6(&mut index, relative_base, parameter_modes);
            } else if opcode == 7 {
                self.opcode_7(&mut index, relative_base, parameter_modes);
            } else if opcode == 8 {
                self.opcode_8(&mut index, relative_base, parameter_modes);
            } else if opcode == 9 {
                self.opcode_9(&mut index, relative_base, parameter_modes);
            }
        }
    }
}

fn part_1(contents: &str) -> i64 {
    let memory_values: Vec<i64> = contents
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let memory = create_memory(memory_values);
    let mut intcode = IntCode { memory };
    intcode.process(1, &mut 0).unwrap()
}

fn part_2(contents: &str) -> u64 {
    0
}

fn create_memory(memory_values: Vec<i64>) -> HashMap<i64, i64> {
    memory_values
        .into_iter()
        .enumerate()
        .map(|(index, value)| (index as i64, value))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    // #[test]
    // fn test_part_1_quine() {
    //     let memory_values = vec![
    //         109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    //     ];
    //     let memory = create_memory(memory_values);
    //     let mut intcode = IntCode {
    //         memory: memory.clone(),
    //     };
    //     intcode.process(0, &mut 0);
    //     assert_eq!(intcode.memory, memory);
    // }

    #[rstest]
    #[case(vec![109, -1, 4, 1, 99], -1)]
    #[case(vec![109, -1, 104, 1, 99], 1)]
    #[case(vec![109, -1, 204, 1, 99], 109)]
    #[case(vec![109, 1, 9, 2, 204, -6, 99], 204)]
    #[case(vec![109, 1, 109, 9, 204, -6, 99], 204)]
    #[case(vec![109, 1, 209, -1, 204, -106, 99], 204)]
    #[case(vec![109, 1, 3, 3, 204, 2, 99], 5)]
    #[case(vec![109, 1, 203, 2, 204, 2, 99], 5)] /////////
    #[case(vec![104, 1125899906842624, 99], 1125899906842624)]
    fn test_part_1_specific_case_outputs(#[case] memory_values: Vec<i64>, #[case] expected: i64) {
        let memory = create_memory(memory_values);
        let mut intcode = IntCode { memory };
        assert_eq!(intcode.process(5, &mut 0).unwrap(), expected);
    }

    #[test]
    fn test_part_1_output_length() {
        let memory_values = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let memory = create_memory(memory_values);
        let mut intcode = IntCode { memory };
        let output = intcode.process(0, &mut 0);
        assert_eq!(output.unwrap().to_string().len(), 16);
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
