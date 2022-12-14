use itertools::{EitherOrBoth::*, Itertools};
use serde_json::{json, Value};
use std::time::Instant;

fn compare(left: &Value, right: &Value) -> i8 {
    for pair in left
        .as_array()
        .unwrap()
        .iter()
        .zip_longest(right.as_array().unwrap().iter())
    {
        match pair {
            Both(left_part, right_part) => match (left_part, right_part) {
                (Value::Number(l), Value::Number(r)) => {
                    let left_num = l.as_u64();
                    let right_num = r.as_u64();
                    if left_num < right_num {
                        return -1;
                    } else if left_num > right_num {
                        return 1;
                    }
                }
                (Value::Number(l), r) => return compare(&json!([l]), r),
                (l, Value::Number(r)) => return compare(l, &json!([r])),
                _ => {
                    let cmp = compare(left_part, right_part);
                    if cmp == 0 {
                        continue;
                    }
                    return cmp;
                }
            },
            Left(l) => return 1,
            Right(r) => return -1,
        }
    }
    0
}

fn part_1(contents: &str) -> usize {
    let mut answer = 0;
    for (index, data) in contents
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .enumerate()
    {
        let lines = data.lines().collect::<Vec<&str>>();
        let left_raw = lines[0];
        let right_raw = lines[1];

        let left = serde_json::from_str(left_raw).unwrap();
        let right = serde_json::from_str(right_raw).unwrap();
        if compare(&left, &right) == -1 {
            answer += index + 1;
        }
    }
    answer
}

fn part_2(contents: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("[1,1,3,1,1]", "[1,1,5,1,1]", -1)]
    #[case("[[1],[2,3,4]]", "[[1],4]", -1)]
    #[case("[9]", "[[8,7,6]]", 1)]
    #[case("[[4,4],4,4]", "[[4,4],4,4,4]", -1)]
    #[case("[7,7,7,7]", "[7,7,7]", 1)]
    #[case("[[[]]]", "[[]]", 1)]
    #[case("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]", 1)]
    fn test_comparison(#[case] left_raw: &str, #[case] right_raw: &str, #[case] in_order: i8) {
        let left = serde_json::from_str(left_raw).unwrap();
        let right = serde_json::from_str(right_raw).unwrap();
        assert_eq!(compare(&left, &right), in_order);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 13);
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
