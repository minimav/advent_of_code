use std::time::Instant;

fn part_1(contents: &str) {
    let earliest_possible_timestamp: u32 = contents.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let bus_ids_raw = contents.lines().nth(1).unwrap();

    let mut earliest_bus_id = 0;
    let mut earliest_leave_timestamp = u32::MAX;
    for bus_id_raw in bus_ids_raw.split(",") {
        if bus_id_raw == "x" {
            continue;
        }
        let bus_id = bus_id_raw.parse::<u32>().unwrap();
        let next_timestamp =
            bus_id + earliest_possible_timestamp - (earliest_possible_timestamp % bus_id);
        if next_timestamp < earliest_leave_timestamp {
            earliest_bus_id = bus_id;
            earliest_leave_timestamp = next_timestamp
        }
    }
    println!(
        "Answer for part 1 is: {}",
        earliest_bus_id * (earliest_leave_timestamp - earliest_possible_timestamp)
    );
}

fn part_2(contents: &str) {
    println!("Answer for part 2 is: {}", 0);
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    part_1(contents);
    part_2(contents);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
