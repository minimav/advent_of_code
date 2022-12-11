use parse_display::{Display, FromStr};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone, Copy, Debug, Display, FromStr)]

enum Operation {
    #[display("Operation: new = old * old")]
    SQUARE,
    #[display("Operation: new = old + {0}")]
    ADD(u128),
    #[display("Operation: new = old * {0}")]
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
    test_modulo: u128,
    test_true: usize,
    test_false: usize,
}

impl Monkey {
    fn apply_test(&self, item: u128) -> usize {
        if item % self.test_modulo == 0 {
            self.test_true
        } else {
            self.test_false
        }
    }
}

#[derive(FromStr, Debug, PartialEq)]
#[display("  Starting items: {items}")]
#[from_str(new = Self::new(items))]
struct Items {
    items: Vec<u128>,
}

impl Items {
    fn new(s: String) -> Self {
        Items {
            items: s
                .split(", ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<u128>().unwrap())
                .collect(),
        }
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn pop(&mut self) -> Option<u128> {
        self.items.pop()
    }

    fn push(&mut self, item: u128) {
        self.items.push(item);
    }
}

fn parse_last<T: std::str::FromStr>(line: &str) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let components = line.split_whitespace().collect::<Vec<&str>>();
    components.last().unwrap().parse::<T>().unwrap()
}

fn parse_monkeys(contents: &str) -> (HashMap<usize, Monkey>, HashMap<usize, Items>) {
    let mut monkey_items: HashMap<usize, Items> = HashMap::new();
    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();

    // properties of a monkey that we'll update as we parse
    let mut monkey_index: usize = 0;
    let mut operation: Operation = Operation::SQUARE;
    let mut test_modulo: u128 = 0;
    let mut test_true: usize = 0;
    let mut test_false: usize = 0;

    let mut lines = contents.lines().peekable();
    while let Some(line) = lines.next() {
        if line.contains("Monkey") {
            monkey_index = parse_last(&line.replace(":", ""));
        } else if line.contains("Starting") {
            let items = String::from(line).parse().unwrap();
            monkey_items.insert(monkey_index, items);
        } else if line.contains("Operation") {
            operation = line.trim().parse().unwrap();
        } else if line.contains("Test") {
            test_modulo = parse_last(line);
        } else if line.contains("true") {
            test_true = parse_last(line);
        } else if line.contains("false") {
            test_false = parse_last(line);
        }
        if lines.peek().is_none() || line.is_empty() {
            let monkey = Monkey {
                operation,
                test_modulo,
                test_true,
                test_false,
            };
            monkeys.insert(monkey_index, monkey);
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
    let max_modulus: u128 = monkeys.iter().map(|(_, x)| x.test_modulo).product();

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
                        let new_item = monkey.operation.apply(item, divide_by_factor) % max_modulus;
                        let new_monkey_index = monkey.apply_test(new_item);
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
