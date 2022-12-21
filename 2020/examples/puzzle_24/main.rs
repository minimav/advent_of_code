use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::time::Instant;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct HexCoord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Colour {
    WHITE,
    BLACK,
}

enum HexDirection {
    EAST,
    SOUTH_EAST,
    SOUTH_WEST,
    WEST,
    NORTH_WEST,
    NORTH_EAST,
}

impl From<&str> for HexDirection {
    fn from(s: &str) -> Self {
        match s {
            "se" => HexDirection::SOUTH_EAST,
            "sw" => HexDirection::SOUTH_WEST,
            "e" => HexDirection::EAST,
            "w" => HexDirection::WEST,
            "ne" => HexDirection::NORTH_EAST,
            "nw" => HexDirection::NORTH_WEST,
            _ => panic!("Unexpected direction to parse!"),
        }
    }
}

impl HexDirection {
    fn parse_line(line: &str) -> Vec<Self> {
        let mut directions: Vec<Self> = Vec::new();

        let mut chars = line.chars().into_iter().peekable();
        loop {
            match chars.peek() {
                Some('e') => directions.push(HexDirection::EAST),
                Some('w') => directions.push(HexDirection::WEST),
                Some('s') => {
                    chars.next();
                    match chars.peek() {
                        Some('e') => directions.push(HexDirection::SOUTH_EAST),
                        Some('w') => directions.push(HexDirection::SOUTH_WEST),
                        _ => panic!("Unexpected input!"),
                    }
                }
                Some('n') => {
                    chars.next();
                    match chars.peek() {
                        Some('e') => directions.push(HexDirection::NORTH_EAST),
                        Some('w') => directions.push(HexDirection::NORTH_WEST),
                        _ => panic!("Unexpected input!"),
                    }
                }
                _ => break,
            }
            chars.next();
        }
        directions
    }
}

impl Default for HexCoord {
    fn default() -> Self {
        HexCoord { x: 0, y: 0 }
    }
}

impl From<HexDirection> for HexCoord {
    fn from(d: HexDirection) -> Self {
        match d {
            HexDirection::EAST => HexCoord { x: 2, y: 0 },
            HexDirection::WEST => HexCoord { x: -2, y: 0 },
            HexDirection::NORTH_WEST => HexCoord { x: -1, y: 2 },
            HexDirection::NORTH_EAST => HexCoord { x: 1, y: 2 },
            HexDirection::SOUTH_EAST => HexCoord { x: 1, y: -2 },
            HexDirection::SOUTH_WEST => HexCoord { x: -1, y: -2 },
        }
    }
}

impl HexCoord {
    fn shift(&mut self, d: HexDirection) {
        let shift_coords = HexCoord::from(d);
        self.x += shift_coords.x;
        self.y += shift_coords.y;
    }

    fn traverse(&mut self, directions: Vec<HexDirection>) {
        for d in directions {
            self.shift(d);
        }
    }

    fn neighbours(&self) -> Vec<HexCoord> {
        let mut east = self.clone();
        east.shift(HexDirection::EAST);
        let mut west = self.clone();
        west.shift(HexDirection::WEST);
        let mut south_west = self.clone();
        south_west.shift(HexDirection::SOUTH_WEST);
        let mut north_west = self.clone();
        north_west.shift(HexDirection::NORTH_WEST);
        let mut south_east = self.clone();
        south_east.shift(HexDirection::SOUTH_EAST);
        let mut north_east = self.clone();
        north_east.shift(HexDirection::NORTH_EAST);
        vec![
            self.clone(),
            east,
            west,
            north_east,
            south_east,
            north_west,
            south_west,
        ]
    }

    fn update(&self, statuses: &HashMap<HexCoord, Colour>) -> Colour {
        let mut black_count = 0;
        for neighbour in self.neighbours().iter() {
            if neighbour == self {
                continue;
            }

            match statuses.get(&neighbour) {
                Some(Colour::BLACK) => black_count += 1,
                _ => (),
            }
        }
        // Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
        // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
        match statuses.get(self) {
            Some(Colour::BLACK) if black_count == 0 || black_count > 2 => Colour::WHITE,
            Some(Colour::BLACK) => Colour::BLACK,
            _ if black_count == 2 => Colour::BLACK,
            _ => Colour::WHITE,
        }
    }
}

fn part_1(contents: &str) -> usize {
    let mut counts: HashMap<HexCoord, u8> = HashMap::new();
    for line in contents.lines() {
        let directions = HexDirection::parse_line(line);
        let mut point = HexCoord::default();
        point.traverse(directions);
        counts.entry(point).and_modify(|x| *x += 1).or_insert(1);
    }

    counts
        .iter()
        .map(|(_, v)| *v)
        .filter(|v| v % 2 == 1)
        .collect::<Vec<u8>>()
        .len()
}

fn count_blacks(statuses: &HashMap<HexCoord, Colour>) -> usize {
    statuses
        .iter()
        .map(|(_, v)| v)
        .filter(|v| *v == &Colour::BLACK)
        .collect::<Vec<&Colour>>()
        .len()
}

fn part_2(contents: &str) -> usize {
    let mut counts: HashMap<HexCoord, u8> = HashMap::new();
    for line in contents.lines() {
        let directions = HexDirection::parse_line(line);
        let mut point = HexCoord::default();
        point.traverse(directions);
        counts.entry(point).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut statuses: HashMap<HexCoord, Colour> = HashMap::new();
    for (coord, count) in counts {
        let colour = if count % 2 == 1 {
            Colour::BLACK
        } else {
            Colour::WHITE
        };
        statuses.insert(coord, colour);
    }

    for iteration in 0..100 {
        let mut new_statuses: HashMap<HexCoord, Colour> = HashMap::new();
        let mut coords_to_check: HashSet<HexCoord> = HashSet::new();
        for (coord, _) in statuses.iter() {
            for neighbour in coord.neighbours() {
                coords_to_check.insert(neighbour);
            }
        }
        for coord in coords_to_check {
            match coord.update(&statuses) {
                Colour::BLACK => {
                    new_statuses.insert(coord, Colour::BLACK);
                }
                _ => (),
            };
        }

        statuses = new_statuses;
        let black_count = count_blacks(&statuses);
        println!(
            "After {} days there were {black_count} black tiles",
            iteration + 1
        );
    }
    count_blacks(&statuses)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 10);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 2208);
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
