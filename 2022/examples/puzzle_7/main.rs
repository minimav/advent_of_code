use std::collections::HashMap;
use std::time::Instant;

fn get_folder_sizes(contents: &str) -> HashMap<String, u64> {
    let mut folder_sizes: HashMap<String, u64> = HashMap::new();
    let mut current_folder: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line == "$ ls" {
            continue;
        } else if line.starts_with("$ cd") {
            let components: Vec<&str> = line.split_whitespace().collect();
            let new_folder = components[2];
            if new_folder == ".." {
                current_folder.pop();
            } else {
                current_folder.push(new_folder);
            }
        } else if line.starts_with("dir") {
            // could add key now, but will wait till we traverse into it
            continue;
        } else {
            let components: Vec<&str> = line.split_whitespace().collect();
            let file_size = components[0].parse::<u64>().unwrap();

            for i in 0..current_folder.len() {
                let parent_folder = current_folder[0..=i].join("/");
                folder_sizes
                    .entry(parent_folder)
                    .and_modify(|e| *e += file_size)
                    .or_insert(file_size);
            }
        }
    }
    folder_sizes
}

fn part_1(contents: &str) -> u64 {
    let folder_sizes = get_folder_sizes(contents);
    let mut answer: u64 = 0;
    for (key, size) in folder_sizes {
        if size < 100_000 {
            answer += size
        }
    }
    answer
}

fn part_2(contents: &str) -> u64 {
    let folder_sizes = get_folder_sizes(contents);
    let total_space: u64 = 70_000_000;
    let unused_space_required: u64 = 30_000_000;
    let current_space_used = *folder_sizes.get("/").unwrap();
    let mut minimal_folder_size_to_remove: u64 = u64::MAX;
    for (folder, size) in folder_sizes {
        if total_space - (current_space_used - size) > unused_space_required
            && size < minimal_folder_size_to_remove
        {
            minimal_folder_size_to_remove = size;
        }
    }
    minimal_folder_size_to_remove
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 95437);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 24933642);
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
