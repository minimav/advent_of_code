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

struct ShipLocation {
    x: i32,
    y: i32,
    direction: Direction,
}

impl ShipLocation {
    fn manhattan_distince(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
    fn transform(&mut self, command: char, units: i32) {
        match command {
            'N' => {
                self.y += units;
            }
            'S' => {
                self.y -= units;
            }
            'E' => {
                self.x += units;
            }
            'W' => {
                self.x -= units;
            }

            'L' => self.direction = Direction::rotate_anticlockwise(self.direction, units),
            'R' => self.direction = Direction::rotate_clockwise(self.direction, units),
            'F' => match self.direction {
                Direction::NORTH => self.y += units,
                Direction::SOUTH => self.y -= units,
                Direction::EAST => self.x += units,
                Direction::WEST => self.x -= units,
            },
            _ => {}
        }
    }
}

struct Waypoint {
    relative_x: i32,
    relative_y: i32,
}

impl Waypoint {
    fn transform(&mut self, command: char, units: i32) {
        match command {
            'N' => {
                self.relative_y += units;
            }
            'S' => {
                self.relative_y -= units;
            }
            'E' => {
                self.relative_x += units;
            }
            'W' => {
                self.relative_x -= units;
            }
            _ => {}
        }
    }
    fn rotate_clockwise(&mut self, degrees: i32) {
        let radians = (degrees as f64).to_radians();
        // we can do this because we know that we will only be rotating by multiples of 90
        // degrees, therefore these are always +/-1
        let sine_theta = radians.sin() as i32;
        let cosine_theta = radians.cos() as i32;
        let new_relative_x = self.relative_x * cosine_theta - self.relative_y * sine_theta;
        let new_relative_y = self.relative_x * sine_theta + self.relative_y * cosine_theta;
        self.relative_x = new_relative_x;
        self.relative_y = new_relative_y;
    }
    fn rotate_anticlockwise(&mut self, degrees: i32) {
        self.rotate_clockwise(360 - (degrees % 360));
    }
}

fn part_1(contents: &str) {
    let mut ship_location = ShipLocation {
        x: 0,
        y: 0,
        direction: Direction::EAST,
    };
    for line in contents.lines() {
        let command = line.chars().next().unwrap();
        let units = line[1..].parse::<i32>().unwrap();
        ship_location.transform(command, units);
    }
    println!(
        "Answer for part 1 is: {}",
        ship_location.manhattan_distince()
    );
}

fn part_2(contents: &str) {
    let mut ship_location = ShipLocation {
        x: 0,
        y: 0,
        direction: Direction::EAST,
    };
    let mut waypoint = Waypoint {
        relative_x: 10,
        relative_y: 1,
    };
    for line in contents.lines() {
        let command = line.chars().next().unwrap();
        let units = line[1..].parse::<i32>().unwrap();

        if String::from("NSEW").contains(command) {
            waypoint.transform(command, units)
        } else if command == 'F' {
            ship_location.x += units * waypoint.relative_x;
            ship_location.y += units * waypoint.relative_y;
        } else if command == 'L' {
            waypoint.rotate_clockwise(units)
        } else {
            waypoint.rotate_anticlockwise(units)
        }
    }
    println!(
        "Answer for part 1 is: {}",
        ship_location.manhattan_distince()
    );
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    part_1(contents);
    part_2(contents);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
