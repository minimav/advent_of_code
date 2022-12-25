use std::{ops::Sub, time::Instant};

fn parse(line: &str) -> i128 {
    let mut line_sum = 0;
    for (index, char) in line.chars().rev().enumerate() {
        let exponent_term = 5i128.pow(index as u32);
        match char {
            '2' => line_sum += 2 * exponent_term,
            '1' => line_sum += exponent_term,
            '0' => continue,
            '-' => line_sum = line_sum.sub(exponent_term),
            '=' => line_sum = line_sum.sub(2 * exponent_term),
            _ => continue,
        };
    }
    line_sum
}

fn to_snafu(number: i128) -> String {
    let mut snafu = String::from("");
    let mut n = number.clone();
    while n > 0 {
        let modulus = n % 5;
        n /= 5;
        let char = match modulus {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "=",
            4 => "-",
            _ => panic!("Not this!"),
        };
        snafu.push_str(&char);
        if modulus >= 3 {
            n += 1;
        }
    }
    snafu.chars().rev().collect::<String>()
}

fn puzzle(contents: &str) -> String {
    let mut answer = 0;
    for line in contents.lines() {
        answer += parse(line);
    }
    to_snafu(answer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("1121-1110-1=0", 314159265)]
    #[case("1-0---0", 12345)]
    fn test_(#[case] input: &str, #[case] expected: i128) {
        assert_eq!(parse(input), expected);
    }

    #[rstest]
    #[case(314159265, "1121-1110-1=0")]
    #[case(12345, "1-0---0")]
    fn test_to_snafu(#[case] input: i128, #[case] expected: String) {
        assert_eq!(to_snafu(input), expected);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let answer = puzzle(contents);
    println!("Answer is: {}", answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
