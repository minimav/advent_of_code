use itertools::Itertools;
use std::time::Instant;

fn find_product(contents: &str, target: u32, combination_size: usize) -> Option<u32> {
    contents
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .combinations(combination_size)
        .filter(|x| x.iter().sum::<u32>() == target)
        .map(|x| x.into_iter().product())
        .next()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(
            find_product(include_str!("./example.txt"), 2020, 2),
            Some(514579)
        );
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(
            find_product(include_str!("./example.txt"), 2020, 3),
            Some(241861950)
        );
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let part_1_answer = find_product(contents, 2020, 2);
    println!("Answer for part 1 is: {:?}", part_1_answer);
    let part_2_answer = find_product(contents, 2020, 3);
    println!("Answer for part 2 is: {:?}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
