use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::time::Instant;

fn parse_orbits(contents: &str, bidirectional: bool) -> HashMap<&str, HashSet<&str>> {
    let mut orbits: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in contents.lines() {
        let orbit: Vec<&str> = line.split(")").collect();

        orbits
            .entry(orbit[0])
            .and_modify(|e| {
                e.insert(orbit[1]);
            })
            .or_insert(HashSet::from_iter(vec![orbit[1]]));

        if bidirectional {
            orbits
                .entry(orbit[1])
                .and_modify(|e| {
                    e.insert(orbit[0]);
                })
                .or_insert(HashSet::from_iter(vec![orbit[0]]));
        }
    }
    orbits
}

fn part_1(contents: &str) -> u32 {
    let orbits = parse_orbits(contents, false);
    let mut orbit_count_checksum = 0;
    let mut nodes: VecDeque<&str> = VecDeque::new();
    for key in orbits.keys() {
        nodes.push_back(key);
    }

    while nodes.len() > 0 {
        let node = nodes.pop_front().unwrap();
        match orbits.get(node) {
            Some(other_nodes) => other_nodes.iter().for_each(|n| {
                orbit_count_checksum += 1;
                nodes.push_back(n);
            }),
            None => {}
        };
    }

    orbit_count_checksum
}

fn part_2(contents: &str) -> u32 {
    let orbits = parse_orbits(contents, true);

    let mut min_distances: HashMap<&str, u32> = HashMap::new();
    min_distances.insert("YOU", 0);

    let mut nodes: BinaryHeap<(u32, &str)> = BinaryHeap::new();
    nodes.push((0, "YOU"));

    let target = "SAN";
    let minimum_distance_to_target = loop {
        let (distance, node) = nodes.pop().unwrap();
        match orbits.get(node) {
            Some(other_nodes) => {
                if other_nodes.contains(target) {
                    break distance - 1;
                }
                other_nodes.iter().for_each(|n| match min_distances.get(n) {
                    Some(min_distance) => {
                        if min_distance > &(distance + 1) {
                            min_distances.insert(n, distance + 1);
                            nodes.push((distance + 1, n))
                        }
                    }
                    None => {
                        min_distances.insert(n, distance + 1);
                        nodes.push((distance + 1, n))
                    }
                })
            }
            None => {}
        };
    };
    minimum_distance_to_target
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example_1.txt")), 42);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example_2.txt")), 4);
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
