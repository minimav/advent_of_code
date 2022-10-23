use std::time::Instant;

fn parse_ticket(line: &str) -> Vec<u64> {
    line.split(",").map(|x| x.parse::<u64>().unwrap()).collect()
}

fn parse_your_tickets(contents: &str) -> Vec<u64> {
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
        // let name = initial_components[0];
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
    let your_ticket = parse_your_tickets(contents);
    let nearby_tickets = parse_nearby_tickets(contents);
    let conditions = parse_conditions(contents);

    let mut answer: u64 = 0;
    for ticket in nearby_tickets.iter() {
        if ticket.iter().any(|x| !passes_a_condition(&conditions, *x)) {
            continue;
        }
        println!("valid ticket={:?}", ticket);
    }

    /* make a matrix
          tickets--->
          third dimension ticket slice where all 1s?
    conditions
    |
    |
    v
    departure is first 6 fields
    */
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
    let contents = include_str!("./example_2.txt");
    let part_1_answer = part_1(contents);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer = part_2(contents);
    println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
