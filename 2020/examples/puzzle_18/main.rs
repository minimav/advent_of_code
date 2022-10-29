use std::time::Instant;

#[derive(Debug)]
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

fn evaluate_expression(expression: &str) -> u128 {
    let components: Vec<Component> = expression
        .split_whitespace()
        .filter(|e| !e.is_empty())
        .flat_map(parse_chunk)
        .collect();
    println!("{:?}", components);
    0
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
