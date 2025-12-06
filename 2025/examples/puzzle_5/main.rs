use std::time::Instant;

fn part_1(contents: &str) -> u32 {
    let lines = contents.lines();
    let mut answer = 0;
    let mut ranges: Vec<(u64, u64)> = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.contains("-") {
            let parts: Vec<&str> = line.split("-").collect();
            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            ranges.push((start, end));
        } else {
            let value: u64 = line.parse().unwrap();
            for range in &ranges {
                if value >= range.0 && value <= range.1 {
                    answer += 1;
                    break;
                }
            }
        }
    }
    return answer;
}

fn part_2(contents: &str) -> u64 {
    let lines = contents.lines();
    // Maintain ranges while keeping them non-overlapping
    let mut ranges: Vec<(u64, u64)> = vec![];
    for line in lines {
        if line.is_empty() {
            break;
        }

        let parts: Vec<&str> = line.split("-").collect();
        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();
        
        if ranges.is_empty() {
            ranges.push((start, end));
            continue;
        }
        
        // Compare to existing ranges and merge if necessary
        for i in 0..ranges.len() {
            let (r_start, r_end) = ranges[i];
            // Ends before current range
            if end + 1 < r_start {
                ranges.insert(i, (start, end));
                break;
            } else if start > r_end + 1 {
                // Ends after this range...
                if i == ranges.len() - 1 {
                    // Ends after the last range so add to the end
                    ranges.push((start, end));
                    break;
                } else {
                    continue;
                }
            } else {
                // There is some overlap, so do an initial merge
                let new_start = std::cmp::min(start, r_start);
                let new_end = std::cmp::max(end, r_end);
                ranges[i] = (new_start, new_end);
                // Now do additional merges if necessary
                let mut j = i + 1;
                while j < ranges.len() {
                    let (next_start, next_end) = ranges[j];
                    if new_end + 1 >= next_start {
                        ranges[i].1 = std::cmp::max(new_end, next_end);
                        ranges.remove(j);
                    } else {
                        break;
                    }
                }
                break;
            }
        }
    }
    // Guaranteed to be non-overlapping ranges so can just sum the sizes
    return ranges.into_iter().map(|(start, end)| end - start + 1).sum::<u64>();
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
        assert_eq!(part_2(include_str!("./example.txt")), 14);
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
