use std::time::Instant;
use std::collections::HashSet;

fn parse(contents: &str) -> HashSet<(i32, i32)> {
    let lines: Vec<&str> = contents.lines().collect();    
    let mut locations = HashSet::new();
    let mut row = 0;
    for line in lines {
        for (col, c) in line.chars().enumerate() {
            if c == '@' {
                locations.insert((row as i32, col as i32));
            }
        }
        row += 1;
    }
    return locations;
}

fn part_1(contents: &str) -> u32 {
    let locations = parse(contents);
    let mut answer = 0;
    for location in &locations {
        let (row, col) = location;
        let mut neighbours = 0;
        let combos = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
        for (dr, dc) in &combos {
            let new_row = row + dr;
            let new_col = col + dc;
            if locations.contains(&(new_row, new_col)) {
                neighbours += 1;
            }
        }
        if neighbours <= 3 {
            answer += 1;
        }
    }
    return answer;
}

fn part_2(contents: &str) -> u32 {
    let mut locations = parse(contents);
    let mut answer = 0;
    let mut size = locations.len();
    loop {
        let mut new_locations: HashSet<(i32, i32)> = HashSet::new();

        for location in &locations {
            let (row, col) = location;
            let mut neighbours = 0;
            let combos = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
            for (dr, dc) in &combos {
                let new_row = row + dr;
                let new_col = col + dc;
                if locations.contains(&(new_row, new_col)) {
                    neighbours += 1;
                }
            }
            if neighbours <= 3 {
                answer += 1;
            } else {
                new_locations.insert((*row, *col));
            }
        }
        if new_locations.len() == size {
            break;
        }
        size = new_locations.len();
        locations = new_locations;
    }

    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 13);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 43);
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
