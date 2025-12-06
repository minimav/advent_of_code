use std::time::Instant;
use std::ops::IndexMut;

fn part_1(contents: &str) -> u32 {
    let mut answer: u32 = 0;
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        let max_index = line.len() - 1;
        let digits: Vec<_> = line.chars().collect();
        let mut current_index: usize = 1;
        let mut current_start = digits[0];
        let mut current_end = digits[1];
        while current_index <= max_index {
            let digit = digits[current_index];
            if (digit > current_start) && (current_index <= max_index - 1) {
                current_start = digit;
                current_end = digits[current_index + 1];
            } else if digit > current_end {
                current_end = digit;
            } 
            current_index += 1;
        }
        answer += current_start.to_digit(10).unwrap() * 10 + current_end.to_digit(10).unwrap();
    }
    return answer;
}

fn part_2(contents: &str) -> u64 {
    let voltage_size = 12;
    let mut answer: u64 = 0;
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        let max_index = line.len() - 1;
        let digits: Vec<_> = line.chars().collect();
        let mut current_index: usize = 1;
        
        // These are the indexes of the best digits so far
        let mut current_best: [usize; 12] = (0..voltage_size).collect::<Vec<_>>().try_into().expect("Wrong size"); 
        while current_index <= max_index {
            let digit = digits[current_index];
            for i in 0..voltage_size {
                if current_index <= current_best[i] {
                    break;
                }
                let value = digits[current_best[i]];
                if digit > value && (current_index <= max_index - voltage_size + 1 + i){
                    (current_index..current_index + voltage_size - i)
                    .enumerate()
                    .for_each(|(offset, c)| {
                        current_best[i + offset] = c;
                    });
                    break;
                }
            }
            current_index += 1;
        }

        let best = current_best.into_iter().enumerate().map(|(i, c)| {
            let power = voltage_size - 1 - i;
            let digit = digits[c].to_digit(10).unwrap() as u64;
            digit * 10u64.pow(power as u32)
        }).sum::<u64>();

        answer += best;
    }
    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 357);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 3121910778619);
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
