use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn valid_password_part_1(
    password: String,
    character: char,
    min_occurrences: u8,
    max_occurrences: u8,
) -> bool {
    let mut occurrences: HashMap<char, u8> = HashMap::new();
    for password_character in password.chars() {
        if !occurrences.contains_key(&password_character) {
            occurrences.insert(password_character, 1);
        } else {
            occurrences.insert(password_character, occurrences[&password_character] + 1);
        }
    }

    // compare count for the selected character
    let num_occurrences = {
        if !occurrences.contains_key(&character) {
            0
        } else {
            occurrences[&character]
        }
    };
    min_occurrences <= num_occurrences && num_occurrences <= max_occurrences
}

fn valid_password_part_2(
    password: String,
    character: char,
    first_index: u8,
    second_index: u8,
) -> bool {
    let mut num_matches = 0;
    match password.chars().nth((first_index - 1).into()) {
        Some(c) if c == character => num_matches += 1,
        _ => {}
    }
    match password.chars().nth((second_index - 1).into()) {
        Some(c) if c == character => num_matches += 1,
        _ => {}
    }
    num_matches == 1
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string("examples/puzzle_2/input.txt").expect("Could not read file");

    let mut num_valid_part_1 = 0;
    let mut num_valid_part_2 = 0;
    for line in contents.lines() {
        let parsed_line: String = line.trim().parse().unwrap();
        let line_parts: Vec<&str> = parsed_line.split_whitespace().collect();

        let conditions: Vec<&str> = line_parts[0].split("-").collect();
        let character = line_parts[1].chars().next().expect("string is empty");
        let password = line_parts[2];

        let min_occurrences = conditions[0].parse::<u8>().unwrap();
        let max_occurrences = conditions[1].parse::<u8>().unwrap();

        if valid_password_part_1(
            password.to_string(),
            character,
            min_occurrences,
            max_occurrences,
        ) {
            num_valid_part_1 += 1;
        }
        if valid_password_part_2(
            password.to_string(),
            character,
            min_occurrences,
            max_occurrences,
        ) {
            num_valid_part_2 += 1;
        }
    }
    println!("{} valid passwwords for part 1", num_valid_part_1);
    println!("{} valid passwwords for part 2", num_valid_part_2);

    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
