use std::time::Instant;

#[derive(Debug)]
struct Range {
    low: u8,
    high: u8,
}

impl Range {
    fn contained_in(&self, other: &Range) -> bool {
        return other.low <= self.low && self.high <= other.high;
    }

    fn overlaps(&self, other: &Range) -> bool {
        return self.low <= other.high && self.high >= other.low
            || other.low <= self.high && other.high >= self.low;
    }
}

impl From<&str> for Range {
    fn from(s: &str) -> Range {
        let raw_bounds: Vec<&str> = s.split("-").collect();
        Range {
            low: raw_bounds[0].parse::<u8>().unwrap(),
            high: raw_bounds[1].parse::<u8>().unwrap(),
        }
    }
}

fn part_1(contents: &str) -> u64 {
    let mut redundant = 0;
    for line in contents.lines() {
        let pairs: Vec<&str> = line.split(",").collect();
        let range_1 = Range::from(pairs[0]);
        let range_2 = Range::from(pairs[1]);
        if range_1.contained_in(&range_2) || range_2.contained_in(&range_1) {
            redundant += 1;
        }
    }
    redundant
}

fn part_2(contents: &str) -> u64 {
    let mut overlap = 0;
    for line in contents.lines() {
        let pairs: Vec<&str> = line.split(",").collect();
        let range_1 = Range::from(pairs[0]);
        let range_2 = Range::from(pairs[1]);
        if range_1.overlaps(&range_2) {
            overlap += 1;
        }
    }
    overlap
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 2);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 4);
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
