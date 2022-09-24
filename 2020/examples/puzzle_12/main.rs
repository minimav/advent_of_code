use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Direction {
    fn degrees(&self) -> i32 {
        match *self {
            Direction::NORTH => 0,
            Direction::EAST => 90,
            Direction::SOUTH => 180,
            Direction::WEST => 270,
        }
    }

    fn direction_from_degrees(degrees: i32) -> Option<Direction> {
        match degrees {
            0 => Some(Direction::NORTH),
            90 => Some(Direction::EAST),
            180 => Some(Direction::SOUTH),
            270 => Some(Direction::WEST),
            _ => None,
        }
    }

    fn rotate_clockwise(current_direction: Direction, degrees: i32) -> Direction {
        let current_degrees = current_direction.degrees();
        let degrees_after_rotation = (current_degrees + degrees) % 360;
        Direction::direction_from_degrees(degrees_after_rotation).unwrap()
    }

    fn rotate_anticlockwise(current_direction: Direction, degrees: i32) -> Direction {
        Direction::rotate_clockwise(current_direction, 360 - (degrees % 360))
    }
}

struct Location {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Location {
    fn transform(&mut self, line: &str) {
        let command = line.chars().next().unwrap();
        let number = line[1..].parse::<i32>().unwrap();

        match command {
            'N' => {
                self.y += number;
            }
            'S' => {
                self.y -= number;
            }
            'E' => {
                self.x += number;
            }
            'W' => {
                self.x -= number;
            }

            'L' => self.direction = Direction::rotate_anticlockwise(self.direction, number),
            'R' => self.direction = Direction::rotate_clockwise(self.direction, number),
            'F' => match self.direction {
                Direction::NORTH => self.y += number,
                Direction::SOUTH => self.y -= number,
                Direction::EAST => self.x += number,
                Direction::WEST => self.x -= number,
                _ => {}
            },
            _ => {}
        }
    }
}

fn part_1(contents: &str) {
    let mut location = Location {
        x: 0,
        y: 0,
        direction: Direction::EAST,
    };
    for command in contents.lines() {
        location.transform(command);
    }
    println!(
        "Answer for part 1 is: {}",
        location.x.abs() + location.y.abs()
    );
}

fn part_2(contents: &str) {
    println!("Answer for part 2 is: {}", 0);
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    part_1(contents);
    part_2(contents);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
