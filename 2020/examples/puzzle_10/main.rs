use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn parse_jolts(contents: &str) -> Vec<u32> {
    let mut jolts: Vec<u32> = vec![0];
    for line in contents.lines() {
        let jolt = line.parse::<u32>().unwrap();
        jolts.push(jolt);
    }
    jolts.sort();
    // add in extra jolt with fixed difference
    jolts.push(jolts[jolts.len() - 1] + 3);
    return jolts;
}

/* Count the jolt differences when moving from one to the next, no skipping */
fn part_1(contents: &str) {
    let jolts = parse_jolts(contents);
    let mut diffs: HashMap<u32, u32> = HashMap::new();
    let mut current_index: usize = 1;
    while current_index < jolts.len() {
        let diff = jolts[current_index] - jolts[current_index - 1];
        diffs.entry(diff).and_modify(|e| *e += 1).or_insert(1);
        current_index += 1;
    }
    println!("Answer for part 1 is: {}", diffs[&1] * diffs[&3]);
}

/* Count all valid sets of adapters, skipping allowed up to +3. */
fn part_2(contents: &str) {
    let jolts = parse_jolts(contents);
    let mut adapter_set_count_for_index: Vec<u64> = vec![0; jolts.len()];
    // seed the count with the 1 set to index 0 (with jolt 0)
    adapter_set_count_for_index[0] = 1;

    for (index, jolt) in jolts.iter().enumerate() {
        if index == 0 {
            continue;
        }
        // deal with -1, -2 and -3 index cases to see how many can reach this jolt
        for offset in 1..4 {
            if index < offset {
                break;
            }
            let other_index = index - offset;
            let other_jolt = jolts[other_index];
            if jolt - other_jolt > 3 {
                break;
            }
            let other_count = adapter_set_count_for_index[other_index];
            adapter_set_count_for_index[index] += other_count;
        }
    }
    println!(
        "Answer for part 2 is: {}",
        adapter_set_count_for_index[jolts.len() - 1]
    );
}

fn main() {
    let start = Instant::now();
    let file_names: [&str; 3] = [
        "examples/puzzle_10/example_1.txt",
        "examples/puzzle_10/example_2.txt",
        "examples/puzzle_10/input.txt",
    ];
    for file_name in file_names.iter() {
        println!("FILE: {}", file_name);
        let contents = fs::read_to_string(file_name).expect("Could not read file");
        part_1(&contents);
        part_2(&contents);
        let duration = start.elapsed();
        println!("Took {:?} to solve this puzzle", duration);
    }
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
