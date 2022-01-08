use std::collections::HashSet;
use std::fs;
use std::time::Instant;

/* Product of the pair that sums to the target (part 1). */
fn find_product(filename: String, target: i32) -> Result<i32, String> {
    let contents = fs::read_to_string(filename).expect("Could not read file");

    let mut values = HashSet::<i32>::new();
    for raw_value in contents.lines() {
        let value: i32 = raw_value.trim().parse().unwrap();

        if values.contains(&(target - value)) {
            println!(
                "{} * {} = {}",
                target - value,
                value,
                (target - value) * value
            );
            return Ok((target - value) * value);
        } else {
            values.insert(value);
        }
    }
    return Err(format!("No values found which sum to {}", target));
}

/* Product of the triple that sums to the target (part 2). */
fn find_triple_product(filename: String, target: i32) -> Result<i32, String> {
    let contents = fs::read_to_string(filename).expect("Could not read file");

    let mut _groups = HashSet::<Vec<i32>>::new();
    for raw_value in contents.lines() {
        let mut new_groups = HashSet::<Vec<i32>>::new();
        let value: i32 = raw_value.trim().parse().unwrap();
        
        for group in _groups.iter() {
            let mut new_group = group.to_vec().clone();
            new_group.push(value);

            if new_group.len() == 3 {
                if new_group.iter().sum::<i32>() == target {
                    let answer = new_group.iter().product::<i32>();
                    println!("{:?} product is {}", new_group, answer);
                    return Ok(answer);
                }
            } else {
                new_groups.insert(new_group.to_vec());
            }
        }
        let initial_group = vec![value];
        new_groups.insert(initial_group);
        
        for group in new_groups.iter() {
            _groups.insert(group.to_vec());
        };
    }
    return Err(format!("No values found which sum to {}", target));
}


fn main() {
    let start = Instant::now();
    let filenames: Vec<String> = vec![
        "src/example_input.txt".to_string(),
        "src/input.txt".to_string(),
    ];
    let target: i32 = 2020;
    for filename in filenames {
        find_product(filename, target).expect("Input has no solution");
    }

    find_triple_product("src/input.txt".to_string(), target).expect("Input has no solution");

    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
