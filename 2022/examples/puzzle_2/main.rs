use std::time::Instant;

#[derive(PartialEq, Clone)]
enum Move {
    ROCK,
    PAPER,
    SCISSORS,
}

#[derive(PartialEq)]
enum Outcome {
    WIN,
    LOSE,
    DRAW,
}

trait Points {
    fn points(&self) -> u64;
}

impl Points for Move {
    fn points(&self) -> u64 {
        match self {
            Move::ROCK => 1,
            Move::PAPER => 2,
            Move::SCISSORS => 3,
        }
    }
}

impl Points for Outcome {
    fn points(&self) -> u64 {
        match self {
            Outcome::WIN => 6,
            Outcome::LOSE => 0,
            Outcome::DRAW => 3,
        }
    }
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Move::ROCK,
            "B" | "Y" => Move::PAPER,
            "C" | "Z" => Move::SCISSORS,
            _ => panic!("Could not convert string to Move"),
        }
    }
}

impl From<&str> for Outcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Outcome::LOSE,
            "Y" => Outcome::DRAW,
            "Z" => Outcome::WIN,
            _ => panic!("Could not convert string to Outcome"),
        }
    }
}

impl Move {
    fn compare(&self, other: &Move) -> Outcome {
        match (self, other) {
            (Move::ROCK, Move::PAPER) => Outcome::LOSE,
            (Move::ROCK, Move::SCISSORS) => Outcome::WIN,
            (Move::PAPER, Move::ROCK) => Outcome::WIN,
            (Move::PAPER, Move::SCISSORS) => Outcome::LOSE,
            (Move::SCISSORS, Move::ROCK) => Outcome::LOSE,
            (Move::SCISSORS, Move::PAPER) => Outcome::WIN,
            _ => Outcome::DRAW,
        }
    }

    fn move_for_outcome(&self, outcome: &Outcome) -> Move {
        match (self, outcome) {
            (_, Outcome::DRAW) => self.clone(),
            (Move::ROCK, Outcome::WIN) => Move::PAPER,
            (Move::ROCK, Outcome::LOSE) => Move::SCISSORS,
            (Move::PAPER, Outcome::WIN) => Move::SCISSORS,
            (Move::PAPER, Outcome::LOSE) => Move::ROCK,
            (Move::SCISSORS, Outcome::WIN) => Move::ROCK,
            (Move::SCISSORS, Outcome::LOSE) => Move::PAPER,
        }
    }
}

fn part_1(contents: &str) -> u64 {
    let mut points: u64 = 0;
    for line in contents.lines() {
        let moves = line.split_whitespace().collect::<Vec<&str>>();
        let their_move = Move::from(moves[0]);
        let my_move = Move::from(moves[1]);
        points += my_move.points();
        points += my_move.compare(&their_move).points();
    }
    points
}

fn part_2(contents: &str) -> u64 {
    let mut points: u64 = 0;
    for line in contents.lines() {
        let moves = line.split_whitespace().collect::<Vec<&str>>();
        let their_move = Move::from(moves[0]);
        let outcome = Outcome::from(moves[1]);
        points += outcome.points();
        points += their_move.move_for_outcome(&outcome).points();
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 15);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 12);
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
