use std::time::Instant;

fn intcode(input: &mut Vec<usize>) -> &mut Vec<usize> {
    let mut index: usize = 0;
    loop {
        let opcode = input[index];
        if opcode == 99 {
            break;
        };

        let value_1_index = input[index + 1];
        let value_2_index = input[index + 2];
        let value_1 = input[value_1_index];
        let value_2 = input[value_2_index];

        let new_value = if opcode == 1 {
            value_1 + value_2
        } else if opcode == 2 {
            value_1 * value_2
        } else {
            panic!("Only 1, 2 and 99 are expected as opcodes");
        };
        let change_index = input[index + 3];
        input[change_index] = new_value;

        index += 4;
    }
    input
}

fn part_1(contents: &str) -> usize {
    let mut input: Vec<usize> = contents
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    // apply changes from problem description
    input[1] = 12;
    input[2] = 2;
    let output = intcode(&mut input);
    output[0]
}

fn part_2(contents: &str, target: usize) -> usize {
    let raw_input: Vec<usize> = contents
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    for noun in 0..99 {
        for verb in 0..99 {
            let mut input = raw_input.clone();
            input[1] = noun as usize;
            input[2] = verb as usize;
            let output = intcode(&mut input);
            if output[0] == target {
                return 100 * noun + verb;
            }
        }
    }
    panic!("Part 2 should have a solution with noun and verb between 0 and 99")
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
    fn test_(#[case] input: Vec<usize>, #[case] expected: Vec<usize>) {
        assert_eq!(intcode(&mut input.to_owned()), &expected);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./input.txt"), 5110675), 1202);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let part_1_answer = part_1(contents);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer = part_2(contents, 19690720);
    println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
