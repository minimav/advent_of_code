use std::collections::HashSet;
use std::time::Instant;

fn parse_grid(contents: &str) -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in contents.lines() {
        let row: Vec<u32> = line
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|x| x.to_digit(10).unwrap())
            .collect();
        grid.push(row);
    }
    grid
}

enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

struct LineOfTrees<'a> {
    trees: &'a Vec<&'a u32>,
    row_index: usize,
    column_index: usize,
    direction: Direction,
}

fn num_visible_in_direction(line: &LineOfTrees, visible: &mut HashSet<(usize, usize)>) {
    // start tree is on the boundary so always visible
    visible.insert((line.row_index, line.column_index));
    let mut min = line.trees[0];
    for (index, tree) in line.trees.iter().skip(1).enumerate() {
        if tree > &min {
            min = *tree;
            match line.direction {
                Direction::RIGHT => {
                    visible.insert((line.row_index, line.column_index + index + 1));
                }
                Direction::LEFT => {
                    visible.insert((line.row_index, line.column_index - index - 1));
                }
                Direction::DOWN => {
                    visible.insert((line.row_index + index + 1, line.column_index));
                }
                Direction::UP => {
                    visible.insert((line.row_index - index - 1, line.column_index));
                }
            }
        }
    }
}

fn part_1(contents: &str) -> u64 {
    let grid = parse_grid(contents);
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    for (row_index, row) in grid.iter().enumerate() {
        let to_right = LineOfTrees {
            trees: &row.iter().map(|x| x).collect(),
            row_index: row_index,
            column_index: 0,
            direction: Direction::RIGHT,
        };
        num_visible_in_direction(&to_right, &mut visible);

        let to_left = LineOfTrees {
            trees: &row.iter().rev().collect(),
            row_index: row_index,
            column_index: row.len() - 1,
            direction: Direction::LEFT,
        };
        num_visible_in_direction(&to_left, &mut visible);
    }
    // ignore first and last columns as left/right cover them
    for column_index in 1..grid[0].len() - 1 {
        let to_bottom = LineOfTrees {
            trees: &grid.iter().map(|x| &x[column_index]).collect(),
            row_index: 0,
            column_index: column_index,
            direction: Direction::DOWN,
        };
        num_visible_in_direction(&to_bottom, &mut visible);

        let to_top = LineOfTrees {
            trees: &grid.iter().rev().map(|x| &x[column_index]).collect(),
            row_index: grid.len() - 1,
            column_index: column_index,
            direction: Direction::UP,
        };
        num_visible_in_direction(&to_top, &mut visible);
    }
    visible.len() as u64
}

fn part_2(contents: &str) -> u64 {
    let grid = parse_grid(contents);
    let mut best_scenic_score: u64 = 0;
    for (row_index, row) in grid.iter().enumerate() {
        if row_index == 0 || row_index == grid.len() - 1 {
            continue;
        }
        for (column_index, value) in row.iter().enumerate() {
            if column_index == 0 || column_index == row.len() - 1 {
                continue;
            }

            let mut scenic_score = 1;

            // check left
            let mut direction_score = 0;
            let mut traverse_column_index = column_index - 1;
            while traverse_column_index >= 0 {
                let next_value = grid[row_index][traverse_column_index];
                if next_value <= *value {
                    direction_score += 1;
                    if traverse_column_index == 0 || next_value == *value {
                        break;
                    } else {
                        traverse_column_index -= 1;
                    }
                } else {
                    direction_score += 1;
                    break;
                }
            }
            println!(
                "{} {:?} left score {}",
                value,
                (row_index, column_index),
                direction_score
            );
            scenic_score *= direction_score;

            // check right
            let mut direction_score = 0;
            let mut traverse_column_index = column_index + 1;
            while traverse_column_index < row.len() {
                let next_value = grid[row_index][traverse_column_index];
                if next_value <= *value {
                    direction_score += 1;
                    if traverse_column_index == row.len() - 1 || next_value == *value {
                        break;
                    } else {
                        traverse_column_index += 1;
                    }
                } else {
                    direction_score += 1;
                    break;
                }
            }
            println!(
                "{} {:?} right score {}",
                value,
                (row_index, column_index),
                direction_score
            );
            scenic_score *= direction_score;

            // check bottom
            let mut direction_score = 0;
            let mut traverse_row_index = row_index + 1;
            while traverse_row_index < grid.len() {
                let next_value = grid[traverse_row_index][column_index];
                if next_value <= *value {
                    direction_score += 1;
                    if traverse_row_index == grid.len() - 1 || next_value == *value {
                        break;
                    } else {
                        traverse_row_index += 1;
                    }
                } else {
                    direction_score += 1;
                    break;
                }
            }
            println!(
                "{} {:?} bottom score {}",
                value,
                (row_index, column_index),
                direction_score
            );
            scenic_score *= direction_score;

            // check down
            let mut direction_score = 0;
            let mut traverse_row_index = row_index - 1;
            while traverse_row_index >= 0 {
                let next_value = grid[traverse_row_index][column_index];
                if next_value <= *value {
                    direction_score += 1;
                    if traverse_row_index == 0 || next_value == *value {
                        break;
                    } else {
                        traverse_row_index -= 1;
                    }
                } else {
                    direction_score += 1;
                    break;
                }
            }
            println!(
                "{} {:?} up score {}",
                value,
                (row_index, column_index),
                direction_score
            );
            scenic_score *= direction_score;

            if scenic_score > best_scenic_score {
                println!(
                    "New best {}, {:?}: {}",
                    value,
                    (row_index, column_index),
                    scenic_score
                );
                best_scenic_score = scenic_score;
            }
        }
    }
    best_scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 21);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 8);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let part_1_answer = part_1(contents);
    println!("Answer for part 1 is: {}", part_1_answer);
    //let part_2_answer = part_2(contents);
    //println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
