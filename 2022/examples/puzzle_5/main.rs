use std::collections::VecDeque;
use std::time::Instant;

#[derive(Debug)]
struct Instruction {
    num_to_move: u8,
    from_stack_index: usize,
    to_stack_index: usize,
}

fn parse_stacks(contents: &str, num_stacks: usize) -> Vec<VecDeque<char>> {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..num_stacks {
        let stack: VecDeque<char> = VecDeque::new();
        stacks.push(stack);
    }

    for line in contents.lines().take_while(|x| x.contains("[")) {
        for stack_index in 0..num_stacks {
            match line.chars().nth(1 + stack_index * 4) {
                Some(' ') => {}
                Some(c) => stacks[stack_index].push_back(c),
                None => {}
            }
        }
    }
    stacks
}

fn parse_instructions(contents: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in contents.lines().skip_while(|x| !x.starts_with("move")) {
        let cleaned_line = line
            .replace("move ", "")
            .replace("from ", "")
            .replace("to ", "");

        let raw_numbers: Vec<&str> = cleaned_line.split_whitespace().collect();

        let num_to_move = raw_numbers[0].parse::<u8>().unwrap();
        let from_stack_index = raw_numbers[1].parse::<usize>().unwrap() - 1;
        let to_stack_index = raw_numbers[2].parse::<usize>().unwrap() - 1;

        instructions.push(Instruction {
            num_to_move,
            from_stack_index,
            to_stack_index,
        })
    }
    instructions
}

fn get_num_stacks(instructions: &Vec<Instruction>) -> usize {
    let mut max_stack_index: usize = 0;
    for instruction in instructions.iter() {
        if instruction.from_stack_index > max_stack_index {
            max_stack_index = instruction.from_stack_index;
        }
        if instruction.to_stack_index > max_stack_index {
            max_stack_index = instruction.to_stack_index;
        }
    }
    max_stack_index + 1
}

fn get_stack_tops(stacks: Vec<VecDeque<char>>) -> String {
    let mut top = String::from("");
    for stack in stacks.iter() {
        match stack.get(0) {
            Some(c) => top.push(*c),
            None => panic!("Stack was empty after all moves"),
        }
    }
    top
}

fn part_1(contents: &str) -> String {
    let instructions: Vec<Instruction> = parse_instructions(contents);
    let num_stacks: usize = get_num_stacks(&instructions);
    let mut stacks: Vec<VecDeque<char>> = parse_stacks(contents, num_stacks);

    for instruction in instructions.iter() {
        for _ in 0..instruction.num_to_move {
            match stacks[instruction.from_stack_index].pop_front() {
                Some(c) => stacks[instruction.to_stack_index].push_front(c),
                None => panic!("Could not pop from stack"),
            }
        }
    }
    get_stack_tops(stacks)
}

fn part_2(contents: &str) -> String {
    let instructions: Vec<Instruction> = parse_instructions(contents);
    let num_stacks: usize = get_num_stacks(&instructions);
    let mut stacks: Vec<VecDeque<char>> = parse_stacks(contents, num_stacks);

    for instruction in instructions.iter() {
        // create temp stack so we respect the new order
        let mut temp_stack: VecDeque<char> = VecDeque::new();
        for _ in 0..instruction.num_to_move {
            match stacks[instruction.from_stack_index].pop_front() {
                Some(c) => temp_stack.push_front(c),
                None => panic!("Could not pop from stack"),
            }
        }
        for _ in 0..instruction.num_to_move {
            match temp_stack.pop_front() {
                Some(c) => stacks[instruction.to_stack_index].push_front(c),
                None => panic!("Could not pop from temp stack"),
            }
        }
    }

    get_stack_tops(stacks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), String::from("CMZ"));
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), String::from("MCD"));
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
