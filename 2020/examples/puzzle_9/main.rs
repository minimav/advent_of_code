use itertools;
use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

fn check_pairs(values: &VecDeque<u64>, number: u64) -> bool {
    for (a, b) in itertools::iproduct!(values, values) {
        if a + b == number {
            return true;
        }
    }
    false
}

fn part_1(contents: &String, preamble_length: usize) -> Option<u64> {
    let mut deque: VecDeque<u64> = VecDeque::with_capacity(preamble_length);
    for line in contents.lines() {
        let number = line.parse::<u64>().unwrap();
        if deque.len() == preamble_length {
            if !check_pairs(&deque, number) {
                return Some(number);
            }
            // with_capacity means at least this capacity in memory, we still
            // need to remove stuff once the queue is full
            deque.pop_back();
        }
        deque.push_front(number);
    }
    None
}

fn get_encryption_weakness(numbers: Vec<u64>, start_index: usize, end_index: usize) -> u64 {
    let mut min = u64::MAX;
    let mut max = 0;
    let mut current_index = start_index;
    while current_index <= end_index {
        let number = numbers[current_index];
        if number < min {
            min = number
        }
        if number > max {
            max = number
        }
        current_index += 1;
    }
    return min + max;
}

fn part_2(contents: &String, target: &u64) {
    let mut numbers: Vec<u64> = Vec::new();
    for line in contents.lines() {
        let number = line.parse::<u64>().unwrap();
        numbers.push(number)
    }
    for (index, _) in numbers.iter().enumerate() {
        let mut total: u64 = 0;
        let mut current_index = index;
        while total < *target {
            match numbers.get(current_index) {
                Some(n) => {
                    total += n;
                    if total == *target {
                        let encryption_weakness =
                            get_encryption_weakness(numbers, index, current_index);
                        println!("Answer for part 2 is: {}", encryption_weakness);
                        return;
                    } else if total > *target {
                        break;
                    }
                }
                _ => {
                    // reached end of the numbers, shouldn't happen
                    break;
                }
            }
            current_index += 1
        }
    }
}

fn main() {
    let start = Instant::now();
    let file_names: [(&str, usize); 2] = [
        ("examples/puzzle_9/example.txt", 5),
        ("examples/puzzle_9/input.txt", 25),
    ];
    for (file_name, preamble_length) in file_names.iter() {
        println!("FILE: {}", file_name);
        let contents = fs::read_to_string(file_name).expect("Could not read file");
        let part_1_answer = part_1(&contents, *preamble_length).unwrap();
        println!("Answer for part 1 is: {}", part_1_answer);
        part_2(&contents, &part_1_answer);
        let duration = start.elapsed();
        println!("Took {:?} to solve this puzzle", duration);
    }
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
