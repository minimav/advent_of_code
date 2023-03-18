use std::time::Instant;

fn count_char(chars: &[char], char_to_match: char) -> usize {
    chars.iter().filter(|x| **x == char_to_match).count()
}

fn part_1(contents: &str, width: usize, height: usize) -> usize {
    let chars: Vec<char> = contents.chars().collect();
    let mut min_zeroes: usize = usize::MAX;
    let mut min_zeroes_digit_calc: usize = 0;
    for layer in chars.chunks(width * height) {
        let num_zeroes = count_char(layer, '0');
        if num_zeroes < min_zeroes {
            min_zeroes = num_zeroes;
            let num_ones = count_char(layer, '1');
            let num_twos = count_char(layer, '2');
            min_zeroes_digit_calc = num_ones * num_twos;
        }
    }
    min_zeroes_digit_calc
}

fn part_2(contents: &str, width: usize, height: usize) -> String {
    let mut output: Vec<Vec<u32>> = vec![vec![2; width]; height];

    let chars: Vec<char> = contents.chars().collect();
    for layer in chars.chunks(width * height) {
        for (index, char) in layer.iter().enumerate() {
            let row = (index / width) as usize;
            let column = (index % width) as usize;
            let current_value = output[row][column];
            let digit = char.to_digit(10).unwrap();
            if current_value == 2 {
                output[row][column] = digit;
            }
        }
    }

    let joined_rows: Vec<String> = output
        .iter()
        .map(|x| {
            x.into_iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect();

    for row in joined_rows.iter() {
        println!("{row}");
    }
    joined_rows.join("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("123456789012", 1)]
    #[case("122326789012", 3)]
    #[case("100456712012", 4)]
    fn test_part_1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part_1(input, 3, 2), expected);
    }

    #[rstest]
    #[case("0222112222120000", String::from("0110"))]
    fn test_part_2(#[case] input: &str, #[case] expected: String) {
        assert_eq!(part_2(input, 2, 2), expected);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let part_1_answer = part_1(contents, 25, 6);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer = part_2(contents, 25, 6);
    println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
