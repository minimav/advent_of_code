use std::time::Instant;
use std::collections::HashMap;

fn part_1(contents: &str, num_pairs: usize) -> u64 {
    let lines: Vec<&str> = contents.lines().collect();
    let mut coords: Vec<Vec<i64>> = Vec::new();
    for line in lines {
        let coord = line.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
        coords.push(coord);
    }

    let mut distances = Vec::<((usize, usize), i64)>::new();
    for i in 0..coords.len() {
        let coord_1 = &coords[i];
        for j in i+1..coords.len() {
            let coord_2 = &coords[j];
            let dist = (coord_1[0] - coord_2[0]).pow(2) + (coord_1[1] - coord_2[1]).pow(2) + (coord_1[2] - coord_2[2]).pow(2);
            distances.push(((i, j), dist));
        }
    }
    distances.sort_by(|a, b| a.1.cmp(&b.1));

    let mut next_group = 1;
    let mut groups = vec![0; coords.len()];
    for ((i, j), _) in &distances[..num_pairs] {
        if groups[*i] == 0 && groups[*j] == 0 {
            groups[*i] = next_group;
            groups[*j] = next_group;
            next_group += 1;
        } else if groups[*i] != 0 && groups[*j] == 0 {
            groups[*j] = groups[*i];
        } else if groups[*i] == 0 && groups[*j] != 0 {
            groups[*i] = groups[*j];
        } else if groups[*i] != groups[*j] {
            let old_group = groups[*j];
            let new_group = groups[*i];
            for g in &mut groups {
                if *g == old_group {
                    *g = new_group;
                }
            }
        }
    }

    let mut group_sizes: HashMap<usize, usize> = HashMap::new();
    for g in groups {
        if g == 0 {
            continue;
        }
        *group_sizes.entry(g).or_default() += 1;
    }
    let mut largest: Vec<_> = group_sizes.into_iter().map(|p| p.1).collect();
    largest.sort_by(|a, b| b.cmp(&a));

    return (largest[0] as u64) * (largest[1] as u64) * (largest[2] as u64);
}

fn part_2(contents: &str) -> u64 {
    let lines: Vec<&str> = contents.lines().collect();
    let mut coords: Vec<Vec<i64>> = Vec::new();
    for line in lines {
        let coord = line.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
        coords.push(coord);
    }

    let mut distances = Vec::<((usize, usize), i64)>::new();
    for i in 0..coords.len() {
        let coord_1 = &coords[i];
        for j in i+1..coords.len() {
            let coord_2 = &coords[j];
            let dist = (coord_1[0] - coord_2[0]).pow(2) + (coord_1[1] - coord_2[1]).pow(2) + (coord_1[2] - coord_2[2]).pow(2);
            distances.push(((i, j), dist));
        }
    }
    distances.sort_by(|a, b| a.1.cmp(&b.1));

    let mut num_groups = 0;
    let mut next_group = 1;
    let mut num_zeros = coords.len();
    let mut groups = vec![0; coords.len()];
    for ((i, j), _) in &distances {
        if groups[*i] == 0 && groups[*j] == 0 {
            groups[*i] = next_group;
            groups[*j] = next_group;
            next_group += 1;
            num_groups += 1;
            num_zeros -= 2;
        } else if groups[*i] != 0 && groups[*j] == 0 {
            groups[*j] = groups[*i];
            num_zeros -= 1;
        } else if groups[*i] == 0 && groups[*j] != 0 {
            groups[*i] = groups[*j];
            num_zeros -= 1;
        } else if groups[*i] != groups[*j] {
            num_groups -= 1;

            let old_group = groups[*j];
            let new_group = groups[*i];
            for g in &mut groups {
                if *g == old_group {
                    *g = new_group;
                }
            }
        }

        // Check exit condition of returning to a single group *and* that
        // everything should be assigned to a group
        if num_groups == 1 && num_zeros == 0 {
            return (coords[*i][0] * coords[*j][0]) as u64;
        } 
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt"), 10), 40);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 25272);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let part_1_answer = part_1(contents, 1000);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_1_finish = start.elapsed();
    println!("Took {:?} to solve part 1 of this puzzle", part_1_finish);
    let part_2_answer = part_2(contents);
    println!("Answer for part 2 is: {}", part_2_answer);
    let part_2_finish = start.elapsed();
    println!("Took {:?} to solve part 2 of this puzzle", part_2_finish);
}
