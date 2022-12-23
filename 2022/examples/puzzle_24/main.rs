use std::time::Instant;

fn part_1(contents: &str) -> u64 {
    0
}

fn part_2(contents: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("<input>", 1)]
    fn test_(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(part_1(input), expected);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 1);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 1);
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
