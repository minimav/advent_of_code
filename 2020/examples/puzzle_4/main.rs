use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

type Validator = fn(&str) -> bool;

fn validate_birth_year(year: &str) -> bool {
    match year.parse::<i32>() {
        Ok(y) => (y >= 1920) && (y <= 2002),
        Err(_) => false,
    }
}

fn validate_issue_year(year: &str) -> bool {
    match year.parse::<i32>() {
        Ok(y) => (y >= 2010) && (y <= 2020),
        Err(_) => false,
    }
}

fn validate_expiration_year(year: &str) -> bool {
    match year.parse::<i32>() {
        Ok(y) => (y >= 2020) && (y <= 2030),
        Err(_) => false,
    }
}

fn validate_height(raw_height: &str) -> bool {
    let height = raw_height.to_string();
    let min_height = if height.ends_with("cm") { 150 } else { 59 };
    let max_height = if height.ends_with("cm") { 193 } else { 76 };

    let numeric_part = &height[0..height.len() - 2];
    match numeric_part.parse::<i32>() {
        Ok(n) => (n >= min_height) && (n <= max_height),
        Err(_) => false,
    }
}

fn validate_hair_colour(colour: &str) -> bool {
    if colour.len() != 7 {
        return false;
    }
    (&colour[0..1] == "#") && colour[1..7].chars().all(char::is_alphanumeric)
}

fn validate_eye_colour(colour: &str) -> bool {
    let valid_colours: HashSet<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .into_iter()
        .collect();
    valid_colours.contains(colour)
}

fn validate_passport_id(id: &str) -> bool {
    (id.len() == 9) && id.chars().all(char::is_numeric)
}

fn part_1(input: &str) -> u32 {
    let required_fields: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .collect();
    let expected_num_valid_fields = required_fields.len();

    let mut num_valid_passports = 0;
    let mut num_valid_fields = 0;

    for line in input.split("\n") {
        if line == "" {
            // new passport about to begin, check old one first
            if num_valid_fields == expected_num_valid_fields {
                num_valid_passports += 1;
            }
            num_valid_fields = 0;
        } else {
            for field_info in line.split(" ") {
                let key_value = field_info.split(":").collect::<Vec<&str>>();
                if required_fields.contains(key_value[0]) {
                    num_valid_fields += 1;
                }
            }
        }
    }

    // check the final passport
    if num_valid_fields == expected_num_valid_fields {
        num_valid_passports += 1;
    }
    num_valid_passports
}

fn part_2(input: &str) -> u32 {
    // each fn type is unique - we add the type we want for the first one and the rest as inferred
    let validators: HashMap<&str, Validator> = vec![
        ("byr", validate_birth_year as Validator),
        ("iyr", validate_issue_year),
        ("eyr", validate_expiration_year),
        ("hgt", validate_height),
        ("hcl", validate_hair_colour),
        ("ecl", validate_eye_colour),
        ("pid", validate_passport_id),
    ]
    .into_iter()
    .collect();
    let expected_num_valid_fields = validators.len();
    let mut num_valid_passports = 0;
    let mut num_valid_fields = 0;

    for line in input.split("\n") {
        if line == "" {
            // new passport about to begin, check old one first
            if num_valid_fields == expected_num_valid_fields {
                num_valid_passports += 1;
            }
            num_valid_fields = 0;
        } else {
            for field_info in line.split(" ") {
                let (key, value) = field_info.split(":").collect_tuple().unwrap();
                match validators.get(key) {
                    Some(validator) => {
                        if validator(value) {
                            num_valid_fields += 1
                        }
                    }
                    None => (),
                }
            }
        }
    }

    // check the final passport
    if num_valid_fields == expected_num_valid_fields {
        num_valid_passports += 1;
    }
    num_valid_passports
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let part_1_answer = part_1(contents);
    let part_2_answer = part_2(contents);

    println!("Part 1: {}, Part 2: {}", part_1_answer, part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
