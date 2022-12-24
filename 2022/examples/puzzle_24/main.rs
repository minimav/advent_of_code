use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum WindDirection {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl From<char> for WindDirection {
    fn from(c: char) -> Self {
        match c {
            'v' => WindDirection::DOWN,
            '^' => WindDirection::UP,
            '>' => WindDirection::RIGHT,
            '<' => WindDirection::LEFT,
            _ => panic!("Wind doesn't go in this direction!"),
        }
    }
}

impl Into<char> for &WindDirection {
    fn into(self) -> char {
        match self {
            WindDirection::DOWN => 'v',
            WindDirection::UP => '^',
            WindDirection::RIGHT => '>',
            WindDirection::LEFT => '<',
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn neighbours(&self) -> Vec<Self> {
        let mut points: Vec<Point> =
            vec![Self::new(self.x + 1, self.y), Self::new(self.x, self.y + 1)];
        if self.x > 0 {
            points.push(Self::new(self.x - 1, self.y));
        }
        if self.y > 0 {
            points.push(Self::new(self.x, self.y - 1));
        }
        points
    }
}

fn print_blizzard(blizzard: &Blizzard, start: &Point, end: &Point, width: &usize, height: &usize) {
    let mut output = vec![vec!['.'; *width]; *height];
    for (blizzard, direction) in blizzard.blizzards.iter() {
        output[blizzard.y][blizzard.x] = direction.into()
    }
    // borders
    output[0] = vec!['#'; *width];
    output[*height - 1] = vec!['#'; *width];

    // start and end
    output[start.y][start.x] = '.';
    output[end.y][end.x] = '.';

    for mut row in output {
        row[0] = '#';
        row[*width - 1] = '#';
        for char in row {
            print!("{char}");
        }
        println!("")
    }
}

#[derive(Debug, Clone)]
struct Blizzard {
    blizzards: Vec<(Point, WindDirection)>,
    occupied: HashSet<Point>,
}

impl From<Vec<(Point, WindDirection)>> for Blizzard {
    fn from(blizzards: Vec<(Point, WindDirection)>) -> Self {
        let mut occupied: HashSet<Point> = HashSet::new();
        for (point, _) in blizzards.iter() {
            occupied.insert(*point);
        }
        Blizzard {
            blizzards,
            occupied,
        }
    }
}

fn update_blizzard(blizzard: Blizzard, grid_width: &usize, grid_height: &usize) -> Blizzard {
    let mut new_blizzards: Vec<(Point, WindDirection)> = Vec::new();
    for (point, direction) in blizzard.blizzards.into_iter() {
        match direction {
            WindDirection::LEFT => {
                if point.x == 1 {
                    new_blizzards.push((Point::new(grid_width - 2, point.y), direction));
                } else {
                    new_blizzards.push((Point::new(point.x - 1, point.y), direction));
                }
            }
            WindDirection::RIGHT => {
                if point.x == grid_width - 2 {
                    new_blizzards.push((Point::new(1, point.y), direction));
                } else {
                    new_blizzards.push((Point::new(point.x + 1, point.y), direction));
                }
            }
            WindDirection::UP => {
                if point.y == 1 {
                    new_blizzards.push((Point::new(point.x, grid_height - 2), direction));
                } else {
                    new_blizzards.push((Point::new(point.x, point.y - 1), direction));
                }
            }
            WindDirection::DOWN => {
                if point.y == grid_height - 2 {
                    new_blizzards.push((Point::new(point.x, 1), direction));
                } else {
                    new_blizzards.push((Point::new(point.x, point.y + 1), direction));
                }
            }
        }
    }
    Blizzard::from(new_blizzards)
}

fn parse_blizzard(contents: &str) -> (Blizzard, usize, usize) {
    let mut blizzards: Vec<(Point, WindDirection)> = Vec::new();
    let grid_width = contents.lines().next().unwrap().len();
    let mut grid_height = 0;
    for (row_index, line) in contents.lines().enumerate() {
        grid_height += 1;
        for (column_index, char) in line.chars().enumerate() {
            if char == '#' || char == '.' {
                continue;
            }
            let direction = WindDirection::from(char);
            blizzards.push((Point::new(column_index, row_index), direction));
        }
    }
    let blizzard = Blizzard::from(blizzards);
    (blizzard, grid_width, grid_height)
}

fn shortest_path(
    start_blizzard: Blizzard,
    start: Point,
    end: Point,
    grid_width: &usize,
    grid_height: &usize,
) -> (Blizzard, usize) {
    let mut blizzard = start_blizzard.clone();
    let mut positions: HashSet<Point> = HashSet::from_iter(vec![start.clone()]);
    let mut num_steps = 0;
    'outer: while positions.len() > 0 {
        blizzard = update_blizzard(blizzard, grid_width, grid_height);
        let mut new_positions: HashSet<Point> = HashSet::new();
        for position in positions {
            if !blizzard.occupied.contains(&position) {
                new_positions.insert(position);
            }
            for neighbour in position.neighbours().into_iter() {
                if neighbour == end {
                    num_steps += 1;
                    break 'outer;
                } else if !blizzard.occupied.contains(&neighbour)
                    && neighbour.x > 0
                    && neighbour.x < grid_width - 1
                    && neighbour.y > 0
                    && neighbour.y < grid_height - 1
                {
                    new_positions.insert(neighbour);
                }
            }
        }
        positions = new_positions;
        num_steps += 1;
    }
    (blizzard, num_steps)
}

fn part_1(contents: &str) -> usize {
    let (blizzard, grid_width, grid_height) = parse_blizzard(contents);
    let start = Point::new(1, 0);
    let end = Point::new(grid_width - 2, grid_height - 1);
    let (_, num_steps) = shortest_path(blizzard, start, end, &grid_width, &grid_height);
    num_steps
}

fn part_2(contents: &str) -> usize {
    let (blizzard, grid_width, grid_height) = parse_blizzard(contents);

    let mut answer = 0;
    let start = Point::new(1, 0);
    let end = Point::new(grid_width - 2, grid_height - 1);

    // leg 1
    let (blizzard, num_steps) = shortest_path(
        blizzard,
        start.clone(),
        end.clone(),
        &grid_width,
        &grid_height,
    );
    answer += num_steps;

    // leg 2
    let (blizzard, num_steps) = shortest_path(
        blizzard,
        end.clone(),
        start.clone(),
        &grid_width,
        &grid_height,
    );
    answer += num_steps;

    // leg 3
    let (blizzard, num_steps) = shortest_path(
        blizzard,
        start.clone(),
        end.clone(),
        &grid_width,
        &grid_height,
    );
    answer += num_steps;
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_minimal_example() {
        assert_eq!(part_1(include_str!("./minimal_example.txt")), 10);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 18);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 54);
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
