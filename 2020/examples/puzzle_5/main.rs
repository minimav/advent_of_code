use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn parse_row(pass_fragment: &str) -> isize {
    let row_map: HashMap<char, char> = vec![('F', '0'), ('B', '1')].into_iter().collect();

    let mut binary: String = "".to_string();
    for char in pass_fragment.chars() {
        match row_map.get(&char) {
            Some(binary_char) => binary.push_str(&binary_char.to_string()),
            None => (),
        }
    }
    return isize::from_str_radix(&binary, 2).unwrap();
}

fn parse_column(pass_fragment: &str) -> isize {
    let column_map: HashMap<char, char> = vec![('L', '0'), ('R', '1')].into_iter().collect();

    let mut binary: String = "".to_string();
    for char in pass_fragment.chars() {
        match column_map.get(&char) {
            Some(binary_char) => binary.push_str(&binary_char.to_string()),
            None => (),
        }
    }
    return isize::from_str_radix(&binary, 2).unwrap();
}

fn compute_seat_id(row: isize, column: isize) -> isize {
    return row * 8 + column;
}

fn maximum_seat_id(contents: &str) -> isize {
    let mut max_seat_id: isize = 0;
    for boarding_pass in contents.lines() {
        let row = parse_row(&boarding_pass[0..7]);
        let column = parse_column(&boarding_pass[7..]);
        let seat_id = compute_seat_id(row, column);
        if seat_id > max_seat_id {
            max_seat_id = seat_id
        }
    }
    return max_seat_id;
}

fn find_seat_id(contents: &str) -> isize {
    let mut all_seat_ids: HashSet<isize> = HashSet::new();
    for boarding_pass in contents.lines() {
        let row = parse_row(&boarding_pass[0..7]);
        let column = parse_column(&boarding_pass[7..]);
        let seat_id = compute_seat_id(row, column);
        all_seat_ids.insert(seat_id);
    }

    let mut my_seat_id: isize = -1;
    loop {
        my_seat_id += 1;
        if all_seat_ids.contains(&my_seat_id) {
            continue;
        }
        let lower_neighbour = my_seat_id - 1;
        let upper_neighbour = my_seat_id + 1;
        if all_seat_ids.contains(&lower_neighbour) && all_seat_ids.contains(&upper_neighbour) {
            break;
        }
    }
    return my_seat_id;
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");

    let max_seat_id = maximum_seat_id(contents);
    println!("Maximum seat id: {}", max_seat_id);

    let seat_id = find_seat_id(contents);
    println!("My seat id: {}", seat_id);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
