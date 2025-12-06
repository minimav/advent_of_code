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
    let combos = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
    loop {
        let mut new_locations: HashSet<(i32, i32)> = HashSet::new();

        for location in &locations {
            let (row, col) = location;
            let mut neighbours = 0;
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


fn parse_fast(lines: Vec<&str>, num_rows: usize, num_cols: usize) -> Vec<u8> {
    // Store locations as flat vector for speed
    let mut locations = vec![0u8; num_rows * num_cols];
    let mut row = 0;
    for line in lines {
        for (col, c) in line.chars().enumerate() {
            if c == '@' {
                locations[row * num_cols + col] = 1;
            }
        }
        row += 1;
    }
    return locations;
}

fn part_1_fast(contents: &str) -> u32 {
    let lines: Vec<&str> = contents.lines().collect();    
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let locations = parse_fast(lines, num_rows, num_cols);

    let mut answer = 0;
    for (index, _) in locations.iter().enumerate().filter(|&(_, &v)| v == 1) {;
        let row = index / num_cols;
        let col = index % num_cols;
        let mut neighbours = 0;
        let combos = [(-1isize, 0isize), (1, 0), (0, -1), (0, 1),
                      (-1, -1), (-1, 1), (1, -1), (1, 1)];
        for (dr, dc) in &combos {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;
            // Bounds check using signed values
            if new_row < 0 || new_col < 0 || new_row >= num_rows as isize || new_col >= num_cols as isize {
                continue;
            }
            let neighbour_index = (new_row as usize) * num_cols + (new_col as usize);
            if locations[neighbour_index] == 1 {
                neighbours += 1;
            }
        }
        if neighbours <= 3 {
            answer += 1;
        }
    }
    return answer;
}

fn part_2_fast(contents: &str) -> u32 {
    let lines: Vec<&str> = contents.lines().collect();    
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let mut locations = parse_fast(lines, num_rows, num_cols);

    let combos = [
        (-1isize, 0isize), (1, 0), (0, -1), (0, 1),
        (-1, -1), (-1, 1), (1, -1), (1, 1)
    ];

    let mut answer = 0;
    let mut mask = locations.iter().enumerate().filter(|&(_, &v)| v == 1).map(|(i, _)| i).collect::<Vec<usize>>();
    loop {
        let current_answer = answer;
        // Create new bit vector and set of non-zero locations for next iteration
        let mut new_locations = locations.clone();
        let mut new_mask = Vec::new();
        
        for index in mask {;
            let row = index / num_cols;
            let col = index % num_cols;
            let mut neighbours = 0;
            
            for (dr, dc) in &combos {
                let new_row = row as isize + dr;
                let new_col = col as isize + dc;
                // Bounds check using signed values
                if new_row < 0 || new_col < 0 || new_row >= num_rows as isize || new_col >= num_cols as isize {
                    continue;
                }
                let neighbour_index = (new_row as usize) * num_cols + (new_col as usize);
                if locations[neighbour_index] == 1 {
                    neighbours += 1;
                }
            }
            if neighbours <= 3 {
                answer += 1;
                new_locations[index] = 0;
            } else {
                new_mask.push(index);
            }
        }
        if current_answer == answer {
            break;
        }
        locations = new_locations;
        mask = new_mask;
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
    fn test_part_1_fast_example() {
        assert_eq!(part_1_fast(include_str!("./example.txt")), 13);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 43);
    }

    #[test]
    fn test_part_2_fast_example() {
        assert_eq!(part_2_fast(include_str!("./example.txt")), 43);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let part_1_answer = part_1_fast(contents);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer = part_2_fast(contents);
    println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
