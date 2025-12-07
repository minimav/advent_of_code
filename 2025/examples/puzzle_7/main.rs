use std::time::Instant;
use std::collections::HashSet;
use std::collections::HashMap;

fn part_1(contents: &str) -> u64 {
    let lines: Vec<&str> = contents.lines().collect();
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let mut start = (0, 0);
    let mut splitters = HashSet::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = (col, row);
            } else if ch == '^' {
                splitters.insert((col, row));
            }
        }
    }

    let mut answer = 0;
    let mut current_row = 1;
    let mut beams = HashSet::from([start]);
    while current_row < num_rows {
        let mut new_beams: HashSet<(usize, usize)> = HashSet::new();
        for beam in beams {
            let (col, row) = beam;
            if splitters.contains(&(col, row + 1)) {
                new_beams.insert((col - 1, row + 1));
                new_beams.insert((col + 1, row + 1));
                answer += 1;
            } else {
                new_beams.insert((col, row + 1));
            }
        }
        current_row += 1;
        beams = new_beams;
    }

    return answer;
}

fn part_2(contents: &str) -> u64 {
    let lines: Vec<&str> = contents.lines().collect();
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let mut start = (0, 0);
    let mut splitters = HashSet::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = (col, row);
            } else if ch == '^' {
                splitters.insert((col, row));
            }
        }
    }

    let mut answer = 0;
    let mut current_row = 1;
    // Maintain counts as we split
    let mut beams = HashMap::from([(start, 1)]);
    while current_row < num_rows {
        let mut new_beams: HashMap<(usize, usize), u64> = HashMap::new();
        for (beam, count) in &beams {
            let (col, row) = beam;
            if splitters.contains(&(*col, row + 1)) {
                new_beams.entry((*col - 1, row + 1)).and_modify(|v| *v += count).or_insert(*count);
                new_beams.entry((*col + 1, row + 1)).and_modify(|v| *v += count).or_insert(*count);
            } else {
                new_beams.entry((*col, row + 1)).and_modify(|v| *v += count).or_insert(*count);
            }
        }
        current_row += 1;
        beams = new_beams;
    }
    return beams.values().map(|v| *v).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example_1() {
        assert_eq!(part_1(include_str!("./example_1.txt")), 21);
    }

    #[test]
    fn test_part_1_example_2() {
        assert_eq!(part_1(include_str!("./example_2.txt")), 2);
    }

    #[test]
    fn test_part_1_example_3() {
        assert_eq!(part_1(include_str!("./example_3.txt")), 3);
    }

    #[test]
    fn test_part_2_example_1() {
        assert_eq!(part_2(include_str!("./example_1.txt")), 40);
    }

    #[test]
    fn test_part_2_example_2() {
        assert_eq!(part_2(include_str!("./example_2.txt")), 3);
    }

    #[test]
    fn test_part_2_example_3() {
        assert_eq!(part_2(include_str!("./example_3.txt")), 4);
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
