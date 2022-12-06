use itertools::Itertools;
use std::collections::HashSet;
use std::time::Instant;

fn part_1(contents: &str) -> u32 {
    let chunk_size: usize = 4;
    let mut end_index: u32 = chunk_size as u32;
    for (a, b, c, d) in contents.chars().tuple_windows::<(_, _, _, _)>() {
        let unique: HashSet<char> = HashSet::from([a, b, c, d]);
        if unique.len() == chunk_size {
            return end_index;
        }
        end_index += 1;
    }
    panic!("Should not get here!")
}

fn part_2(contents: &str) -> u32 {
    let chunk_size: usize = 14;
    let mut end_index: u32 = chunk_size as u32;
    let chars: Vec<char> = contents.chars().collect();
    for char_group in chars.windows(chunk_size) {
        let unique: HashSet<&char> = HashSet::from_iter(char_group.iter());
        if unique.len() == chunk_size {
            return end_index;
        }
        end_index += 1;
    }
    panic!("Should not get here!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 7);
    }

    #[rstest]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_part_1_examples(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(part_1(input), expected);
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
