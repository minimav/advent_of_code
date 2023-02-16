use std::time::Instant;

fn check_password(password: &u64) -> bool {
    let digits: Vec<u64> = (0..=5)
        .rev()
        .map(|x| (password / 10u64.pow(x)) % 10)
        .collect();

    // must be at least one pair of repeated digits
    if !digits.windows(2).any(|x| x[0] == x[1]) {
        return false;
    }

    // must be monotically increasing
    if digits.windows(2).any(|x| x[0] > x[1]) {
        return false;
    }

    true
}

fn check_password_enhanced(password: &u64) -> bool {
    let digits: Vec<u64> = (0..=5)
        .rev()
        .map(|x| (password / 10u64.pow(x)) % 10)
        .collect();

    // must be monotically increasing
    if digits.windows(2).any(|x| x[0] > x[1]) {
        return false;
    }

    let mut index: usize = 1;
    let mut num_repeats = 1;
    let mut current_digit = digits[0];
    while index < digits.len() {
        let next_digit = digits[index];
        if next_digit == current_digit {
            num_repeats += 1;
        } else {
            if num_repeats == 2 {
                return true;
            }
            num_repeats = 1;
            current_digit = next_digit;
        }
        index += 1;
    }
    // check if final pair of digits satisfy the condition
    return num_repeats == 2;
}

fn part_1(contents: &str) -> usize {
    let bounds: Vec<u64> = contents
        .split("-")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let lower_bound = bounds[0];
    let upper_bound = bounds[1];
    (lower_bound..=upper_bound).filter(check_password).count()
}

fn part_2(contents: &str) -> usize {
    let bounds: Vec<u64> = contents
        .split("-")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let lower_bound = bounds[0];
    let upper_bound = bounds[1];
    (lower_bound..=upper_bound)
        .filter(check_password_enhanced)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(111111, true)]
    #[case(223450, false)]
    #[case(123789, false)]
    fn test_check_password(#[case] password: u64, #[case] expected: bool) {
        assert_eq!(check_password(&password), expected);
    }

    #[rstest]
    #[case(112233, true)]
    #[case(123444, false)]
    #[case(111122, true)]
    fn test_check_password_enhanced(#[case] password: u64, #[case] expected: bool) {
        assert_eq!(check_password_enhanced(&password), expected);
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
