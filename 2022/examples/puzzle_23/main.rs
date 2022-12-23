use num::complex::Complex;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    NORTH_WEST,
    NORTH_EAST,
    SOUTH_EAST,
    SOUTH_WEST,
}

fn neighbours(point: &Complex<i64>) -> HashMap<Direction, Complex<i64>> {
    HashMap::from_iter(vec![
        (Direction::EAST, point + Complex::new(1, 0)),
        (Direction::WEST, point + Complex::new(-1, 0)),
        (Direction::NORTH, point + Complex::new(0, -1)),
        (Direction::SOUTH, point + Complex::new(0, 1)),
        (Direction::SOUTH_EAST, point + Complex::new(1, 1)),
        (Direction::NORTH_EAST, point + Complex::new(1, -1)),
        (Direction::NORTH_WEST, point + Complex::new(-1, -1)),
        (Direction::SOUTH_WEST, point + Complex::new(-1, 1)),
    ])
}

fn check_north(
    point: &Complex<i64>,
    has_neighbours: &HashMap<Direction, bool>,
) -> Option<Complex<i64>> {
    // If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
    if !has_neighbours.get(&Direction::NORTH).unwrap()
        && !has_neighbours.get(&Direction::NORTH_EAST).unwrap()
        && !has_neighbours.get(&Direction::NORTH_WEST).unwrap()
    {
        Some(point + Complex::new(0, -1))
    } else {
        None
    }
}

fn check_south(
    point: &Complex<i64>,
    has_neighbours: &HashMap<Direction, bool>,
) -> Option<Complex<i64>> {
    // If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
    if !has_neighbours.get(&Direction::SOUTH).unwrap()
        && !has_neighbours.get(&Direction::SOUTH_EAST).unwrap()
        && !has_neighbours.get(&Direction::SOUTH_WEST).unwrap()
    {
        Some(point + Complex::new(0, 1))
    } else {
        None
    }
}

fn check_east(
    point: &Complex<i64>,
    has_neighbours: &HashMap<Direction, bool>,
) -> Option<Complex<i64>> {
    // If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
    if !has_neighbours.get(&Direction::EAST).unwrap()
        && !has_neighbours.get(&Direction::NORTH_EAST).unwrap()
        && !has_neighbours.get(&Direction::SOUTH_EAST).unwrap()
    {
        Some(point + Complex::new(1, 0))
    } else {
        None
    }
}

fn check_west(
    point: &Complex<i64>,
    has_neighbours: &HashMap<Direction, bool>,
) -> Option<Complex<i64>> {
    // If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
    if !has_neighbours.get(&Direction::WEST).unwrap()
        && !has_neighbours.get(&Direction::SOUTH_WEST).unwrap()
        && !has_neighbours.get(&Direction::NORTH_WEST).unwrap()
    {
        Some(point + Complex::new(-1, 0))
    } else {
        None
    }
}

fn get_answer(grid: HashSet<Complex<i64>>) -> i64 {
    let min_x = grid.iter().map(|p| p.re).min().unwrap();
    let max_x = grid.iter().map(|p| p.re).max().unwrap();
    let min_y = grid.iter().map(|p| p.im).min().unwrap();
    let max_y = grid.iter().map(|p| p.im).max().unwrap();
    (max_x - min_x + 1) * (max_y - min_y + 1) - (grid.len() as i64)
}

fn print_grid(grid: &HashSet<Complex<i64>>) {
    let min_x = grid.iter().map(|p| p.re).min().unwrap();
    let max_x = grid.iter().map(|p| p.re).max().unwrap();
    let min_y = grid.iter().map(|p| p.im).min().unwrap();
    let max_y = grid.iter().map(|p| p.im).max().unwrap();
    let mut grid_vec: Vec<Vec<char>> =
        vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    for point in grid.iter() {
        grid_vec[(point.im - min_y) as usize][(point.re - min_x) as usize] = '#'
    }
    for row in grid_vec {
        for char in row {
            print!("{char}");
        }
        println!("");
    }
}

fn simulate(contents: &str, num_rounds: usize) -> (HashSet<Complex<i64>>, usize) {
    let mut grid: HashSet<Complex<i64>> = HashSet::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                grid.insert(Complex::new(x as i64, y as i64));
            }
        }
    }
    let num_elves = grid.len();
    let mut rounds_completed = 0;
    for round in 0..num_rounds {
        let mut new_grid: HashSet<Complex<i64>> = HashSet::with_capacity(num_elves);
        let mut new_point_counts: HashMap<Complex<i64>, usize> = HashMap::with_capacity(num_elves);
        let mut potential_moves: HashMap<Complex<i64>, Complex<i64>> =
            HashMap::with_capacity(num_elves);
        let mut no_moves = 0;

        for point in grid.iter() {
            let neighbours = neighbours(&point);
            let has_neighbours: HashMap<Direction, bool> =
                HashMap::from_iter(neighbours.into_iter().map(|(d, p)| (d, grid.contains(&p))));
            let possible = if has_neighbours.iter().all(|(_, x)| !x) {
                None
            } else {
                let possibles: Vec<Option<Complex<i64>>> = vec![
                    check_north(&point, &has_neighbours),
                    check_south(&point, &has_neighbours),
                    check_west(&point, &has_neighbours),
                    check_east(&point, &has_neighbours),
                ];

                possibles
                    .into_iter()
                    .cycle()
                    .skip(round % 4)
                    .take(4)
                    .flatten()
                    .next()
            };

            match possible {
                Some(new_point) => {
                    new_point_counts
                        .entry(new_point)
                        .and_modify(|x| *x += 1)
                        .or_insert(1);
                    potential_moves.insert(point.clone(), new_point.clone());
                }
                _ => {
                    new_grid.insert(*point);
                    no_moves += 1;
                }
            }
        }

        for (point, other_point) in potential_moves {
            match new_point_counts.get(&other_point) {
                Some(c) if c > &1 => {
                    new_grid.insert(point);
                    no_moves += 1;
                }
                Some(_) => {
                    new_grid.insert(other_point);
                }
                None => panic!("Should not be missing here!"),
            }
        }
        rounds_completed += 1;
        if no_moves == num_elves {
            break;
        }
        grid = new_grid;
    }
    (grid, rounds_completed)
}

fn part_1(contents: &str, num_rounds: usize) -> i64 {
    let (grid, _) = simulate(contents, num_rounds);
    get_answer(grid)
}

fn part_2(contents: &str) -> usize {
    let (_, rounds_completed) = simulate(contents, 1_000_000);
    rounds_completed
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_smaller_example_one_move() {
        assert_eq!(part_1(include_str!("./smaller_example.txt"), 1), 5);
    }

    #[test]
    fn test_part_1_smaller_example_two_moves() {
        assert_eq!(part_1(include_str!("./smaller_example.txt"), 2), 15);
    }

    #[test]
    fn test_part_1_smaller_example_three_moves() {
        assert_eq!(part_1(include_str!("./smaller_example.txt"), 3), 25);
    }

    #[test]
    fn test_part_1_example_one_move() {
        // 9 x 9 rectangle bounds elves, 22 elves
        assert_eq!(part_1(include_str!("./example.txt"), 1), 9 * 9 - 22);
    }

    #[test]
    fn test_part_1_example_two_moves() {
        // 9 x 11 rectangle bounds elves, 22 elves
        assert_eq!(part_1(include_str!("./example.txt"), 2), 9 * 11 - 22);
    }

    #[test]
    fn test_part_1_example_three_moves() {
        // 10 x 11 rectangle bounds elves, 22 elves
        assert_eq!(part_1(include_str!("./example.txt"), 3), 10 * 11 - 22);
    }

    #[test]
    fn test_part_1_example_four_moves() {
        // 10 x 11 rectangle bounds elves, 22 elves
        assert_eq!(part_1(include_str!("./example.txt"), 4), 10 * 11 - 22);
    }

    #[test]
    fn test_part_1_example_five_moves() {
        // 11 x 11 rectangle bounds elves, 22 elves
        assert_eq!(part_1(include_str!("./example.txt"), 5), 11 * 11 - 22);
    }

    #[test]
    fn test_part_1_example_ten_moves() {
        assert_eq!(part_1(include_str!("./example.txt"), 10), 110);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 20);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let part_1_answer = part_1(contents, 10);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer = part_2(contents);
    println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
