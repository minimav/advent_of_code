use core::num;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl From<&str> for Direction {
    fn from(raw: &str) -> Self {
        match raw {
            "D" => Direction::DOWN,
            "U" => Direction::UP,
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            _ => panic!("Can't parse direction!"),
        }
    }
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    steps: i32,
}

impl From<Vec<&str>> for Command {
    fn from(raw: Vec<&str>) -> Self {
        let direction = Direction::from(raw[0]);
        let steps = raw[1].parse::<i32>().unwrap();
        Command { direction, steps }
    }
}

impl Direction {
    fn make_move(&self, knot: &mut (i32, i32)) {
        match self {
            Direction::DOWN => knot.1 -= 1,
            Direction::UP => knot.1 += 1,
            Direction::LEFT => knot.0 -= 1,
            Direction::RIGHT => knot.0 += 1,
        };
    }
}

fn update_knot_from_delta(delta: (i32, i32), knot: (i32, i32)) -> (i32, i32) {
    match delta {
        // delta is a line case
        (2, 0) => (knot.0 + 1, knot.1),
        (-2, 0) => (knot.0 - 1, knot.1),
        (0, 2) => (knot.0, knot.1 + 1),
        (0, -2) => (knot.0, knot.1 - 1),
        // adjacent cases where no move is required
        (x, y) if x.abs() <= 1 && y.abs() <= 1 => knot,
        (x, y) => {
            // one step diagonal move to keep up
            (knot.0 + x.signum(), knot.1 + y.signum())
        }
    }
}

fn knot_mover(contents: &str, num_knots: usize) -> u64 {
    let mut knots: Vec<(i32, i32)> = Vec::new();
    for _ in 0..num_knots {
        knots.push((0, 0));
    }
    let mut tail_knot_positions: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    for line in contents.lines() {
        let command = Command::from(line.split_whitespace().collect::<Vec<&str>>());
        for _ in 0..command.steps {
            command.direction.make_move(&mut knots[0]);

            for index in 0..num_knots - 1 {
                let previous_knot = knots[index];
                let knot = knots[index + 1];
                let delta = (previous_knot.0 - knot.0, previous_knot.1 - knot.1);
                knots[index + 1] = update_knot_from_delta(delta, knot);
            }
            tail_knot_positions.insert(knots[num_knots - 1]);
        }
    }
    tail_knot_positions.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(knot_mover(include_str!("./example_1.txt"), 2), 13);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(knot_mover(include_str!("./example_1.txt"), 10), 1);
    }

    #[test]
    fn test_part_2_larger_example() {
        assert_eq!(knot_mover(include_str!("./example_2.txt"), 10), 36);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let part_1_answer = knot_mover(contents, 2);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer = knot_mover(contents, 10);
    println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
