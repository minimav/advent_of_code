use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

static NULL_BAG: &str = "no other bag";

fn part_1(contents: &str) {
    let mut parent_bags: HashMap<String, Vec<String>> = HashMap::new();

    for line in contents.lines() {
        let deplural_line: String = line.replace("bags", "bag").replace(".", "");
        let components: Vec<&str> = deplural_line.split(" contain ").collect();
        if components[1] == NULL_BAG {
            continue;
        }

        let child_bags_raw = components[1].split(", ");
        for child_bag_raw in child_bags_raw {
            let child_bag_split: Vec<&str> = child_bag_raw.splitn(2, " ").collect();
            let number_of_bags = child_bag_split[0].parse::<i32>().unwrap();
            let child_bag = child_bag_split[1].to_owned();

            parent_bags
                .entry(child_bag)
                .or_default()
                .push(components[0].to_owned());
        }
    }

    let mut seen_bags: HashSet<String> = HashSet::new();
    let mut current_bags: VecDeque<String> = VecDeque::from([String::from("shiny gold bag")]);
    let mut bags_that_can_hold: HashSet<String> = HashSet::new();
    while current_bags.len() > 0 {
        let current_bag = current_bags.pop_front().unwrap().to_string();
        match parent_bags.get(&current_bag) {
            Some(bags) => {
                for bag in bags.iter() {
                    if !seen_bags.contains(bag) {
                        current_bags.push_back(bag.to_string())
                    }
                    bags_that_can_hold.insert(bag.to_string());
                }
            }
            None => {}
        }

        seen_bags.insert(current_bag);
    }

    println!("Answer for part 1 is: {}", bags_that_can_hold.len());
}

fn part_2(contents: &str) {}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    part_1(contents);
    part_2(contents);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
