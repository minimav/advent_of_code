use std::time::Instant;

fn part_1(contents: &str) -> u64 {
    let mut points: u64 = 0;
    for line in contents.lines() {
        let moves = line.split_whitespace().collect::<Vec<&str>>();

        // points for move
        match moves[1] {
            "X" => {
                points += 1;
            }
            "Y" => {
                points += 2;
            }
            "Z" => {
                points += 3;
            }
            _ => {}
        }

        // points for round
        if (moves[0] == "A" && moves[1] == "Y")
            || (moves[0] == "B" && moves[1] == "Z")
            || (moves[0] == "C" && moves[1] == "X")
        {
            points += 6;
        } else if (moves[0] == "A" && moves[1] == "X")
            || (moves[0] == "B" && moves[1] == "Y")
            || (moves[0] == "C" && moves[1] == "Z")
        {
            points += 3;
        }
    }
    points
}

fn part_2(contents: &str) -> u64 {
    let mut points: u64 = 0;
    for line in contents.lines() {
        let moves = line.split_whitespace().collect::<Vec<&str>>();

        // points result
        match moves[1] {
            "Y" => {
                points += 3;
            }
            "Z" => {
                points += 6;
            }
            _ => {}
        }

        // points for move
        if moves[0] == "A" {
            // rock
            if moves[1] == "X" {
                // scissors to lose
                points += 3;
            } else if moves[1] == "Y" {
                // rock to draw
                points += 1;
            } else {
                // paper to win
                points += 2;
            }
        } else if moves[0] == "B" {
            // paper
            if moves[1] == "X" {
                // rock to lose
                points += 1;
            } else if moves[1] == "Y" {
                // paper to draw
                points += 2;
            } else {
                // scissors to win
                points += 3;
            }
        } else if moves[0] == "C" {
            // scissors
            if moves[1] == "X" {
                // paper to lose
                points += 2;
            } else if moves[1] == "Y" {
                // scissors to draw
                points += 3;
            } else {
                // rock to win
                points += 1;
            }
        }
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
