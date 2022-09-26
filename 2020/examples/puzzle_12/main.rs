use rstest::*;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl From<&Direction> for i32 {
    fn from(direction: &Direction) -> i32 {
        match direction {
            Direction::NORTH => 0,
            Direction::EAST => 90,
            Direction::SOUTH => 180,
            Direction::WEST => 270,
        }
    }
}

impl From<i32> for Direction {
    fn from(degrees: i32) -> Direction {
        match degrees {
            0 => Direction::NORTH,
            90 => Direction::EAST,
            180 => Direction::SOUTH,
            270 => Direction::WEST,
            _ => panic!("Only multiples of 90 in [0, 360) are expected!"),
        }
    }
}

impl From<char> for Direction {
    fn from(direction: char) -> Direction {
        match direction {
            'N' => Direction::NORTH,
            'E' => Direction::EAST,
            'S' => Direction::SOUTH,
            'W' => Direction::WEST,
            _ => panic!("Only N, S, E and W are expected"),
        }
    }
}

impl Direction {
    fn rotate_clockwise(&self, degrees: i32) -> Direction {
        let current_degrees = i32::from(self);
        let degrees_after_rotation = (current_degrees + degrees) % 360;
        Direction::from(degrees_after_rotation)
    }

    fn rotate_anticlockwise(&self, degrees: i32) -> Direction {
        self.rotate_clockwise(360 - (degrees % 360))
    }
}

trait Transform {
    fn transform(&mut self, command: char, units: i32) -> ();
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
}

impl Transform for ShipLocation {
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

            'L' => self.direction = Direction::rotate_anticlockwise(&self.direction, units),
            'R' => self.direction = Direction::rotate_clockwise(&self.direction, units),
            'F' => match self.direction {
                Direction::NORTH => self.y += units,
                Direction::SOUTH => self.y -= units,
                Direction::EAST => self.x += units,
                Direction::WEST => self.x -= units,
            },
            _ => panic!("Only commands N, S, E, W, L, R and R are supported"),
        }
    }
}

impl Default for ShipLocation {
    fn default() -> Self {
        ShipLocation {
            x: 0,
            y: 0,
            direction: Direction::EAST,
        }
    }
}

struct Waypoint {
    relative_x: i32,
    relative_y: i32,
}

impl Waypoint {
    fn rotate_clockwise(&mut self, degrees: i32) {
        if degrees % 90 != 0 {
            panic!("Only rotations by multiples of 90 degrees are supported")
        }
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

impl Transform for Waypoint {
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
            _ => panic!("Only commands N, S, E and W are supported"),
        }
    }
}

impl Default for Waypoint {
    fn default() -> Self {
        Waypoint {
            relative_x: 10,
            relative_y: 1,
        }
    }
}

fn part_1(contents: &str) {
    let mut ship_location = ShipLocation::default();
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
    let mut ship_location = ShipLocation::default();
    let mut waypoint = Waypoint::default();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_transform() {
        let mut waypoint = Waypoint::default();
        waypoint.transform('F', 100);
    }

    #[test]
    #[should_panic]
    fn invalid_rotation() {
        let mut waypoint = Waypoint::default();
        waypoint.rotate_clockwise(35);
    }

    #[rstest]
    #[case('E', 90)]
    #[case('S', 180)]
    #[case('W', 0)]
    #[case('N', 270)]
    fn rotation_direction_and_reverse(#[case] direction_char: char, #[case] degrees: i32) {
        let direction: Direction = direction_char.into();
        assert_eq!(
            direction,
            direction
                .rotate_clockwise(degrees)
                .rotate_anticlockwise(degrees)
        );
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    part_1(contents);
    part_2(contents);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
