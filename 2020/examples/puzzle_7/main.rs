use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

static NULL_BAG: &str = "no other bag";

/* Create a tree where a bag points to its parents (i.e. bags it is contained in) */
fn parse_parent_tree(contents: &str) -> HashMap<String, Vec<String>> {
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
            let child_bag = child_bag_split[1].to_owned();

            parent_bags
                .entry(child_bag)
                .or_default()
                .push(components[0].to_owned());
        }
    }
    return parent_bags;
}

#[derive(Clone, Debug)]
struct Bag {
    name: String,
    count: i32,
}

/* Create a tree where a bag points to its children (i.e. bags it contains) */
fn parse_child_tree(contents: &str) -> HashMap<String, Vec<Bag>> {
    let mut child_bags: HashMap<String, Vec<Bag>> = HashMap::new();

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

            child_bags
                .entry(components[0].to_owned())
                .or_default()
                .push(Bag {
                    name: child_bag,
                    count: number_of_bags,
                });
        }
    }
    return child_bags;
}

fn part_1(contents: &String) {
    let parent_bags = parse_parent_tree(contents);

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

fn part_2(contents: &String) {
    let child_bags = parse_child_tree(contents);

    let start_bag: Bag = Bag {
        name: String::from("shiny gold bag"),
        count: 1,
    };
    let mut current_bags: VecDeque<Bag> = VecDeque::from([start_bag]);
    let mut num_bags: i32 = 0;
    while current_bags.len() > 0 {
        let current_bag = current_bags.pop_front().unwrap();
        match child_bags.get(&current_bag.name) {
            Some(bags) => {
                for bag in bags.iter() {
                    num_bags += current_bag.count * bag.count;
                    let mut new_bag: Bag = bag.clone();
                    new_bag.count = current_bag.count * bag.count;
                    current_bags.push_back(new_bag)
                }
            }
            None => {}
        }
    }

    println!("Answer for part 2 is: {}", num_bags);
}

fn main() {
    let start = Instant::now();
    let file_names: [&str; 3] = [
        "examples/puzzle_7/example_1.txt",
        "examples/puzzle_7/example_2.txt",
        "examples/puzzle_7/input.txt",
    ];
    for file_name in file_names.iter() {
        println!("FILE: {}", file_name);
        let contents = fs::read_to_string(file_name).expect("Could not read file");
        part_1(&contents);
        part_2(&contents);
        let duration = start.elapsed();
        println!("Took {:?} to solve this puzzle", duration);
    }
}
