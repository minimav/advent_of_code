use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::Instant;

fn part_1(contents: &str) -> u32 {
    let mut max_elf: u32 = 0;
    let mut current_elf: u32 = 0;
    let mut lines = contents.lines().peekable();
    while let Some(line) = lines.next() {
        if lines.peek().is_none() || line.is_empty() {
            if current_elf > max_elf {
                max_elf = current_elf
            }
            current_elf = 0
        } else {
            current_elf += line.parse::<u32>().unwrap();
        }
    }
    max_elf
}

fn part_2(contents: &str) -> u32 {
    // use Reverse to make min-heap so popping returns the min
    let mut max_elves = BinaryHeap::with_capacity(3);
    let mut current_elf: u32 = 0;
    let mut lines = contents.lines().peekable();
    while let Some(line) = lines.next() {
        if lines.peek().is_none() || line.is_empty() {
            if max_elves.len() < 3 {
                max_elves.push(Reverse(current_elf));
            } else if max_elves.iter().any(|x| current_elf > x.0) {
                max_elves.pop();
                max_elves.push(Reverse(current_elf));
            }
            current_elf = 0
        } else {
            current_elf += line.parse::<u32>().unwrap();
        }
    }
    max_elves.iter().map(|x| x.0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 24000);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 45000);
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
