use std::fs;
use std::time::Instant;

fn part_1(contents: &str) -> u32 {
    let mut max_elf: u32 = 0;
    let mut current_elf: u32 = 0;
    for line in contents.lines() {
        if line.is_empty() {
            if current_elf > max_elf {
                max_elf = current_elf
            }
            current_elf = 0
        } else {
            current_elf += line.parse::<u32>().unwrap();
        }
    }
    if current_elf > max_elf {
        max_elf = current_elf
    }
    max_elf
}

fn part_2(contents: &str) -> u32 {
    let mut max_elves: Vec<u32> = Vec::new();
    let mut current_elf: u32 = 0;
    for line in contents.lines() {
        if line.is_empty() {
            if max_elves.len() < 3 {
                max_elves.push(current_elf);
                max_elves.sort();
            } else if max_elves.iter().any(|x| &current_elf > x) {
                max_elves.drain(0..1);
                max_elves.push(current_elf);
                max_elves.sort();
            }
            current_elf = 0
        } else {
            current_elf += line.parse::<u32>().unwrap();
        }
    }
    if max_elves.iter().any(|x| &current_elf > x) {
        max_elves.drain(0..1);
        max_elves.push(current_elf);
    }
    max_elves.iter().sum()
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
