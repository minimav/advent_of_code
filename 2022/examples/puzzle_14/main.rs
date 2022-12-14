use std::collections::HashSet;
use std::time::Instant;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let components = s.split(",").collect::<Vec<&str>>();
        Point {
            x: components[0].parse::<u32>().unwrap(),
            y: components[1].parse::<u32>().unwrap(),
        }
    }
}

fn part_1(contents: &str) -> u64 {
    let mut rocks: Vec<Vec<Point>> = Vec::new();
    let mut filled_squares: HashSet<Point> = HashSet::new();
    for line in contents.lines() {
        let rock_line: Vec<Point> = line
            .replace(" ", "")
            .split("->")
            .map(|x| Point::from(x))
            .collect::<Vec<_>>();
        rocks.push(rock_line)
    }

    for line in rocks.iter() {
        for pair in line.windows(2) {
            let start = pair[0];
            let end = pair[1];
            if start.x == end.x && start.y < end.y {
                // vertical line
                for y in start.y..=end.y {
                    filled_squares.insert(Point { x: start.x, y });
                }
            } else if start.x == end.x && start.y > end.y {
                // vertical line
                for y in end.y..=start.y {
                    filled_squares.insert(Point { x: start.x, y });
                }
            } else if start.x < end.x {
                // horizontal line
                for x in start.x..=end.x {
                    filled_squares.insert(Point { x, y: start.y });
                }
            } else {
                // horizontal line
                for x in end.x..=start.x {
                    filled_squares.insert(Point { x, y: start.y });
                }
            }
        }
    }

    let max_x = rocks.iter().fold(0, |max, r| {
        let line_max = r.iter().map(|l| l.x).max().unwrap();
        if line_max > max {
            line_max
        } else {
            max
        }
    });
    let max_y = rocks.iter().fold(0, |max, r| {
        let line_max = r.iter().map(|l| l.y).max().unwrap();
        if line_max > max {
            line_max
        } else {
            max
        }
    });
    let min_x = rocks.iter().fold(u32::MAX, |min, r| {
        let line_min = r.iter().map(|l| l.x).min().unwrap();
        if line_min < min {
            line_min
        } else {
            min
        }
    });
    let min_y = rocks.iter().fold(u32::MAX, |min, r| {
        let line_min = r.iter().map(|l| l.y).min().unwrap();
        if line_min < min {
            line_min
        } else {
            min
        }
    });

    let mut num_sand_particles: u64 = 0;
    'outer: loop {
        let mut sand = Point { x: 500, y: 0 };
        'inner: loop {
            // check out of bounds
            if sand.x > max_x || sand.x < min_x || sand.y >= max_y {
                break 'outer;
            }
            let below = Point {
                x: sand.x,
                y: sand.y + 1,
            };
            let diag_left = Point {
                x: sand.x - 1,
                y: sand.y + 1,
            };
            let diag_right = Point {
                x: sand.x + 1,
                y: sand.y + 1,
            };
            if !filled_squares.contains(&below) {
                sand = below;
            } else if !filled_squares.contains(&diag_left) {
                sand = diag_left;
            } else if !filled_squares.contains(&diag_right) {
                sand = diag_right;
            } else {
                filled_squares.insert(sand);
                num_sand_particles += 1;
                break 'inner;
            }
        }
    }
    num_sand_particles
}

fn part_2(contents: &str) -> u64 {
    let mut rocks: Vec<Vec<Point>> = Vec::new();
    let mut filled_squares: HashSet<Point> = HashSet::new();
    for line in contents.lines() {
        let rock_line: Vec<Point> = line
            .replace(" ", "")
            .split("->")
            .map(|x| Point::from(x))
            .collect::<Vec<_>>();
        rocks.push(rock_line)
    }

    for line in rocks.iter() {
        for pair in line.windows(2) {
            let start = pair[0];
            let end = pair[1];
            if start.x == end.x && start.y < end.y {
                // vertical line
                for y in start.y..=end.y {
                    filled_squares.insert(Point { x: start.x, y });
                }
            } else if start.x == end.x && start.y > end.y {
                // vertical line
                for y in end.y..=start.y {
                    filled_squares.insert(Point { x: start.x, y });
                }
            } else if start.x < end.x {
                // horizontal line
                for x in start.x..=end.x {
                    filled_squares.insert(Point { x, y: start.y });
                }
            } else {
                // horizontal line
                for x in end.x..=start.x {
                    filled_squares.insert(Point { x, y: start.y });
                }
            }
        }
    }

    let max_x = rocks.iter().fold(0, |max, r| {
        let line_max = r.iter().map(|l| l.x).max().unwrap();
        if line_max > max {
            line_max
        } else {
            max
        }
    });
    let max_y = rocks.iter().fold(0, |max, r| {
        let line_max = r.iter().map(|l| l.y).max().unwrap();
        if line_max > max {
            line_max
        } else {
            max
        }
    });
    let min_x = rocks.iter().fold(u32::MAX, |min, r| {
        let line_min = r.iter().map(|l| l.x).min().unwrap();
        if line_min < min {
            line_min
        } else {
            min
        }
    });
    let min_y = rocks.iter().fold(u32::MAX, |min, r| {
        let line_min = r.iter().map(|l| l.y).min().unwrap();
        if line_min < min {
            line_min
        } else {
            min
        }
    });
    let floor_y = max_y + 2;

    let mut num_sand_particles: u64 = 0;
    'outer: loop {
        let mut sand = Point { x: 500, y: 0 };
        'inner: loop {
            let below = Point {
                x: sand.x,
                y: sand.y + 1,
            };
            let diag_left = Point {
                x: sand.x - 1,
                y: sand.y + 1,
            };
            let diag_right = Point {
                x: sand.x + 1,
                y: sand.y + 1,
            };
            if sand.y + 1 == floor_y {
                filled_squares.insert(sand);
                num_sand_particles += 1;
                break 'inner;
            } else if !filled_squares.contains(&below) {
                sand = below;
            } else if !filled_squares.contains(&diag_left) {
                sand = diag_left;
            } else if !filled_squares.contains(&diag_right) {
                sand = diag_right;
            } else if sand.x == 500 && sand.y == 0 {
                num_sand_particles += 1;
                break 'outer;
            } else {
                filled_squares.insert(sand);
                num_sand_particles += 1;
                break 'inner;
            }
        }
    }
    num_sand_particles
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 24);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 93);
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
