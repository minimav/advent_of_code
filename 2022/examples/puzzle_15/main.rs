use parse_display::{Display, FromStr};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::{str::FromStr, time::Instant};

#[derive(Clone, Copy, Debug, Display, FromStr)]
#[display(r"Sensor at x={x}, y={y}")]
struct Sensor {
    x: i128,
    y: i128,
}

#[derive(Clone, Copy, Debug, Display, FromStr)]
#[display(r"closest beacon is at x={x}, y={y}")]
struct Beacon {
    x: i128,
    y: i128,
}

impl Sensor {
    fn manhattan_distance(&self, other: Beacon) -> i128 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn tuning_frequency(x: i128, y: i128) -> i128 {
    (x * 4_000_000) + y
}

fn part_1(contents: &str, row_index: i128) -> i128 {
    let mut beacon_xs: HashSet<i128> = HashSet::new();
    let mut not_possible_in_row: Vec<(i128, i128)> = Vec::new();
    for line in contents.lines() {
        let components = line.split(": ").collect::<Vec<&str>>();
        let sensor = Sensor::from_str(components[0]).unwrap();
        let beacon = Beacon::from_str(components[1]).unwrap();

        // record beacon x co-ordinates in row of interest for later
        if beacon.y == row_index {
            beacon_xs.insert(beacon.x);
        }

        // remove locations for this beacon sensor pair
        let distance = sensor.manhattan_distance(beacon);
        let y_diff = (sensor.y - row_index).abs();

        // doesn't stop anything on row being a beacon
        if distance < y_diff {
            continue;
        }
        // no equal distances!
        let max_x_diff = (distance - y_diff).abs();
        not_possible_in_row.push((sensor.x - max_x_diff, sensor.x + max_x_diff))
    }

    not_possible_in_row.sort();

    // merge ranges
    let mut merged_not_possible_in_row: Vec<(i128, i128)> = Vec::new();
    let mut current_range = not_possible_in_row[0];
    for next_range in not_possible_in_row.iter().skip(1) {
        if current_range.1 < next_range.0 {
            // no overlap with next range, includes everything inclusively
            merged_not_possible_in_row.push(current_range);
            current_range = *next_range;
        } else if current_range.1 < next_range.1 {
            current_range = (current_range.0, next_range.1);
        } else {
            // (1, 3) w/ (2, 2) => (1, 3) case
            continue;
        }
    }
    merged_not_possible_in_row.push(current_range);

    // remove beacons from ranges
    let mut split_not_possible_in_row: Vec<(i128, i128)> = Vec::new();
    for range in merged_not_possible_in_row.iter() {
        let mut beacons_in_range = beacon_xs
            .iter()
            .filter(|x| range.0 <= **x && **x <= range.1)
            .collect::<Vec<_>>();
        if beacons_in_range.len() == 0 {
            split_not_possible_in_row.push(*range);
            continue;
        }

        beacons_in_range.sort();
        if range.0 <= *beacons_in_range[0] - 1 {
            let first_range = (range.0, *beacons_in_range[0] - 1);
            split_not_possible_in_row.push(first_range);
        }
        let last = beacons_in_range.last().unwrap();
        if *last + 1 <= range.1 {
            let last_range = (*last + 1, range.1);
            split_not_possible_in_row.push(last_range);
        }

        for pair in beacons_in_range.windows(2) {
            let start = pair[0];
            let end = pair[1];
            if start == end || *start == end - 1 {
                continue;
            } else {
                split_not_possible_in_row.push((start + 1, end - 1));
            }
        }
    }

    let mut answer = 0;
    for range in split_not_possible_in_row.iter() {
        answer += range.1 - range.0 + 1;
    }
    answer
}

fn part_2(contents: &str) -> i128 {
    let min_x = 0;
    let max_x = 4_000_000;
    let mut sensor_data: Vec<(Sensor, i128)> = Vec::new();
    for line in contents.lines() {
        let components = line.split(": ").collect::<Vec<&str>>();
        let sensor = Sensor::from_str(components[0]).unwrap();
        let beacon = Beacon::from_str(components[1]).unwrap();
        let distance = sensor.manhattan_distance(beacon);
        sensor_data.push((sensor, distance))
    }
    for row_index in min_x..=max_x {
        let mut not_possible_in_row: Vec<(i128, i128)> = Vec::new();
        for (sensor, distance) in sensor_data.iter() {
            let y_diff = (sensor.y - row_index).abs();

            // doesn't stop anything on row being a beacon
            if distance < &y_diff {
                continue;
            }
            // no equal distances!
            let max_x_diff = (distance - y_diff).abs();
            not_possible_in_row.push((
                max(min_x, sensor.x - max_x_diff),
                min(max_x, sensor.x + max_x_diff),
            ))
        }

        not_possible_in_row.sort();

        // merge ranges
        let mut merged_not_possible_in_row: Vec<(i128, i128)> = Vec::new();
        let mut current_range = not_possible_in_row[0];
        for next_range in not_possible_in_row.iter().skip(1) {
            if current_range.1 < next_range.0 {
                // no overlap with next range, includes everything inclusively
                merged_not_possible_in_row.push(current_range);
                current_range = *next_range;
            } else if current_range.1 < next_range.1 {
                current_range = (current_range.0, next_range.1);
            } else {
                // (1, 3) w/ (2, 2) => (1, 3) case
                continue;
            }
        }
        merged_not_possible_in_row.push(current_range);

        // check if beacon can be on this row
        // if on row, merging should not make one continuous range
        if merged_not_possible_in_row.len() > 1 {
            let x = merged_not_possible_in_row[0].1 + 1;
            return tuning_frequency(x, row_index);
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt"), 10), 26);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 56000011);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let part_1_answer = part_1(contents, 2_000_000);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer = part_2(contents);
    println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
