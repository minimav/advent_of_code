use std::time::Instant;
use std::collections::{HashMap, HashSet, VecDeque};

fn part_1(contents: &str) -> u32 {
    let start = "you";
    let end = "out";

    let lines: Vec<&str> = contents.lines().collect();
    let mut string_to_index: HashMap<&str, usize> = HashMap::new();
    let mut connections: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        let node = line.split(":").collect::<Vec<_>>()[0];
        string_to_index.insert(node, i);
    }
    string_to_index.insert(end, lines.len());

    for line in lines.iter() {
        let comps = line.split(": ").collect::<Vec<_>>();
        let node = string_to_index.get(comps[0]).unwrap();
        let to = comps[1].split(" ").map(|v| *string_to_index.get(v).unwrap()).collect::<Vec<_>>();
        connections.insert(*node, to);
    }
    
    let start_index = string_to_index.get(start).unwrap();
    let end_index = string_to_index.get(end).unwrap();
    let mut answer = 0;
    let mut queue = VecDeque::new();
    queue.push_front((start_index, HashSet::from([start_index]), 1));
    while queue.len() > 0 {
        let (index, seen, num_paths) = queue.pop_front().unwrap();
        let to_nodes = connections.get(index);
        match to_nodes {
            None => answer += num_paths,
            Some(nodes) => {
                for other_index in nodes {
                    let mut new_seen = seen.clone();
                    new_seen.insert(other_index);
                    queue.push_back((other_index, new_seen, num_paths));
                }
            }
        }
    }
    return answer;
}

fn part_2(contents: &str) -> u64 {
    let start = "svr";
    let end = "out";

    let lines: Vec<&str> = contents.lines().collect();
    let mut string_to_index: HashMap<&str, usize> = HashMap::new();
    let mut connections: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        let node = line.split(":").collect::<Vec<_>>()[0];
        string_to_index.insert(node, i);
    }
    string_to_index.insert(end, lines.len());

    for line in lines.iter() {
        let comps = line.split(": ").collect::<Vec<_>>();
        let node = string_to_index.get(comps[0]).unwrap();
        let to = comps[1].split(" ").map(|v| *string_to_index.get(v).unwrap()).collect::<Vec<_>>();
        connections.insert(*node, to);
    }
    
    let start_index = string_to_index.get(start).unwrap();
    let dac_index = string_to_index.get("dac").unwrap();
    let fft_index = string_to_index.get("fft").unwrap();
    let mut answer = 0;
    let mut current: HashMap<(usize, (bool, bool)), u64> = HashMap::from([
        ((*start_index, (false, false)), 1)
    ]);
    while current.len() > 0 {
        let mut next: HashMap<(usize, (bool, bool)), u64> = HashMap::new();
        for ((index, (seen_dac, seen_fft)), num_paths) in current.iter() {
            let to_nodes = connections.get(index);
            match to_nodes {
                None => {
                    if *seen_dac && *seen_fft {
                        answer += num_paths;
                    }
                },
                Some(nodes) => {
                    for other_index in nodes {
                        let new_seen = {
                            if other_index == dac_index {
                                (true, *seen_fft)
                            } else if other_index == fft_index {
                                (*seen_dac, true)
                            } else {
                                (*seen_dac, *seen_fft)
                            }
                        };
                        next.entry((*other_index, new_seen))
                        .and_modify(|v| *v += *num_paths)
                        .or_insert(*num_paths);
                    }
                }
            }
        }
        current = next;
    }
    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example_1.txt")), 5);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example_2.txt")), 2);
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
    println!("Took {:?} to solve puzzle", duration);
}
