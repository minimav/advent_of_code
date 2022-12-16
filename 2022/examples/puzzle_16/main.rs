use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u64,
    tunnels: Vec<String>,
}

#[derive(Debug)]
struct Route {
    current: String,
    opened: HashSet<String>,
    flow_rate: u64,
}

impl Default for Route {
    fn default() -> Self {
        Route {
            current: String::from("AA"),
            opened: HashSet::new(),
            flow_rate: 0,
        }
    }
}

fn parse_valves(contents: &str) -> HashMap<String, Valve> {
    let mut valves: HashMap<String, Valve> = HashMap::new();
    for line in contents.lines() {
        let re = Regex::new(r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels lead to valves (.+)")
            .unwrap();
        let captures = re.captures(line).unwrap();
        let name: String = String::from(&captures[1]);
        let valve: Valve = Valve {
            name: name.to_owned(),
            flow_rate: captures[2].parse::<u64>().unwrap(),
            tunnels: captures[3]
                .split(", ")
                .map(|x| String::from(x))
                .collect::<Vec<String>>(),
        };
        valves.insert(name, valve);
    }
    valves
}

fn part_1(contents: &str) -> u64 {
    let valves = parse_valves(contents);
    let total_minutes = 30;
    let mut current_minutes = 0;
    let mut routes: Vec<Route> = vec![Route::default()];
    let mut best_per_valve_set_and_current: HashMap<(String, Vec<String>), u64> = HashMap::new();

    while current_minutes < total_minutes {
        let mut next_routes: Vec<Route> = Vec::new();
        for route in routes.into_iter() {
            let valve = valves.get(&route.current).unwrap();

            // deal with open valve case
            if valve.flow_rate > 0 && !route.opened.contains(&valve.name) {
                let new_flow_rate =
                    route.flow_rate + (total_minutes - current_minutes - 1) * valve.flow_rate;

                let mut new_opened = route.opened.clone();
                new_opened.insert(route.current.clone());

                let tmp_opened = new_opened.clone();
                let mut vec_opened = tmp_opened
                    .iter()
                    .map(|x| x.to_owned())
                    .collect::<Vec<String>>();
                vec_opened.sort();

                let name = route.current.to_owned();
                let key = (name, vec_opened);

                match best_per_valve_set_and_current.get(&key) {
                    Some(flow_rate) if flow_rate >= &new_flow_rate => continue,
                    _ => {
                        best_per_valve_set_and_current
                            .entry(key)
                            .and_modify(|e| *e = new_flow_rate)
                            .or_insert(new_flow_rate);

                        next_routes.push(Route {
                            current: route.current.to_owned(),
                            opened: new_opened,
                            flow_rate: new_flow_rate,
                        });
                    }
                }
            }

            // deal with move cases
            for next_valve in valve.tunnels.iter() {
                let tmp_opened = route.opened.clone();
                let mut vec_opened = tmp_opened
                    .iter()
                    .map(|x| x.to_owned())
                    .collect::<Vec<String>>();
                vec_opened.sort();
                let key = (next_valve.to_owned(), vec_opened);
                match best_per_valve_set_and_current.get(&key) {
                    Some(flow_rate) => {
                        if flow_rate >= &route.flow_rate {
                            continue;
                        }
                    }
                    _ => {}
                }

                best_per_valve_set_and_current
                    .entry(key)
                    .and_modify(|e| *e = route.flow_rate)
                    .or_insert(route.flow_rate);

                next_routes.push(Route {
                    current: next_valve.to_owned(),
                    opened: route.opened.clone(),
                    flow_rate: route.flow_rate,
                })
            }
        }

        current_minutes += 1;
        if current_minutes > 0 {
            let num_routes = next_routes.len();
            let num_stored = best_per_valve_set_and_current.len();
            println!("MINUTES {current_minutes}, num routes {num_routes} num stored {num_stored}");
        }

        if next_routes.len() == 0 {
            break;
        };
        routes = next_routes;
    }

    best_per_valve_set_and_current
        .iter()
        .map(|(_, v)| *v)
        .max()
        .unwrap()
}

fn part_2(_contents: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 1651);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 1);
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
