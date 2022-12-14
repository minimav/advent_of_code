use itertools::{EitherOrBoth::*, Itertools};
use serde_json::{json, Value};
use std::cmp::Ordering;
use std::time::Instant;

fn compare(left: &Value, right: &Value) -> Ordering {
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
                        return Ordering::Less;
                    } else if left_num > right_num {
                        return Ordering::Greater;
                    }
                }
                (Value::Number(l), r) => return compare(&json!([l]), r),
                (l, Value::Number(r)) => return compare(l, &json!([r])),
                _ => match compare(left_part, right_part) {
                    Ordering::Equal => continue,
                    cmp => return cmp,
                },
            },
            Left(l) => return Ordering::Greater,
            Right(r) => return Ordering::Less,
        }
    }
    Ordering::Equal
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
        match compare(&left, &right) {
            Ordering::Less => {
                answer += index + 1;
            }
            _ => {}
        }
    }
    answer
}

#[derive(Eq, Clone)]
struct Packet(Value);

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        compare(&self.0, &other.0)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        compare(&self.0, &other.0) == Ordering::Equal
    }
}

fn part_2(contents: &str) -> usize {
    let mut packets: Vec<Packet> = Vec::new();
    let index_packet_1 = Packet(json!([[2]]));
    let index_packet_2 = Packet(json!([[6]]));
    packets.push(index_packet_1);
    packets.push(index_packet_2);

    for data in contents.split("\n\n").collect::<Vec<&str>>().iter() {
        let lines = data.lines().collect::<Vec<&str>>();
        let left_raw = lines[0];
        let right_raw = lines[1];

        let left = serde_json::from_str(left_raw).unwrap();
        let right = serde_json::from_str(right_raw).unwrap();
        packets.push(Packet(left));
        packets.push(Packet(right));
    }

    packets.sort();
    let mut answer = 1;
    let index_packet_1 = Packet(json!([[2]]));
    let index_packet_2 = Packet(json!([[6]]));
    for (index, packet) in packets.iter().enumerate() {
        if packet == &index_packet_1 || packet == &index_packet_2 {
            answer *= index + 1
        }
    }
    answer
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
        assert_eq!(part_2(include_str!("./example.txt")), 140);
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
