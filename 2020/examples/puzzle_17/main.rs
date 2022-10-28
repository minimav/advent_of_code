use itertools::iproduct;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
enum State {
    ON,
    OFF,
}

impl From<char> for State {
    fn from(c: char) -> State {
        match c {
            '#' => State::ON,
            '.' => State::OFF,
            _ => panic!("Only . and # expected"),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Default for Cube {
    fn default() -> Self {
        Cube { x: 0, y: 0, z: 0 }
    }
}

impl Cube {
    fn nearby(&self) -> [Cube; 26] {
        let mut cubes: [Cube; 26] = Default::default();
        let mut index: usize = 0;
        for (x_offset, y_offset, z_offset) in iproduct!(-1..=1, -1..=1, -1..=1) {
            if x_offset == 0 && y_offset == 0 && z_offset == 0 {
                continue;
            }
            cubes[index] = Cube {
                x: self.x + x_offset,
                y: self.y + y_offset,
                z: self.z + z_offset,
            };
            index += 1;
        }
        cubes
    }

    fn num_nearby_on(&self, on_cubes: &HashSet<Cube>) -> u64 {
        let mut on: u64 = 0;
        for cube in self.nearby().iter() {
            if on_cubes.contains(cube) {
                on += 1
            }
        }
        on
    }
}

fn part_1(contents: &str) -> u64 {
    let mut on_cubes: HashSet<Cube> = HashSet::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let cube = Cube {
                x: x as i32,
                y: y as i32,
                z: 0,
            };
            if State::from(c) == State::ON {
                on_cubes.insert(cube);
            }
        }
    }
    for iteration in 0..6 {
        let mut new_on_cubes: HashSet<Cube> = HashSet::new();
        let mut cubes_done: HashSet<Cube> = HashSet::new();
        for existing_cube in on_cubes.iter() {
            for cube in existing_cube.nearby().iter() {
                if cubes_done.contains(cube) {
                    continue;
                }
                let state = {
                    if on_cubes.contains(cube) {
                        State::ON
                    } else {
                        State::OFF
                    }
                };

                let num_nearby_on = cube.num_nearby_on(&on_cubes);
                let new_state: State = {
                    if state == State::ON && (num_nearby_on == 2 || num_nearby_on == 3) {
                        State::ON
                    } else if state == State::ON {
                        State::OFF
                    } else if num_nearby_on == 3 {
                        State::ON
                    } else {
                        State::OFF
                    }
                };
                if new_state == State::ON {
                    new_on_cubes.insert(*cube);
                }
                cubes_done.insert(*cube);
            }
        }
        on_cubes = new_on_cubes;
    }
    on_cubes.len() as u64
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct HyperCube {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

impl Default for HyperCube {
    fn default() -> Self {
        HyperCube {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

impl HyperCube {
    fn nearby(&self) -> [HyperCube; 80] {
        let mut cubes: [HyperCube; 80] = [HyperCube::default(); 80];
        let mut index: usize = 0;
        for (w_offset, x_offset, y_offset, z_offset) in iproduct!(-1..=1, -1..=1, -1..=1, -1..=1) {
            if w_offset == 0 && x_offset == 0 && y_offset == 0 && z_offset == 0 {
                continue;
            }
            cubes[index] = HyperCube {
                w: self.w + w_offset,
                x: self.x + x_offset,
                y: self.y + y_offset,
                z: self.z + z_offset,
            };
            index += 1;
        }
        cubes
    }

    fn num_nearby_on(&self, on_cubes: &HashSet<HyperCube>) -> u64 {
        let mut on: u64 = 0;
        for cube in self.nearby().iter() {
            if on_cubes.contains(cube) {
                on += 1
            }
        }
        on
    }
}

fn part_2(contents: &str) -> u64 {
    let mut on_cubes: HashSet<HyperCube> = HashSet::new();
    for (x, line) in contents.lines().enumerate() {
        for (w, c) in line.chars().enumerate() {
            let cube = HyperCube {
                w: w as i32,
                x: x as i32,
                y: 0,
                z: 0,
            };
            if State::from(c) == State::ON {
                on_cubes.insert(cube);
            }
        }
    }
    for iteration in 0..6 {
        let mut new_on_cubes: HashSet<HyperCube> = HashSet::new();
        let mut cubes_done: HashSet<HyperCube> = HashSet::new();
        for existing_cube in on_cubes.iter() {
            for cube in existing_cube.nearby().iter() {
                if cubes_done.contains(cube) {
                    continue;
                }
                let state = {
                    if on_cubes.contains(cube) {
                        State::ON
                    } else {
                        State::OFF
                    }
                };

                let num_nearby_on = cube.num_nearby_on(&on_cubes);
                let new_state: State = {
                    if state == State::ON && (num_nearby_on == 2 || num_nearby_on == 3) {
                        State::ON
                    } else if state == State::ON {
                        State::OFF
                    } else if num_nearby_on == 3 {
                        State::ON
                    } else {
                        State::OFF
                    }
                };
                if new_state == State::ON {
                    new_on_cubes.insert(*cube);
                }
                cubes_done.insert(*cube);
            }
        }
        on_cubes = new_on_cubes;
    }
    on_cubes.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 112);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 848);
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
