use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Debug)]
enum Rule {
    SubString(String),
    Sequence(Vec<String>),
    Or(Vec<String>, Vec<String>),
}

impl Default for Rule {
    fn default() -> Self {
        Rule::SubString("".to_string())
    }
}

fn parse_sequence(rule: &str) -> Vec<String> {
    rule.split_whitespace()
        .into_iter()
        .map(|x| x.to_string())
        .collect()
}

fn parse_rule(rule: &str) -> Rule {
    if rule.contains('|') {
        let subsequences: Vec<&str> = rule.split("|").collect();
        Rule::Or(
            parse_sequence(subsequences[0]),
            parse_sequence(subsequences[1]),
        )
    } else if rule.contains('"') {
        let index = rule.chars().position(|c| c == '"').unwrap();
        Rule::SubString(String::from(rule.chars().nth(index + 1).unwrap()))
    } else {
        Rule::Sequence(parse_sequence(rule))
    }
}

fn parse_rules(contents: &str) -> HashMap<String, Rule> {
    let mut rules: HashMap<String, Rule> = HashMap::new();
    for line in contents.lines().take_while(|x| x != &"") {
        match line.split_once(':') {
            Some((rule_number, rule)) => {
                rules.insert(rule_number.to_string(), parse_rule(rule));
            }
            None => panic!("Shouldn't happen"),
        }
    }
    rules
}

fn parse_messages(contents: &str) -> HashSet<String> {
    let mut messages: HashSet<String> = HashSet::new();
    for line in contents.lines().skip_while(|x| x.contains(":")) {
        if line.is_empty() {
            continue;
        }
        messages.insert(line.to_string());
    }
    messages
}

#[derive(Debug)]
struct PossibleMessage {
    message: String,
    rule_numbers: Vec<String>,
}

fn concat(seq_1: &Vec<String>, seq_2: &[String]) -> Vec<String> {
    seq_1.iter().cloned().chain(seq_2.iter().cloned()).collect()
}

fn iterate_messages<'a>(
    possible_messages: &'a Vec<PossibleMessage>,
    valid_messages: &'a mut HashSet<String>,
    messages: &'a HashSet<String>,
    rules: &'a HashMap<String, Rule>,
) -> Vec<PossibleMessage> {
    // get maximum length of any input messages
    let max_len: usize = messages
        .iter()
        .reduce(|x, y| if x.len() > y.len() { x } else { y })
        .unwrap()
        .len();

    let mut next_possible_messages: Vec<PossibleMessage> = Vec::new();
    for possible_message in possible_messages {
        let (rule_number, remaining_rule_numbers) =
            possible_message.rule_numbers.split_first().unwrap();
        let rule = rules.get(rule_number).unwrap();

        match rule {
            Rule::SubString(character) => {
                let next_message: String = possible_message.message.clone() + character;
                if next_message.len() > max_len {
                    continue;
                } else if messages.contains(&next_message) {
                    if remaining_rule_numbers.len() == 0 {
                        valid_messages.insert(next_message);
                    } else {
                        next_possible_messages.push(PossibleMessage {
                            message: next_message,
                            rule_numbers: remaining_rule_numbers.to_vec(),
                        });
                    }
                } else if remaining_rule_numbers.len() == 0 {
                    continue;
                } else {
                    for message in messages {
                        if message.starts_with(&next_message) {
                            next_possible_messages.push(PossibleMessage {
                                message: next_message,
                                rule_numbers: remaining_rule_numbers.to_vec(),
                            });
                            break;
                        }
                    }
                }
            }
            Rule::Sequence(seq) => {
                next_possible_messages.push(PossibleMessage {
                    message: possible_message.message.clone(),
                    rule_numbers: concat(seq, remaining_rule_numbers),
                });
            }
            Rule::Or(option_1, option_2) => {
                next_possible_messages.push(PossibleMessage {
                    message: possible_message.message.clone(),
                    rule_numbers: concat(option_1, remaining_rule_numbers),
                });
                next_possible_messages.push(PossibleMessage {
                    message: possible_message.message.clone(),
                    rule_numbers: concat(option_2, remaining_rule_numbers),
                });
            }
        }
    }
    next_possible_messages
}

fn find_valid_message(rules: HashMap<String, Rule>, messages: HashSet<String>) -> usize {
    let mut valid_messages: HashSet<String> = HashSet::new();

    let mut possible_messages = vec![PossibleMessage {
        message: String::from(""),
        rule_numbers: vec![String::from("0")],
    }];
    loop {
        let next_possible_messages =
            iterate_messages(&possible_messages, &mut valid_messages, &messages, &rules);
        possible_messages = next_possible_messages;
        if possible_messages.len() == 0 {
            break;
        }
    }
    valid_messages.len()
}

fn part_1(contents: &str) -> usize {
    let rules = parse_rules(contents);
    let messages: HashSet<String> = parse_messages(contents);
    find_valid_message(rules, messages)
}

fn part_2(contents: &str) -> usize {
    let mut rules = parse_rules(contents);
    rules.entry("8".to_string()).and_modify(|v| {
        *v = Rule::Or(
            vec!["42".to_string()],
            vec!["42".to_string(), "8".to_string()],
        )
    });
    rules.entry("11".to_string()).and_modify(|v| {
        *v = Rule::Or(
            vec!["42".to_string(), "31".to_string()],
            vec!["42".to_string(), "11".to_string(), "31".to_string()],
        )
    });
    let messages: HashSet<String> = parse_messages(contents);
    find_valid_message(rules, messages)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 2);
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
