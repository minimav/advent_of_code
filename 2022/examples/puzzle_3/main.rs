#[macro_use]
extern crate lazy_static;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

lazy_static! {
    static ref CHAR_POINTS: HashMap<char, u64> = {
        let mut m = HashMap::new();
        let alphanumeric = "abcdefghijklmnopqrstuvwxyz";
        for (index, char) in alphanumeric.chars().enumerate() {
            m.insert(char, (index + 1) as u64);
            m.insert(char.to_ascii_uppercase(), (index + 27) as u64);
        }
        m
    };
}

fn part_1(contents: &str) -> u64 {
    let mut points: u64 = 0;
    for line in contents.lines() {
        let mut chars: HashSet<char> = HashSet::new();
        for (char_index, char) in line.chars().enumerate() {
            if char_index < line.len() / 2 {
                chars.insert(char);
            } else if chars.contains(&char) {
                points += CHAR_POINTS.get(&char).unwrap();
                break;
            }
        }
    }
    points
}

fn part_2(contents: &str) -> u64 {
    let mut points: u64 = 0;
    for group in &contents.lines().chunks(3) {
        let mut num_lines_in: HashMap<char, u8> = HashMap::new();
        for line in group {
            let mut chars: HashSet<char> = HashSet::new();
            for char in line.chars() {
                if !chars.contains(&char) {
                    num_lines_in
                        .entry(char)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                    chars.insert(char);
                }
            }
        }

        for (char, value) in num_lines_in {
            if value == 3 {
                points += CHAR_POINTS.get(&char).unwrap();
                break;
            }
        }
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 157);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 70);
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
