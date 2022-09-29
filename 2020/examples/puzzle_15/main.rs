use std::collections::HashMap;
use std::time::Instant;

fn memory_game(contents: &str, num_turns: u64) -> u64 {
    let mut turn: u64 = 1;
    let mut last_seen: HashMap<u64, u64> = HashMap::new();
    for raw_value in contents.split(",") {
        let value = raw_value.parse::<u64>().unwrap();
        last_seen.insert(value, turn);
        turn += 1;
    }

    let mut current_value: u64 = 0;
    while turn < num_turns {
        let possible_previous_turn = last_seen.get(&current_value);
        let old_current_value = current_value;
        match possible_previous_turn {
            Some(previous_turn) => {
                current_value = turn - previous_turn;
            }
            None => {
                current_value = 0;
            }
        }
        last_seen.insert(old_current_value, turn);
        //println!("Turn {}: {}", turn, current_value);
        turn += 1;
    }
    current_value
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("0,3,6", 436)]
    #[case("1,3,2", 1)]
    #[case("2,1,3", 10)]
    #[case("1,2,3", 27)]
    #[case("2,3,1", 78)]
    #[case("3,2,1", 438)]
    #[case("3,1,2", 1836)]
    fn test_part_1_examples(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(memory_game(input, 2020), expected);
    }

    #[rstest]
    #[case("0,3,6", 175594)]
    #[case("1,3,2", 2578)]
    #[case("2,1,3", 3544142)]
    #[case("1,2,3", 261214)]
    #[case("2,3,1", 6895259)]
    #[case("3,2,1", 18)]
    #[case("3,1,2", 362)]
    fn test_part_2_examples(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(memory_game(input, 30000000), expected);
    }
}

fn main() {
    let start = Instant::now();
    let input = "0,5,4,1,10,14,7";
    let part_1_answer = memory_game(input, 2020);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer = memory_game(input, 30000000);
    println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
