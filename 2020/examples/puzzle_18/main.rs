use std::cmp;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
enum Component {
    Add,
    Multiply,
    OpenBracket,
    CloseBracket,
    Integer { i: u128 },
}

impl From<&str> for Component {
    fn from(s: &str) -> Component {
        match s.trim() {
            "*" => Component::Multiply,
            "+" => Component::Add,
            "(" => Component::OpenBracket,
            ")" => Component::CloseBracket,
            integer => Component::Integer {
                i: integer.parse::<u128>().unwrap(),
            },
        }
    }
}

impl From<char> for Component {
    fn from(c: char) -> Component {
        Component::from(c.to_string())
    }
}

impl From<String> for Component {
    fn from(s: String) -> Component {
        Component::from(s.as_str())
    }
}

fn parse_chunk(chunk: &str) -> Vec<Component> {
    let mut output: Vec<Component> = Vec::new();
    let mut buffer: Vec<char> = Vec::new();
    for char in chunk.chars() {
        if char == '(' || char == ')' {
            if !buffer.is_empty() {
                output.push(Component::from(String::from_iter(buffer.iter())));
                buffer = Vec::new();
            }
            output.push(Component::from(char));
        } else {
            buffer.push(char);
        }
    }
    if !buffer.is_empty() {
        output.push(Component::from(String::from_iter(buffer.iter())));
    }
    output
}

fn extract_number(component: Component) -> Result<u128, String> {
    match component {
        Component::Integer { i } => Ok(i),
        _ => return Err("First component must be an integer!".to_string()),
    }
}

fn apply_operator(value: u128, operator: &Component, next_value: u128) -> Result<u128, String> {
    match operator {
        Component::Add => Ok(value + next_value),
        Component::Multiply => Ok(value * next_value),
        _ => return Err("Expected a * or + here!".to_string()),
    }
}

fn evaluate_no_brackets(components: Vec<Component>) -> Component {
    let mut value: u128 = 0;
    let mut operator: Component = Component::Add;
    for (i, component) in components.into_iter().enumerate() {
        if i == 0 {
            value += extract_number(component).unwrap();
            continue;
        } else if i % 2 == 1 {
            operator = component;
        } else {
            let next_value = extract_number(component).unwrap();
            value = apply_operator(value, &operator, next_value).unwrap();
        }
    }
    Component::Integer { i: value }
}

fn evaluate_expression(expression: &str) -> u128 {
    let mut components: Vec<Component> = expression
        .split_whitespace()
        .filter(|e| !e.is_empty())
        .flat_map(parse_chunk)
        .collect();

    loop {
        let mut last_opening_bracket = 0;
        let mut first_closing_bracket = 0;
        for (i, component) in components.iter().enumerate() {
            if component == &Component::OpenBracket {
                last_opening_bracket = i
            } else if component == &Component::CloseBracket {
                first_closing_bracket = i;
                break;
            }
        }
        if cmp::max(last_opening_bracket, first_closing_bracket) == 0 {
            return match evaluate_no_brackets(components) {
                Component::Integer { i } => i,
                _ => panic!("Shouldn't happen!"),
            };
        }
        let mut after_components = components.split_off(first_closing_bracket + 1);
        let mut middle_components = components.split_off(last_opening_bracket + 1);
        middle_components.truncate(middle_components.len() - 1);
        let bracket_eval = evaluate_no_brackets(middle_components);

        // drop ( from expression we evaluated, then add in next parts
        components.truncate(components.len() - 1);
        components.push(bracket_eval);
        components.append(&mut after_components);
    }
}

fn part_1(contents: &str) -> u128 {
    let mut answer: u128 = 0;
    for expression in contents.lines() {
        answer += evaluate_expression(expression);
    }
    answer
}

fn part_2(_contents: &str) -> u128 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(vec![Component::Integer { i: 1 }, Component::Add, Component::Integer { i: 2 }], Component::Integer {i:3})]
    #[case(vec![Component::Integer { i: 3 }, Component::Multiply, Component::Integer { i: 2 }], Component::Integer {i:6})]
    #[case(vec![Component::Integer { i: 1 }, Component::Add, Component::Integer { i: 2 }, Component::Multiply, Component::Integer { i: 3 }], Component::Integer {i:9})]
    #[case(vec![Component::Integer { i: 1 }, Component::Multiply, Component::Integer { i: 2 }, Component::Add, Component::Integer { i: 3 }], Component::Integer {i:5})]
    fn test_evaluate_no_brackets(
        #[case] components: Vec<Component>,
        #[case] expected_output: Component,
    ) {
        assert_eq!(evaluate_no_brackets(components), expected_output);
    }

    #[rstest]
    #[case("1 + 2 * 3 + 4 * 5 + 6", 71)]
    #[case("1 + (2 * 3) + (4 * (5 + 6))", 51)]
    #[case("2 * 3 + (4 * 5)", 26)]
    #[case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437)]
    #[case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240)]
    #[case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632)]
    fn test_evaluate_expression(#[case] expression: &str, #[case] expected_output: u128) {
        assert_eq!(evaluate_expression(expression), expected_output);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let part_1_answer: u128 = part_1(contents);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer: u128 = part_2(contents);
    println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
