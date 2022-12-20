use std::collections::VecDeque;
use std::time::Instant;

fn score(deck: VecDeque<u64>) -> u64 {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i as u64 + 1) * v)
        .sum()
}

fn parse_decks(contents: &str) -> (VecDeque<u64>, VecDeque<u64>) {
    let mut player_1: VecDeque<u64> = VecDeque::new();
    let mut player_2: VecDeque<u64> = VecDeque::new();
    let mut current_player = 0;
    for line in contents.lines() {
        if line.is_empty() {
            continue;
        } else if line.contains("Player") {
            current_player += 1;
        } else {
            let number = line.parse::<u64>().unwrap();
            if current_player == 1 {
                player_1.push_back(number);
            } else {
                player_2.push_back(number);
            }
        }
    }
    (player_1, player_2)
}
fn part_1(contents: &str) -> u64 {
    let (mut player_1, mut player_2) = parse_decks(contents);
    println!("{player_1:?} vs {player_2:?}");
    loop {
        match (player_1.pop_front(), player_2.pop_front()) {
            (Some(v), None) => {
                player_1.push_front(v);
                return score(player_1);
            }
            (None, Some(v)) => {
                player_2.push_front(v);
                return score(player_2);
            }
            (Some(value_1), Some(value_2)) => {
                if value_1 > value_2 {
                    player_1.push_back(value_1);
                    player_1.push_back(value_2);
                } else if value_2 > value_1 {
                    player_2.push_back(value_2);
                    player_2.push_back(value_1);
                } else {
                    panic!("Cards should be unique!")
                }
            }
            (None, None) => panic!("Both decks are empty!"),
        }
    }
}

fn part_2(contents: &str) -> u64 {
    let (mut player_1, mut player_2) = parse_decks(contents);
    println!("{player_1:?} vs {player_2:?}");
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 306);
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
