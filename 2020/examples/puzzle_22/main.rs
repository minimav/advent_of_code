use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

enum Winner {
    PLAYER_1,
    PLAYER_2,
}

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

fn game(
    mut player_1: VecDeque<u64>,
    mut player_2: VecDeque<u64>,
    seen_games: &mut HashMap<(Vec<u64>, Vec<u64>), Winner>,
) -> (Winner, u64) {
    let mut hands_seen: HashSet<(Vec<u64>, Vec<u64>)> = HashSet::new();
    let original_key = (
        Vec::from_iter(player_1.clone()),
        Vec::from_iter(player_2.clone()),
    );
    loop {
        // do played game before and recursion checks
        let key = (
            Vec::from_iter(player_1.clone()),
            Vec::from_iter(player_2.clone()),
        );
        match seen_games.get(&key) {
            Some(winner) => match winner {
                Winner::PLAYER_1 => return (Winner::PLAYER_1, score(player_1)),
                Winner::PLAYER_2 => return (Winner::PLAYER_2, score(player_2)),
            },
            _ => (),
        }
        if hands_seen.contains(&key) {
            seen_games.insert(original_key, Winner::PLAYER_1);
            return (Winner::PLAYER_1, score(player_1));
        }
        hands_seen.insert(key.clone());

        match (player_1.pop_front(), player_2.pop_front()) {
            (Some(v), None) => {
                player_1.push_front(v);
                seen_games.insert(original_key, Winner::PLAYER_1);
                return (Winner::PLAYER_1, score(player_1));
            }
            (None, Some(v)) => {
                player_2.push_front(v);
                seen_games.insert(original_key, Winner::PLAYER_2);
                return (Winner::PLAYER_2, score(player_2));
            }
            (Some(value_1), Some(value_2)) => {
                let winner = if value_1 <= (player_1.len() as u64)
                    && value_2 <= (player_2.len() as u64)
                {
                    game(
                        VecDeque::from_iter(player_1.clone().into_iter().take(value_1 as usize)),
                        VecDeque::from_iter(player_2.clone().into_iter().take(value_2 as usize)),
                        seen_games,
                    )
                    .0
                } else if value_1 > value_2 {
                    Winner::PLAYER_1
                } else {
                    Winner::PLAYER_2
                };
                match winner {
                    Winner::PLAYER_1 => {
                        player_1.push_back(value_1);
                        player_1.push_back(value_2);
                    }
                    Winner::PLAYER_2 => {
                        player_2.push_back(value_2);
                        player_2.push_back(value_1);
                    }
                }
            }
            (None, None) => panic!("Both decks are empty!"),
        }
    }
}

fn part_2(contents: &str) -> u64 {
    let (player_1, player_2) = parse_decks(contents);
    let mut seen_games: HashMap<(Vec<u64>, Vec<u64>), Winner> = HashMap::new();
    let (_, score) = game(player_1, player_2, &mut seen_games);
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_score() {
        assert_eq!(
            score(VecDeque::from_iter([7, 5, 6, 2, 4, 1, 10, 8, 9, 3])),
            291
        );
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 306);
    }

    #[test]
    fn test_recursion_check() {
        let player_1 = VecDeque::from_iter([43, 19]);
        let player_2 = VecDeque::from_iter([2, 29, 14]);
        let mut seen_games: HashMap<(Vec<u64>, Vec<u64>), Winner> = HashMap::new();
        game(player_1, player_2, &mut seen_games);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 291);
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
