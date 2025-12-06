use std::time::Instant;

fn part_1(contents: &str) -> u64 {
    let mut answer: u64 = 0;
    let line = contents.lines().next().unwrap();
    line.split(",").for_each(|id| {
        let nums = id.split("-").collect::<Vec<&str>>();
        let start: u64 = nums[0].parse().unwrap();
        let start_str = nums[0];
        let end: u64 = nums[1].parse().unwrap();
        let end_str = nums[1];

        if (start_str.len() % 2 == 1) && (start_str.len() == end_str.len()) {
            // Both same odd length so can't have a mirror
            return;
        }

        let mirror_start: u64 = if start_str.len() % 2 == 0 {
            start_str[..(start_str.len() / 2)].parse().unwrap()
        } else {
            10u64.pow((start_str.len() / 2) as u32)
        };

        let mirror_end: u64 = if end_str.len() % 2 == 0 {
            end_str[..(end_str.len() / 2)].parse().unwrap()
        } else {
            10u64.pow((end_str.len() / 2) as u32) - 1
        };

        for mirror in mirror_start..=mirror_end {
            let half = mirror.to_string();
            let full = half.clone() + &half.clone();
            let num = full.parse::<u64>().unwrap();
            if (num >= start) && (num <= end) {
                answer += num;
            }
        };
    });

    return answer
}

fn part_2(contents: &str) -> u64 {
    let mut answer: u64 = 0;
    let line = contents.lines().next().unwrap();
    line.split(",").for_each(|id| {
        let nums = id.split("-").collect::<Vec<&str>>();
        let start: u64 = nums[0].parse().unwrap();
        let start_str = nums[0];
        let end: u64 = nums[1].parse().unwrap();
        let end_str = nums[1];

        let max_mirror_len = end_str.len() / 2;
        let mut mirrors = std::collections::HashSet::new();
        
        for digit_len in 1..=max_mirror_len {
            // To cover the 95-115 case we need to deal with 95-99 and 100-115
            // separately since both 99 and 111 are valid mirrors with a single
            // digit being repeated.
            let num_start_repeats = (start_str.len() / digit_len) as usize;
            let num_end_repeats = (end_str.len() / digit_len) as usize;
            if (start_str.len() % digit_len == 0) && (num_start_repeats > 1) {
                let mirror_start: u64 = start_str[..digit_len].parse().unwrap();
                let mirror_end: u64 = if start_str.len() == end_str.len() {
                    // 45-88 case - can only go up to the first digits of end
                    end_str[..digit_len].parse().unwrap()
                } else {
                    // 45-103 case - can go up to max for this digit length
                    10u64.pow(digit_len as u32) - 1
                };

                for mirror in mirror_start..=mirror_end {
                    let fragment = mirror.to_string();
                    let full = fragment.repeat(num_start_repeats);
                    let num = full.parse::<u64>().unwrap();
                    if (num >= start) && (num <= end) {
                        mirrors.insert(num);
                    }
                };
            }
            if (end_str.len() % digit_len == 0) && (num_end_repeats > 1) {
                let mirror_start: u64 = if start_str.len() == end_str.len() {
                    start_str[..digit_len].parse().unwrap()
                } else {
                    10u64.pow(digit_len as u32 - 1)    
                };
                let mirror_end: u64 = end_str[..digit_len].parse().unwrap();

                for mirror in mirror_start..=mirror_end {
                    let fragment = mirror.to_string();
                    let full = fragment.repeat(num_end_repeats);
                    let num = full.parse::<u64>().unwrap();
                    if (num >= start) && (num <= end) {
                        mirrors.insert(num);
                    }
                };
            }
        }

        answer += mirrors.iter().sum::<u64>();
    });

    return answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 1227775554);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 4174379265);
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
