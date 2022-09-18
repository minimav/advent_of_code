use std::collections::{HashMap, HashSet};
use std::time::Instant;

static NO_OP: &str = "nop";
static ACC: &str = "acc";
static JUMP: &str = "jmp";

/* Output is in form command number: (move by this amount, increment acc by this) */
fn parse_instructions(contents: &str) -> HashMap<i32, (i32, i32)> {
    let mut commands: HashMap<i32, (i32, i32)> = HashMap::new();
    for (index, line) in contents.lines().enumerate() {
        let split_line: Vec<&str> = line.split(" ").collect();

        let value: i32 = split_line[1].parse::<i32>().unwrap();
        if split_line[0] == NO_OP {
            commands.insert(index as i32, (1, 0));
        } else if split_line[0] == ACC {
            commands.insert(index as i32, (1, value));
        } else if split_line[0] == JUMP {
            commands.insert(index as i32, (value, 0));
        }
    }
    commands
}

fn part_1(contents: &str) {
    let commands = parse_instructions(contents);
    let mut acc = 0;
    let mut seen_indexes: HashSet<i32> = HashSet::new();
    let mut current_index: i32 = 0;
    loop {
        if current_index < 0 {
            panic!("Index went negative, this shouldn't happen!");
        } else if seen_indexes.contains(&current_index) {
            break;
        }
        seen_indexes.insert(current_index);
        let (move_by, acc_change) = commands.get(&current_index).unwrap();
        acc += acc_change;
        current_index += move_by
    }
    println!("Answer to part 1 is: {}", acc);
}

/* Perturb the first jump/nop command directly after the previously perturbed one.
 *
 * Output is in form command number: (move by this amount, increment acc by this)
 */
fn parse_instructions_with_perturbation(
    contents: &str,
    last_perturb_index: i32,
) -> (HashMap<i32, (i32, i32)>, i32) {
    let mut commands: HashMap<i32, (i32, i32)> = HashMap::new();
    let mut done_perturbation = false;
    let mut perturb_index: i32 = -1;
    for (index, line) in contents.lines().enumerate() {
        let split_line: Vec<&str> = line.split(" ").collect();
        let value: i32 = split_line[1].parse::<i32>().unwrap();

        // accumulator commands are never perturbed
        if split_line[0] == ACC {
            commands.insert(index as i32, (1, value));
            continue;
        }
        // perturb on the next jump/nop command after the last one we perturbed
        let should_perturb = index as i32 > last_perturb_index && !done_perturbation;

        if split_line[0] == NO_OP && !should_perturb {
            commands.insert(index as i32, (1, 0));
        } else if split_line[0] == NO_OP {
            commands.insert(index as i32, (value, 0));
            done_perturbation = true;
            perturb_index = index as i32;
        } else if split_line[0] == JUMP && !should_perturb {
            commands.insert(index as i32, (value, 0));
        } else if split_line[0] == JUMP {
            commands.insert(index as i32, (1, 0));
            done_perturbation = true;
            perturb_index = index as i32;
        }
    }
    if perturb_index < 0 {
        panic!("No perturbation occurred, this shouldn't happen!");
    }
    return (commands, perturb_index);
}

fn part_2(contents: &str) {
    let mut perturb_index = 0;

    // outer loop we perturb the next jump/nop command each time
    loop {
        let (commands, new_perturb_index) =
            parse_instructions_with_perturbation(contents, perturb_index);
        perturb_index = new_perturb_index;
        let mut acc = 0;
        let mut seen_indexes: HashSet<i32> = HashSet::new();
        let mut current_index: i32 = 0;
        let num_lines = contents.lines().count() as i32;
        let mut success = false;

        // inner loop traverses the commands with the single perturbation
        loop {
            if current_index < 0 {
                panic!("Index went negative, this shouldn't happen!");
            } else if current_index == num_lines {
                success = true;
                break;
            } else if seen_indexes.contains(&current_index) {
                break;
            }
            seen_indexes.insert(current_index);
            let (move_by, acc_change) = commands.get(&current_index).unwrap();
            acc += acc_change;
            current_index += move_by
        }

        if success {
            println!("Answer to part 2 is: {}", acc);
            break;
        }
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    part_1(contents);
    part_2(contents);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
