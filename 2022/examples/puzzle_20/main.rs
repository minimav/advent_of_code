use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct FileNumber {
    value: i128,
    current_index: usize,
}

fn parse_input(contents: &str) -> HashMap<usize, FileNumber> {
    let mut numbers: HashMap<usize, FileNumber> = HashMap::new();
    for (index, line) in contents.lines().enumerate() {
        numbers.insert(
            index,
            FileNumber {
                value: line.trim().parse::<i128>().unwrap(),
                current_index: index,
            },
        );
    }
    numbers
}

fn get_zero_index(numbers: &HashMap<usize, FileNumber>) -> usize {
    for (index, number) in numbers.iter() {
        if number.value == 0 {
            return number.current_index;
        }
    }
    panic!("Could not find 0!")
}

fn get_answer(zero_index: usize, numbers: &HashMap<usize, FileNumber>) -> i128 {
    let mut answer = 0;
    for offset in [1000, 2000, 3000] {
        let index = (zero_index + offset) % numbers.len();
        for (_, number) in numbers.iter() {
            if number.current_index == index {
                answer += number.value;
                break;
            }
        }
    }
    answer
}

fn create_vector(numbers: &HashMap<usize, FileNumber>) -> Vec<i128> {
    let mut vec: Vec<i128> = vec![0; numbers.len()];
    for (_, number) in numbers.iter() {
        vec[number.current_index] = number.value;
    }
    vec
}

fn process(
    input_numbers: HashMap<usize, FileNumber>,
    num_iterations: u8,
) -> HashMap<usize, FileNumber> {
    let length = input_numbers.len();
    let mut numbers = input_numbers.clone();
    for _ in 0..num_iterations {
        for start_index in 0..length {
            let mut new_numbers: HashMap<usize, FileNumber> = HashMap::new();
            let number_to_shift = numbers.get(&start_index).unwrap();
            if number_to_shift.value == 0 || number_to_shift.value % (length as i128 - 1) == 0 {
                continue;
            }

            let number_to_shift_new_index =
                (number_to_shift.current_index as i128 + number_to_shift.value)
                    .rem_euclid((length - 1) as i128) as usize;

            for (other_start_index, number) in numbers.iter() {
                let mut new_current_index = number.current_index.clone();
                if start_index == *other_start_index {
                    new_current_index = number_to_shift_new_index;
                } else {
                    if number_to_shift.current_index < number.current_index {
                        new_current_index -= 1;
                    };

                    if number_to_shift_new_index <= new_current_index {
                        new_current_index += 1;
                    }
                }
                new_current_index = new_current_index % length;

                new_numbers.insert(
                    *other_start_index,
                    FileNumber {
                        value: number.value,
                        current_index: new_current_index,
                    },
                );
            }
            numbers = new_numbers;
        }
    }
    numbers
}

fn part_1(contents: &str) -> i128 {
    let input_numbers = parse_input(contents);
    let numbers = process(input_numbers, 1);
    let zero_index = get_zero_index(&numbers);
    get_answer(zero_index, &numbers)
}

fn part_2(contents: &str) -> i128 {
    let mut input_numbers = parse_input(contents);
    for (_, number) in input_numbers.iter_mut() {
        number.value *= 811589153;
    }
    let numbers = process(input_numbers, 10);
    let zero_index = get_zero_index(&numbers);
    get_answer(zero_index, &numbers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("0\n1\n2\n3\n4", 0)]
    #[case("0\n1\n2\n3", 0)]
    #[case("0\n1\n2", 3)]
    #[case("0\n1\n2\n3\n4\n5", 6)]
    fn test_get_answer(#[case] input: &str, #[case] expected: i128) {
        let numbers = parse_input(input);
        let answer = get_answer(0, &numbers);
        assert_eq!(answer, expected);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 3);
    }

    #[rstest]
    #[case("-1\n-2\n-3\n0", vec![-3, -2, -1, 0])]
    #[case("1\n2\n-3\n0", vec![1, -3, 2, 0])]
    #[case("-1\n0\n0\n0", vec![0, 0, -1, 0])]
    #[case("-2\n0\n0\n0", vec![0, -2, 0, 0])]
    #[case("-3\n0\n0\n0", vec![-3, 0, 0, 0])]
    #[case("3\n0\n0\n0", vec![3, 0, 0, 0])]
    #[case("4\n0\n0\n0", vec![0, 4, 0, 0])]
    #[case("-4\n0\n0\n0", vec![0, 0, -4, 0])]
    #[case("-10\n0\n-3\n1", vec![0, 1, -3, -10])]
    fn test_part_1_vector(#[case] input: &str, #[case] expected: Vec<i128>) {
        let input_numbers = parse_input(input);
        let numbers = process(input_numbers, 1);
        assert_eq!(create_vector(&numbers), expected);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 1623178306);
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
