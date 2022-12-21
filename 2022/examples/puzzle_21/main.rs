use either::Either;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::time::Instant;

#[derive(Debug, Clone)]
enum Equation {
    EQUALS(Either<String, i128>, Either<String, i128>),
    ADD(Either<String, i128>, Either<String, i128>),
    SUBTRACT(Either<String, i128>, Either<String, i128>),
    TIMES(Either<String, i128>, Either<String, i128>),
    NUMBER(i128),
    UNKNOWN_NUMBER(String),
    DIVIDE(Either<String, i128>, Either<String, i128>),
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Equation::EQUALS(l, r) => {
                write!(f, "{l} == {r}")
            }
            Equation::ADD(l, r) => {
                write!(f, "({l} + {r})")
            }
            Equation::SUBTRACT(l, r) => {
                write!(f, "({l} - {r})")
            }
            Equation::TIMES(l, r) => {
                write!(f, "({l} * {r})")
            }
            Equation::DIVIDE(l, r) => {
                write!(f, "({l} / {r})")
            }
            Equation::NUMBER(v) => {
                write!(f, "{v}")
            }
            Equation::UNKNOWN_NUMBER(v) => {
                write!(f, "{v}")
            }
        }
    }
}

impl Equation {
    fn substitute_lhs(&self, key: &String, number: i128) -> Equation {
        match self {
            Equation::EQUALS(Either::Left(v), right) if v == key => {
                Equation::EQUALS(Either::Right(number), right.clone())
            }
            Equation::ADD(Either::Left(v), right) if v == key => {
                Equation::ADD(Either::Right(number), right.clone())
            }
            Equation::SUBTRACT(Either::Left(v), right) if v == key => {
                Equation::SUBTRACT(Either::Right(number), right.clone())
            }
            Equation::TIMES(Either::Left(v), right) if v == key => {
                Equation::TIMES(Either::Right(number), right.clone())
            }
            Equation::DIVIDE(Either::Left(v), right) if v == key => {
                Equation::DIVIDE(Either::Right(number), right.clone())
            }
            _ => self.clone(),
        }
    }

    fn substitute_rhs(&self, key: &String, number: i128) -> Equation {
        match self {
            Equation::EQUALS(left, Either::Left(v)) if v == key => {
                Equation::EQUALS(left.clone(), Either::Right(number))
            }
            Equation::ADD(left, Either::Left(v)) if v == key => {
                Equation::ADD(left.clone(), Either::Right(number))
            }
            Equation::SUBTRACT(left, Either::Left(v)) if v == key => {
                Equation::SUBTRACT(left.clone(), Either::Right(number))
            }
            Equation::TIMES(left, Either::Left(v)) if v == key => {
                Equation::TIMES(left.clone(), Either::Right(number))
            }
            Equation::DIVIDE(left, Either::Left(v)) if v == key => {
                Equation::DIVIDE(left.clone(), Either::Right(number))
            }
            _ => self.clone(),
        }
    }

    fn compute(&self) -> Equation {
        match self {
            Equation::ADD(Either::Right(v_1), Either::Right(v_2)) => Equation::NUMBER(v_1 + v_2),
            Equation::SUBTRACT(Either::Right(v_1), Either::Right(v_2)) => {
                Equation::NUMBER(v_1 - v_2)
            }
            Equation::TIMES(Either::Right(v_1), Either::Right(v_2)) => Equation::NUMBER(v_1 * v_2),
            Equation::DIVIDE(Either::Right(v_1), Either::Right(v_2)) => Equation::NUMBER(v_1 / v_2),
            _ => self.clone(),
        }
    }
}

fn parse_equations(contents: &str) -> HashMap<String, Equation> {
    let mut equations: HashMap<String, Equation> = HashMap::new();
    for line in contents.lines() {
        let components = line.trim().split_whitespace().collect::<Vec<&str>>();
        if line.contains("+") {
            equations.insert(
                components[0].replace(":", ""),
                Equation::ADD(
                    Either::Left(components[1].to_owned()),
                    Either::Left(components[3].to_owned()),
                ),
            );
        } else if line.contains("-") {
            equations.insert(
                components[0].replace(":", ""),
                Equation::SUBTRACT(
                    Either::Left(components[1].to_owned()),
                    Either::Left(components[3].to_owned()),
                ),
            );
        } else if line.contains("*") {
            equations.insert(
                components[0].replace(":", ""),
                Equation::TIMES(
                    Either::Left(components[1].to_owned()),
                    Either::Left(components[3].to_owned()),
                ),
            );
        } else if line.contains("/") {
            equations.insert(
                components[0].replace(":", ""),
                Equation::DIVIDE(
                    Either::Left(components[1].to_owned()),
                    Either::Left(components[3].to_owned()),
                ),
            );
        } else {
            equations.insert(
                components[0].replace(":", ""),
                Equation::NUMBER(components[1].parse::<i128>().unwrap()),
            );
        }
    }
    equations
}

fn part_1(contents: &str) -> i128 {
    let mut equations = parse_equations(contents);
    let mut number_keys: VecDeque<String> = VecDeque::new();
    for (key, equation) in equations.iter() {
        match equation {
            Equation::NUMBER(_) => number_keys.push_front(key.clone()),
            _ => (),
        }
    }
    while let Some(key) = number_keys.pop_front() {
        let mut new_equations = HashMap::new();
        let number = match equations.get(&key) {
            Some(Equation::NUMBER(n)) => *n,
            _ => panic!("Could not find number!"),
        };
        for (other_key, equation) in equations {
            match equation {
                Equation::NUMBER(_) => {
                    new_equations.insert(other_key, equation);
                    continue;
                }
                _ => (),
            }

            let new_equation = equation
                .substitute_lhs(&key, number)
                .substitute_rhs(&key, number)
                .compute();
            match new_equation {
                Equation::NUMBER(_) => {
                    number_keys.push_back(other_key.clone());
                }
                _ => (),
            }
            new_equations.insert(other_key, new_equation);
        }
        equations = new_equations;
    }
    match equations.get(&String::from("root")) {
        Some(Equation::NUMBER(v)) => *v,
        _ => panic!("root was not a number!"),
    }
}

fn part_2(contents: &str) -> String {
    let mut equations = parse_equations(contents);
    let mut number_keys: VecDeque<String> = VecDeque::new();

    // make modifications to input
    equations
        .entry(String::from("root"))
        .and_modify(|x| match x {
            Equation::ADD(left, right) => *x = Equation::EQUALS(left.clone(), right.clone()),
            Equation::SUBTRACT(left, right) => *x = Equation::EQUALS(left.clone(), right.clone()),
            Equation::TIMES(left, right) => *x = Equation::EQUALS(left.clone(), right.clone()),
            Equation::DIVIDE(left, right) => *x = Equation::EQUALS(left.clone(), right.clone()),
            _ => panic!("root should be an operator"),
        });

    equations
        .entry(String::from("humn"))
        .and_modify(|x| *x = Equation::UNKNOWN_NUMBER(String::from("humn")));

    for (key, equation) in equations.iter() {
        match equation {
            Equation::NUMBER(_) => number_keys.push_front(key.clone()),
            _ => (),
        }
    }
    while let Some(key) = number_keys.pop_front() {
        let mut new_equations = HashMap::new();
        let number = match equations.get(&key) {
            Some(Equation::NUMBER(n)) => *n,
            _ => panic!("Could not find number!"),
        };
        for (other_key, equation) in equations {
            match equation {
                Equation::NUMBER(_) => {
                    new_equations.insert(other_key, equation);
                    continue;
                }
                _ => (),
            }

            let new_equation = equation
                .substitute_lhs(&key, number)
                .substitute_rhs(&key, number)
                .compute();
            match new_equation {
                Equation::NUMBER(_) => {
                    number_keys.push_back(other_key.clone());
                }
                _ => (),
            }
            new_equations.insert(other_key, new_equation);
        }
        equations = new_equations;
    }

    // truncate to only unknown variables
    let mut unknowns: HashMap<String, Equation> = HashMap::new();
    for (key, equation) in equations {
        match equation {
            Equation::NUMBER(_) => (),
            _ => {
                unknowns.insert(key, equation);
            }
        };
    }

    let mut keys_to_substitute: VecDeque<String> = VecDeque::from([String::from("root")]);
    let mut final_equation = String::from("root");
    while let Some(key) = keys_to_substitute.pop_front() {
        let (_, equation) = unknowns.remove_entry(&String::from(key.clone())).unwrap();
        let equation_string = equation.clone().to_string();
        final_equation = final_equation.replace(&key, &equation_string);

        // add next keys to substitute
        match equation.clone() {
            Equation::EQUALS(Either::Left(v), _) => keys_to_substitute.push_front(v),
            Equation::ADD(Either::Left(v), _) => keys_to_substitute.push_front(v),
            Equation::SUBTRACT(Either::Left(v), _) => keys_to_substitute.push_front(v),
            Equation::TIMES(Either::Left(v), _) => keys_to_substitute.push_front(v),
            Equation::DIVIDE(Either::Left(v), _) => keys_to_substitute.push_front(v),
            Equation::UNKNOWN_NUMBER(v) => keys_to_substitute.push_front(v),
            _ => (),
        }
        match equation.clone() {
            Equation::EQUALS(_, Either::Left(v)) => keys_to_substitute.push_front(v),
            Equation::ADD(_, Either::Left(v)) => keys_to_substitute.push_front(v),
            Equation::SUBTRACT(_, Either::Left(v)) => keys_to_substitute.push_front(v),
            Equation::TIMES(_, Either::Left(v)) => keys_to_substitute.push_front(v),
            Equation::DIVIDE(_, Either::Left(v)) => keys_to_substitute.push_front(v),
            Equation::UNKNOWN_NUMBER(v) => keys_to_substitute.push_front(v),
            _ => (),
        }
    }
    final_equation
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 152);
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
