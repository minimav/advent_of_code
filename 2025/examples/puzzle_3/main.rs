use std::time::Instant;

fn part_1(contents: &str) -> u32 {
    let mut position: i32 = 50;
    let mut zero_visits: u32 = 0;
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        let instruction = line.chars().next().unwrap();
        let value: i32 = line[1..].parse().unwrap();
        match instruction {
            'R' => {
                position = (position + value) % 100;
            }
            'L' => {
                position = (position - value) % 100;
            }
            _ => panic!("Unknown instruction"),
        }
        if position == 0 {
            zero_visits += 1;
        }
    }
    return zero_visits
}

fn part_2(contents: &str) -> u32 {
    let mut position: i32 = 50;
    let mut zero_visits: u32 = 0;
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        let instruction = line.chars().next().unwrap();
        let value: i32 = line[1..].parse().unwrap();
        match instruction {
            'R' => {
                let to_zero = (100 - position) % 100;
                if value >= to_zero {
                    zero_visits += ((value - to_zero) / 100) as u32;
                    if to_zero > 0 {
                        zero_visits += 1;
                    }
                }
                
                position = (position + value) % 100;
            }
            'L' => {
                let to_zero = position;
                if value >= to_zero {
                    zero_visits += ((value - to_zero) / 100) as u32;
                    if to_zero > 0 {
                        zero_visits += 1;
                    }
                }
                position = (position - value) % 100;
            }
            _ => panic!("Unknown instruction"),
        }
    }
    return zero_visits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 3);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 6);
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
