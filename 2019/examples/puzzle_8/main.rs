use std::time::Instant;

fn count_char(chars: &[char], char_to_match: char) -> usize {
    chars.iter().filter(|x| **x == char_to_match).count()
}

fn part_1(contents: &str, width: usize, height: usize) -> usize {
    let chars: Vec<char> = contents.chars().collect();
    let mut min_zeroes: usize = usize::MAX;
    let mut min_zeroes_digit_calc: usize = 0;
    for chunk in chars.chunks(width * height) {
        let num_zeroes = count_char(chunk, '0');
        if num_zeroes < min_zeroes {
            min_zeroes = num_zeroes;
            let num_ones = count_char(chunk, '1');
            let num_twos = count_char(chunk, '2');
            min_zeroes_digit_calc = num_ones * num_twos;
        }
    }
    min_zeroes_digit_calc
}

fn part_2(contents: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("123456789012", 1)]
    #[case("122326789012", 3)]
    #[case("100456712012", 4)]
    fn test_(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part_1(input, 3, 2), expected);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let part_1_answer = part_1(contents, 25, 6);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer = part_2(contents);
    println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
