use std::time::Instant;

const MODULO: u128 = 20201227;

fn get_loop_size(public_key: u128) -> u128 {
    let mut loop_number = 0;
    let mut value = 1;
    loop {
        value *= 7;
        value = value % MODULO;
        loop_number += 1;
        if value == public_key {
            return loop_number;
        }
    }
}

fn get_encryption_key(card_public_key: u128, door_public_key: u128) -> u128 {
    let card_loop_size = get_loop_size(card_public_key);
    let door_loop_size = get_loop_size(door_public_key);

    let subject_number = door_public_key;
    let mut value = 1;
    for _ in 0..card_loop_size {
        value *= subject_number;
        value = value % MODULO
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_get_encryption_key_example() {
        assert_eq!(part_1(5764801, 17807724), 14897079);
    }
}

fn main() {
    let start = Instant::now();
    let answer = get_encryption_key(17115212, 3667832);
    println!("Answer is: {}", answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
