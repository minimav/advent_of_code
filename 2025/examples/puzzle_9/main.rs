use std::time::Instant;


fn part_1(contents: &str) -> i64 {
    let lines: Vec<&str> = contents.lines().collect();
    let mut coords: Vec<[i64; 2]> = Vec::new();
    for line in lines {
        let coord_v = line.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
        let coord: [i64; 2] = [coord_v[0], coord_v[1]];
        coords.push(coord);
    }

    let mut max_area = 0;
    for i in 0..coords.len() {
        let coord_1 = &coords[i];
        for j in i+1..coords.len() {
            let coord_2 = &coords[j];
            let area = ((coord_1[0] - coord_2[0]).abs() + 1) * ((coord_1[1] - coord_2[1]).abs() + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }
    return max_area;
}

fn part_2(contents: &str) -> i64 {
    let lines: Vec<&str> = contents.lines().collect();
    let mut coords: Vec<[i64; 2]> = Vec::new();
    for line in lines {
        let coord_v = line.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
        let coord: [i64; 2] = [coord_v[0], coord_v[1]];
        coords.push(coord);
    }

    let mut max_area = 0;
    for i in 0..coords.len() {
        let coord_1 = &coords[i];
        for j in i+1..coords.len() {
            let coord_2 = &coords[j];

            let min_x = std::cmp::min(coord_1[0], coord_2[0]);
            let max_x = std::cmp::max(coord_1[0], coord_2[0]);
            let min_y = std::cmp::min(coord_1[1], coord_2[1]);
            let max_y = std::cmp::max(coord_1[1], coord_2[1]);
            
            // check if this rectangle is valid by confirming no border line is
            // intersected with
            let mut valid = true;
            for k in 0..coords.len() {
                // these two points define a line segment on the border
                let b1 = &coords[k];
                let b2 = if k == coords.len() - 1 { &coords[0] } else { &coords[k + 1] };

                if (b1[0] == b2[0]) && (min_x < b1[0]) && (b1[0] < max_x) {
                    // vertical case
                    if (b1[1] < b2[1]) && (b1[1] < max_y) && (b2[1] > min_y) {
                        valid = false;
                        break;
                    } else if (b2[1] < b1[1]) && (b2[1] < max_y) && (b1[1] > min_y) {
                        valid = false;
                        break;
                    }
                } else if (b1[1] == b2[1]) && (min_y < b1[1]) && (b1[1] < max_y) {
                    // horizontal case
                    if (b1[0] < b2[0]) && (b1[0] < max_x) && (b2[0] > min_x) {
                        valid = false;
                        break;
                    } else if (b2[0] < b1[0]) && (b2[0] < max_x) && (b1[0] > min_x){
                        valid = false;
                        break;
                    }
                }
            }
            if !valid {
                //println!("Invalid {:?} --> {:?}", coord_1, coord_2);
                continue
            }

            let area = ((coord_1[0] - coord_2[0]).abs() + 1) * ((coord_1[1] - coord_2[1]).abs() + 1);
            //println!("VALID {:?} --> {:?} ({})", coord_1, coord_2, area);
            if area > max_area {
                max_area = area;
            }
        }
    }
    return max_area;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 50);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 24);
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
    println!("Took {:?} to solve puzzle", duration);
}
