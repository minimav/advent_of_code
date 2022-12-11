use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
enum Operation {
    SQUARE,
    ADD(u128),
    MULTIPLY(u128),
}

impl Operation {
    fn apply(&self, item: u128, divide_by_factor: u128) -> u128 {
        let new_item = match self {
            Operation::SQUARE => item * item,
            Operation::ADD(value) => item + value,
            Operation::MULTIPLY(value) => item * value,
        };
        new_item / divide_by_factor
    }
}

#[derive(Debug)]
struct Monkey {
    operation: Operation,
    test: (u128, usize, usize),
}

fn apply_test(test: (u128, usize, usize), item: u128) -> usize {
    if item % test.0 == 0 {
        test.1
    } else {
        test.2
    }
}

fn parse_monkeys(contents: &str) -> (HashMap<usize, Monkey>, HashMap<usize, Vec<u128>>) {
    let mut monkey_items: HashMap<usize, Vec<u128>> = HashMap::new();
    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();

    // properties of a monkey that we'll update as we parse
    let mut monkey_index: usize = 0;
    let mut operation: Operation = Operation::SQUARE;
    let mut test_mod: u128 = 0;
    let mut test_true: usize = 0;
    let mut test_false: usize = 0;

    let mut lines = contents.lines().peekable();
    while let Some(line) = lines.next() {
        if lines.peek().is_none() || line.is_empty() {
            let monkey = Monkey {
                operation,
                test: (test_mod, test_true, test_false),
            };
            monkeys.insert(monkey_index, monkey);
        } else if line.contains("Monkey") {
            let cleaned_line = line.replace(":", "");
            let components = cleaned_line.split_whitespace().collect::<Vec<&str>>();
            monkey_index = components[1].parse::<usize>().unwrap();
        } else if line.contains("Starting") {
            let cleaned_line = line.replace(" ", "");
            let components = cleaned_line.split(":").collect::<Vec<&str>>();
            let items = components[1]
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<u128>().unwrap())
                .collect();
            monkey_items.insert(monkey_index, items);
        } else if line.contains("Operation") {
            if line.contains("* old") {
                operation = Operation::SQUARE
            } else if line.contains("*") {
                let components = line.split_whitespace().collect::<Vec<&str>>();
                let value = components.last().unwrap().parse::<u128>().unwrap();
                operation = Operation::MULTIPLY(value)
            } else if line.contains("+") {
                let components = line.split_whitespace().collect::<Vec<&str>>();
                let value = components.last().unwrap().parse::<u128>().unwrap();
                operation = Operation::ADD(value)
            }
        } else if line.contains("Test") {
            let components = line.split_whitespace().collect::<Vec<&str>>();
            test_mod = components.last().unwrap().parse::<u128>().unwrap();
        } else if line.contains("true") {
            let components = line.split_whitespace().collect::<Vec<&str>>();
            test_true = components.last().unwrap().parse::<usize>().unwrap();
        } else if line.contains("false") {
            let components = line.split_whitespace().collect::<Vec<&str>>();
            test_false = components.last().unwrap().parse::<usize>().unwrap();
        }
    }

    (monkeys, monkey_items)
}

fn process_monkeys(contents: &str, num_rounds: u32, divide_by_factor: u128) -> usize {
    let (mut monkeys, mut monkey_items) = parse_monkeys(contents);
    let mut num_observed: HashMap<usize, usize> = HashMap::new();
    let mut move_queue: Vec<(usize, u128)> = Vec::new();
    let num_monkeys = monkey_items.len();

    // modulus by this to prevent overflows
    let max_mod = monkeys
        .iter()
        .map(|(_, x)| x.test.0)
        .reduce(|a, b| a * b)
        .unwrap();

    for _ in 0..num_rounds {
        for monkey_index in 0..num_monkeys {
            // do any moves from previous monkey
            for (move_monkey_index, item) in move_queue.iter() {
                match monkey_items.get_mut(move_monkey_index) {
                    Some(items) => {
                        items.push(*item);
                    }
                    _ => {}
                }
            }
            move_queue = Vec::new();

            let monkey = monkeys.get_mut(&monkey_index).unwrap();
            let num_items = monkey_items[&monkey_index].len();

            // update the number observed
            num_observed
                .entry(monkey_index)
                .and_modify(|e| *e += num_items)
                .or_insert(num_items);

            for _ in 0..monkey_items[&monkey_index].len() {
                match monkey_items.get_mut(&monkey_index) {
                    Some(items) => {
                        let item = items.pop().unwrap();
                        let new_item = monkey.operation.apply(item, divide_by_factor) % max_mod;
                        let new_monkey_index = apply_test(monkey.test, new_item);
                        move_queue.push((new_monkey_index, new_item));
                    }
                    _ => {}
                }
            }
        }
    }
    let mut items_seen = num_observed.values().collect::<Vec<&usize>>();
    items_seen.sort_by(|a, b| b.cmp(a));
    items_seen[0] * items_seen[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(process_monkeys(include_str!("./example.txt"), 20, 3), 10605);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(
            process_monkeys(include_str!("./example.txt"), 10_000, 1),
            2713310158
        );
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let part_1_answer = process_monkeys(contents, 20, 3);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer = process_monkeys(contents, 10_000, 1);
    println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
