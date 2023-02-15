use std::time::Instant;

fn fuel_required(module: u32) -> u32 {
    module / 3 - 2
}

fn fuel_required_iterative(mut module: u32) -> u32 {
    let mut ans: u32 = 0;
    while module > 6 {
        module = module / 3 - 2;
        ans += module;
    }
    ans
}

fn part_1(contents: &str) -> u32 {
    contents
        .lines()
        .map(|x| fuel_required(x.parse::<u32>().unwrap()))
        .sum()
}

fn part_2(contents: &str) -> u32 {
    contents
        .lines()
        .map(|x| fuel_required_iterative(x.parse::<u32>().unwrap()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(12, 2)]
    #[case(14, 2)]
    #[case(1969, 654)]
    #[case(100756, 33583)]
    fn test_part_1_examples(#[case] module: u32, #[case] expected: u32) {
        assert_eq!(fuel_required(module), expected);
    }

    #[rstest]
    #[case(12, 2)]
    #[case(14, 2)]
    #[case(1969, 966)]
    #[case(100756, 50346)]
    fn test_part_2_examples(#[case] module: u32, #[case] expected: u32) {
        assert_eq!(fuel_required_iterative(module), expected);
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
