#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet};
use std::time::Instant;

lazy_static! {
    static ref CHAR_ELEVATIONS: HashMap<char, usize> = {
        let mut m = HashMap::new();
        let alphanumeric = "abcdefghijklmnopqrstuvwxyz";
        for (index, char) in alphanumeric.chars().enumerate() {
            m.insert(char, index);
        }
        m.insert('S', 0);
        m.insert('E', 25);
        m
    };
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct TraversalState {
    location: Location,
    num_steps: u64,
}

impl Default for Location {
    fn default() -> Self {
        Location { x: 0, y: 0 }
    }
}

struct ElevationsData<'a> {
    elevations: HashMap<Location, &'a usize>,
    start_location: Location,
    end_location: Location,
}

fn parse(contents: &str) -> ElevationsData {
    let mut elevations: HashMap<Location, &usize> = HashMap::new();
    let mut start_location = Location::default();
    let mut end_location = Location::default();
    for (row_index, line) in contents.lines().enumerate() {
        for (column_index, char) in line.chars().enumerate() {
            let location = Location {
                x: column_index,
                y: row_index,
            };
            if char == 'S' {
                start_location = location.clone();
            } else if char == 'E' {
                end_location = location.clone();
            }
            elevations.insert(location, CHAR_ELEVATIONS.get(&char).unwrap());
        }
    }
    ElevationsData {
        elevations,
        start_location,
        end_location,
    }
}

fn add_move_state(
    new_state: TraversalState,
    elevations_data: &ElevationsData,
    current_elevation: &usize,
    next_current_states: &mut HashSet<TraversalState>,
) {
    let new_elevation = elevations_data.elevations.get(&new_state.location).unwrap();
    if **new_elevation <= *current_elevation + 1 {
        next_current_states.insert(new_state);
    }
}

fn shortest_path(elevations_data: ElevationsData, start_locations: Vec<Location>) -> u64 {
    let mut fewest_steps = u64::MAX;

    let max_x_index = elevations_data
        .elevations
        .keys()
        .map(|l| l.x)
        .max()
        .unwrap();
    let max_y_index = elevations_data
        .elevations
        .keys()
        .map(|l| l.y)
        .max()
        .unwrap();

    for start_location in start_locations.iter() {
        let mut current_states: HashSet<TraversalState> = HashSet::from_iter([TraversalState {
            location: *start_location,
            num_steps: 0,
        }]);
        let mut fastest_steps: HashMap<Location, u64> = HashMap::new();
        loop {
            let mut next_current_states: HashSet<TraversalState> = HashSet::new();
            for state in current_states.iter() {
                if &state.num_steps > fastest_steps.entry(state.location).or_insert(u64::MAX) {
                    continue;
                }
                fastest_steps
                    .entry(state.location)
                    .and_modify(|e| *e = state.num_steps)
                    .or_insert(state.num_steps);

                let current_elevation = elevations_data.elevations.get(&state.location).unwrap();

                // left
                if state.location.x > 0 {
                    let left = TraversalState {
                        location: Location {
                            x: state.location.x - 1,
                            y: state.location.y,
                        },
                        num_steps: state.num_steps + 1,
                    };
                    add_move_state(
                        left,
                        &elevations_data,
                        current_elevation,
                        &mut next_current_states,
                    );
                }

                // right
                if state.location.x < max_x_index {
                    let right = TraversalState {
                        location: Location {
                            x: state.location.x + 1,
                            y: state.location.y,
                        },
                        num_steps: state.num_steps + 1,
                    };
                    add_move_state(
                        right,
                        &elevations_data,
                        current_elevation,
                        &mut next_current_states,
                    );
                }

                // up
                if state.location.y > 0 {
                    let up = TraversalState {
                        location: Location {
                            x: state.location.x,
                            y: state.location.y - 1,
                        },
                        num_steps: state.num_steps + 1,
                    };
                    add_move_state(
                        up,
                        &elevations_data,
                        current_elevation,
                        &mut next_current_states,
                    );
                }

                // down
                if state.location.y < max_y_index {
                    let down = TraversalState {
                        location: Location {
                            x: state.location.x,
                            y: state.location.y + 1,
                        },
                        num_steps: state.num_steps + 1,
                    };
                    add_move_state(
                        down,
                        &elevations_data,
                        current_elevation,
                        &mut next_current_states,
                    );
                }
            }

            if next_current_states.len() == 0 {
                break;
            }

            current_states = next_current_states;
        }
        // get quickest route from current start point
        // update best overall if it is the quickest
        match fastest_steps.get(&elevations_data.end_location) {
            Some(steps) => {
                if steps < &fewest_steps {
                    fewest_steps = *steps;
                }
            }
            None => {}
        };
    }
    fewest_steps
}

fn part_1(contents: &str) -> u64 {
    let elevations_data = parse(contents);
    let start_locations: Vec<Location> = vec![elevations_data.start_location];
    shortest_path(elevations_data, start_locations)
}

fn part_2(contents: &str) -> u64 {
    let elevations_data = parse(contents);
    let start_locations: Vec<Location> = elevations_data
        .elevations
        .iter()
        .filter(|(_, v)| ***v == 0)
        .map(|(k, _)| k.clone())
        .collect::<Vec<Location>>();
    shortest_path(elevations_data, start_locations)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 31);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 29);
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
