use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn parse_ticket(line: &str) -> Vec<u64> {
    line.split(",").map(|x| x.parse::<u64>().unwrap()).collect()
}

fn parse_your_ticket(contents: &str) -> Vec<u64> {
    let marker = "your ticket:";
    for line in contents.lines().skip_while(|x| x != &marker) {
        if line == marker {
            continue;
        }
        return parse_ticket(line);
    }
    panic!("Should not get here!")
}

fn parse_nearby_tickets(contents: &str) -> Vec<Vec<u64>> {
    let mut tickets: Vec<Vec<u64>> = Vec::new();
    let marker = "nearby tickets:";
    for line in contents.lines().skip_while(|x| x != &marker) {
        if line == marker {
            continue;
        }
        tickets.push(parse_ticket(line));
    }
    tickets
}

#[derive(Debug)]
struct Range {
    min: u64,
    max: u64,
}

fn parse_conditions(contents: &str) -> Vec<Vec<Range>> {
    let mut conditions: Vec<Vec<Range>> = Vec::new();
    for line in contents.lines().take_while(|x| x.len() > 0) {
        let mut condition: Vec<Range> = Vec::new();
        let initial_components: Vec<&str> = line.split(": ").collect();
        for component in initial_components[1].split(" ") {
            if component.contains(":") || component == "or" {
                continue;
            }
            let raw_range: Vec<&str> = component.split("-").collect();
            let range = Range {
                min: raw_range[0].parse::<u64>().unwrap(),
                max: raw_range[1].parse::<u64>().unwrap(),
            };
            condition.push(range);
        }
        conditions.push(condition);
    }
    conditions
}

fn passes_a_condition(conditions: &Vec<Vec<Range>>, value: u64) -> bool {
    for condition in conditions.iter() {
        for range in condition.iter() {
            if (range.min <= value) && (value <= range.max) {
                return true;
            }
        }
    }
    false
}

fn part_1(contents: &str) -> u64 {
    let nearby_tickets = parse_nearby_tickets(contents);
    let conditions = parse_conditions(contents);

    let mut answer: u64 = 0;
    for ticket in nearby_tickets.iter() {
        for value in ticket.iter() {
            if !passes_a_condition(&conditions, *value) {
                answer += value
            }
        }
    }
    answer
}

fn part_2(contents: &str) -> u64 {
    let your_ticket = parse_your_ticket(contents);
    let nearby_tickets = parse_nearby_tickets(contents);
    let conditions = parse_conditions(contents);

    // get all valid tickets that we'll use to check mappings
    let mut valid_tickets: Vec<&Vec<u64>> = vec![&your_ticket];
    for ticket in nearby_tickets.iter() {
        if ticket.iter().any(|x| !passes_a_condition(&conditions, *x)) {
            continue;
        }
        valid_tickets.push(ticket);
    }

    // setup data structure to record valid mapping from
    // condition index => set of valid ticket positions
    let mut possibles: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (condition_index, _) in conditions.iter().enumerate() {
        let mut all_values: HashSet<usize> = HashSet::new();
        for ticket_value_index in 0..your_ticket.len() {
            all_values.insert(ticket_value_index);
        }
        possibles.insert(condition_index, all_values);
    }

    // remove possible mappings if a position in a ticket cannot correspond
    // to a condition
    for ticket in valid_tickets.iter() {
        for (value_index, &value) in ticket.iter().enumerate() {
            for (condition_index, condition) in conditions.iter().enumerate() {
                let mut possible_mapping = false;
                for range in condition.iter() {
                    if (range.min <= value) && (value <= range.max) {
                        possible_mapping = true;
                    }
                }
                if !possible_mapping {
                    possibles.entry(condition_index).and_modify(|e| {
                        e.remove(&value_index);
                    });
                }
            }
        }
    }

    // clean up iteratively based on mappings which must be true
    let mut mapped: HashSet<usize> = HashSet::new();
    while mapped.len() < conditions.len() {
        // pick a singleton
        let mut next_index: usize = 0;
        let mut next_position: usize = 0;
        for (condition_index, positions) in possibles.iter() {
            if !mapped.contains(&condition_index) && positions.len() == 1 {
                next_index = *condition_index;
                next_position = *positions.iter().next().unwrap();
                break;
            }
        }
        mapped.insert(next_index);

        // remove this index from all other sets
        for condition_index in 0..conditions.len() {
            possibles.entry(condition_index).and_modify(|e| {
                if condition_index != next_index && e.contains(&next_position) {
                    e.remove(&next_position);
                }
            });
        }
    }

    // compute the answer via looking up only valid mapping per condition
    // on your ticket
    let mut answer: u64 = 1;
    for condition_index in 0..6 {
        match possibles.get(&condition_index) {
            Some(positions) => {
                let position_index = positions.iter().next().unwrap();
                answer *= your_ticket[*position_index];
            }
            None => panic!("Each condition should have a single position!"),
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example_1.txt")), 71);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example_2.txt")), 11 * 12 * 13);
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
