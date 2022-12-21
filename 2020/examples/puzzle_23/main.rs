use std::time::Instant;

fn do_move(
    cups: &mut Vec<u64>,
    current_cup: &mut u64,
    current_cup_index: &mut usize,
    length: usize,
) {
    //println!("{cups:?} {current_cup_index}");

    let remove_index = (*current_cup_index + 1) % length;
    let value_1 = cups.remove(remove_index);
    let value_2 = if remove_index < cups.len() {
        cups.remove(remove_index)
    } else {
        cups.remove(0)
    };
    let value_3 = if remove_index < cups.len() {
        cups.remove(remove_index)
    } else {
        cups.remove(0)
    };
    //println!("{value_1} {value_2} {value_3}");

    let mut destination: u64 = current_cup.clone();
    loop {
        if destination == 1 {
            destination = length as u64;
        } else {
            destination -= 1;
        }
        if !(destination == value_1 || destination == value_2 || destination == value_3) {
            break;
        }
    }
    //println!("{destination}");

    let destination_index = cups.iter().position(|x| x == &destination).unwrap();
    //println!("{destination_index}");

    cups.insert((destination_index + 1) % length, value_3);
    cups.insert((destination_index + 1) % length, value_2);
    cups.insert((destination_index + 1) % length, value_1);
    //println!("{cups:?}");

    let updated_current_cup_index = cups.iter().position(|x| x == current_cup).unwrap();
    *current_cup_index = (updated_current_cup_index + 1) % length;
    *current_cup = cups[*current_cup_index];
}

fn part_1(contents: &str, num_moves: u8) -> u64 {
    let mut cups: Vec<u64> = Vec::new();
    for char in contents.chars() {
        let cup: u64 = char.to_digit(10).unwrap().into();
        cups.push(cup)
    }
    let length = cups.len();
    let mut current_cup = cups[0];
    let mut current_cup_index: usize = 0;
    for _ in 0..num_moves {
        do_move(&mut cups, &mut current_cup, &mut current_cup_index, length);
    }

    let one_index = cups.iter().position(|x| x == &1).unwrap();
    cups.iter()
        .enumerate()
        .map(|(i, x)| {
            if i == one_index {
                0
            } else {
                let power = if i >= one_index {
                    length - i + one_index - 1
                } else {
                    one_index - i - 1
                };
                x * 10u64.pow(power as u32)
            }
        })
        .sum()
}

fn part_2(contents: &str, num_moves: u64, numbers_up_to: usize) -> u64 {
    let mut cups: Vec<u64> = Vec::new();
    for char in contents.chars() {
        let cup: u64 = char.to_digit(10).unwrap().into();
        cups.push(cup)
    }
    for cup in (cups.len() + 1)..=numbers_up_to {
        cups.push(cup as u64);
    }

    let length = cups.len();
    let mut current_cup = cups[0];
    let mut current_cup_index: usize = 0;

    let start = Instant::now();
    for move_number in 0..num_moves {
        // 7 seconds/ 1000 moves => 70 000 seconds ~ 22 hours for all moves
        if move_number % 1_000 == 0 {
            let duration = start.elapsed();
            println!("Took {:?} up till move {move_number}", duration);
        }
        do_move(&mut cups, &mut current_cup, &mut current_cup_index, length);
    }

    let one_index = cups.iter().position(|x| x == &1).unwrap();
    cups[(one_index + 1) % length] * cups[(one_index + 2) % length]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example_few_moves() {
        assert_eq!(part_1("389125467", 10), 92658374);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1("389125467", 100), 67384529);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2("389125467", 10_000_000, 1_000_000), 149245887792);
    }
}

fn main() {
    let start = Instant::now();
    let part_1_answer = part_1("284573961", 100);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer = part_2("284573961", 10_000_000, 1_000_000);
    println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
