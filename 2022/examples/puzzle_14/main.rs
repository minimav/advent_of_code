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

struct Bounds {
    min_x: u32,
    max_x: u32,
    min_y: u32,
    max_y: u32,
}

impl From<&HashSet<Point>> for Bounds {
    fn from(filled_squares: &HashSet<Point>) -> Self {
        Bounds {
            max_x: filled_squares.iter().map(|p| p.x).max().unwrap(),
            min_x: filled_squares.iter().map(|p| p.x).min().unwrap(),
            max_y: filled_squares.iter().map(|p| p.y).max().unwrap(),
            min_y: filled_squares.iter().map(|p| p.y).min().unwrap(),
        }
    }
}

impl Bounds {
    fn check_out_of_bounds(&self, point: Point) -> bool {
        point.x > self.max_x || point.x < self.min_x || point.y >= self.max_y
    }
}

fn parse(contents: &str) -> HashSet<Point> {
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
                // vertical line, down
                for y in start.y..=end.y {
                    filled_squares.insert(Point { x: start.x, y });
                }
            } else if start.x == end.x && start.y > end.y {
                // vertical line, up
                for y in end.y..=start.y {
                    filled_squares.insert(Point { x: start.x, y });
                }
            } else if start.x < end.x {
                // horizontal line, right
                for x in start.x..=end.x {
                    filled_squares.insert(Point { x, y: start.y });
                }
            } else {
                // horizontal line, left
                for x in end.x..=start.x {
                    filled_squares.insert(Point { x, y: start.y });
                }
            }
        }
    }
    filled_squares
}

fn part_1(contents: &str) -> u64 {
    let mut filled_squares = parse(contents);
    let bounds = Bounds::from(&filled_squares);

    let mut num_sand_particles: u64 = 0;
    'outer: loop {
        let mut sand = Point { x: 500, y: 0 };
        'inner: loop {
            if bounds.check_out_of_bounds(sand) {
                break 'outer;
            }

            let below = Point {
                x: sand.x,
                y: sand.y + 1,
            };
            if !filled_squares.contains(&below) {
                sand = below;
                continue;
            }
            let diag_left = Point {
                x: sand.x - 1,
                y: sand.y + 1,
            };

            if !filled_squares.contains(&diag_left) {
                sand = diag_left;
                continue;
            }

            let diag_right = Point {
                x: sand.x + 1,
                y: sand.y + 1,
            };
            if !filled_squares.contains(&diag_right) {
                sand = diag_right;
                continue;
            }

            filled_squares.insert(sand);
            num_sand_particles += 1;
            break 'inner;
        }
    }
    num_sand_particles
}

fn part_2(contents: &str) -> u64 {
    let mut filled_squares = parse(contents);
    let bounds = Bounds::from(&filled_squares);
    let floor_y = bounds.max_y + 2;

    let mut num_sand_particles: u64 = 0;
    'outer: loop {
        let mut sand = Point { x: 500, y: 0 };
        'inner: loop {
            if sand.y + 1 == floor_y {
                filled_squares.insert(sand);
                num_sand_particles += 1;
                break 'inner;
            }

            let below = Point {
                x: sand.x,
                y: sand.y + 1,
            };
            if !filled_squares.contains(&below) {
                sand = below;
                continue;
            }

            let diag_left = Point {
                x: sand.x - 1,
                y: sand.y + 1,
            };
            if !filled_squares.contains(&diag_left) {
                sand = diag_left;
                continue;
            }

            let diag_right = Point {
                x: sand.x + 1,
                y: sand.y + 1,
            };
            if !filled_squares.contains(&diag_right) {
                sand = diag_right;
                continue;
            }

            if sand.x == 500 && sand.y == 0 {
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
