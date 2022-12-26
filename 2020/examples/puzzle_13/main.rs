use aoc_utils::number_theory::extended_euclidean_algorithm;
use std::time::Instant;

fn part_1(contents: &str) {
    let earliest_possible_timestamp: i128 =
        contents.lines().nth(0).unwrap().parse::<i128>().unwrap();
    let bus_ids_raw = contents.lines().nth(1).unwrap();

    let mut earliest_bus_id = 0;
    let mut earliest_leave_timestamp = i128::MAX;
    for bus_id_raw in bus_ids_raw.split(",") {
        if bus_id_raw == "x" {
            continue;
        }
        let bus_id = bus_id_raw.parse::<i128>().unwrap();
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

struct ModuloCondition {
    n: i128,
    rem: i128,
}

/* Essentially the Chinese Remainder Theorem */
fn part_2(bus_ids_raw: &str) -> i128 {
    let mut conditions: Vec<ModuloCondition> = Vec::new();
    for (index, bus_id_raw) in bus_ids_raw.split(",").enumerate() {
        if bus_id_raw == "x" {
            continue;
        }
        let bus_id = bus_id_raw.parse::<i128>().unwrap();
        conditions.push(ModuloCondition {
            n: bus_id,
            rem: index as i128,
        })
    }
    // compute miss one out products
    let product: i128 = conditions.iter().map(|condition| condition.n).product();
    let Ns: Vec<i128> = conditions
        .iter()
        .map(|condition| (product / condition.n) as i128)
        .collect();

    // find M such that M * N + m * n = 1
    let Ms: Vec<i128> = conditions
        .iter()
        .zip(Ns.iter())
        .map(|(condition, N)| extended_euclidean_algorithm(condition.n, *N).0)
        .collect();

    let chinese_remainder_answer: i128 = conditions
        .iter()
        .zip(Ms.iter())
        .zip(Ns.iter())
        .map(|((condition, M), N)| condition.rem * M * N)
        .sum();

    chinese_remainder_answer % product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(part_2("17,x,13,19"), 3417);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(part_2("67,7,59,61"), 754018);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(part_2("67,x,7,59,61"), 779210);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(part_2("67,7,x,59,61"), 1261476);
    }

    #[test]
    fn test_case_5() {
        assert_eq!(part_2("1789,37,47,1889"), 1202161486);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    part_1(contents);

    let bus_ids_raw = contents.lines().nth(1).unwrap();
    let answer = part_2(bus_ids_raw);
    println!("Answer for part 2 is: {}", answer);

    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
