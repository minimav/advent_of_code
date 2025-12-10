use std::time::Instant;
use itertools::Itertools;
use nalgebra::{DMatrix, SVD};
use microlp::{Problem, OptimizationDirection, ComparisonOp};


fn find_minimal_presses(line: &str) -> u32 {
    let components: Vec<&str> = line.split(" ").collect();
    let raw_lights = components[0];
    let raw_buttons = &components[1..components.len() - 1];

    // Parse the target lights
    let mut power: u32 = 0;
    let mut lights: u32 = 0;
    for c in raw_lights[1..raw_lights.len() - 1].chars() {
        if c == '#' {
            lights += 2u32.pow(power)
        }
        power += 1;
    }

    // Parse the buttons we can press
    let mut buttons: Vec<u32> = Vec::new();
    for raw_button in raw_buttons {
        let mut button: u32 = 0;
        for c in raw_button[1..raw_button.len() - 1].chars() {
            if c == ',' {
                continue
            }
            button += 2u32.pow(c.to_digit(10).expect("..."))
        }
        buttons.push(button)
    }

    for combo in buttons.into_iter().powerset() {
        let length = combo.len();
        if combo.into_iter().reduce(|a,b| a ^ b) == Some(lights) {
            return length as u32
        }
    }
    return 0;
    
    /* Slower iniital implementation
    // General idea:
    // * start of with each number in a queue
    // * pop, push button
    // * if more steps than current best skip
    // * if new fastest to value, record, otherwise continue
    // * put new state on the queue    
    let mut queue: BinaryHeap<Reverse<(u32, u32)>> = {
        BinaryHeap::from([Reverse((0, 0))])
    };
    // Record best number of steps to a particular value
    let mut best_to_value: HashMap<u32, u32> = [(0, 0)].into();
    while queue.peek().is_some() {
        let Reverse(state) = queue.pop().unwrap();
        let (num_switches, current_lights) = state;
        
        for button in &buttons {
            let new_lights = current_lights ^ button;

             if num_switches + 1 >= *best_to_value.entry(new_lights).or_insert(u32::MAX) {
                continue;
            }

            best_to_value
                .entry(new_lights)
                .and_modify(|e| *e = num_switches + 1)
                .or_insert(num_switches + 1);

            queue.push(Reverse((num_switches + 1, new_lights)));
        }
    }
    return *best_to_value.get(&lights).unwrap();
    */
}

fn joltages(line: &str) -> f64 {
    let components: Vec<&str> = line.split(" ").collect();
    let raw_buttons = &components[1..components.len() - 1];
    let raw_jolts = components[components.len() - 1];

    let jolts = raw_jolts[1..raw_jolts.len() - 1].split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let num_rows = jolts.len();
    let num_columns = raw_buttons.len();
    let max_presses = jolts.iter().max().unwrap();

    let mut problem = Problem::new(OptimizationDirection::Minimize);
    
    let variables: Vec<_> = (0..raw_buttons.len()).map(
        |_| problem.add_integer_var(1.0, (0, *max_presses))
    ).collect();

    let mut constraints = vec![vec![]; num_rows];
    for (variable_index, raw_button) in raw_buttons.into_iter().enumerate() {
        for c in raw_button[1..raw_button.len() - 1].split(",") {
            let row_index = c.parse::<usize>().unwrap();
            constraints[row_index].push((variables[variable_index], 1.0))
        }
    }

    constraints.into_iter().enumerate().for_each(|(i, c)| {
        problem.add_constraint(c, ComparisonOp::Eq, jolts[i].into());
    });
    let solution = problem.solve().unwrap();
    //println!("{:?}", solution.objective());
    return solution.objective();

    /* Pseudo-inverse approach won't give integer solutions
    let jolts = raw_jolts[1..raw_jolts.len() - 1].split(",").map(|n| n.parse::<f64>().unwrap()).collect::<Vec<f64>>();
    let jolts_matrix = DMatrix::<f64>::from_column_slice(jolts.len(), 1, &jolts);

    // Parse the buttons into a matrix
    let mut buttons = DMatrix::<f64>::zeros(num_rows, num_columns);
    for (col_index, raw_button) in raw_buttons.into_iter().enumerate() {
        for c in raw_button[1..raw_button.len() - 1].split(",") {
            let row_index = c.parse::<usize>().unwrap();
            buttons[(row_index, col_index)] = 1.0;
        }
    }

    let buttons_svd = SVD::new(buttons, true, true);
    let buttons_inv = buttons_svd.pseudo_inverse(1e-5).unwrap();
    let result = buttons_inv * jolts_matrix;
    return result.sum();
    */
}

fn part_1(contents: &str) -> u32 {
    return contents.lines().map(|line| find_minimal_presses(line)).sum();
}

fn part_2(contents: &str) -> f64 {
    return contents.lines().map(|line| joltages(line)).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_single_line_1() {
        assert_eq!(find_minimal_presses("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"), 2);
    }

    #[test]
    fn test_part_1_single_line_2() {
        assert_eq!(find_minimal_presses("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"), 3);
    }

    #[test]
    fn test_part_1_single_line_3() {
        assert_eq!(find_minimal_presses("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"), 2);
    }

    #[test]
    fn test_part_1_single_line_4() {
        assert_eq!(find_minimal_presses("[##.#####..] (1,9) (0,3,7,8,9) (5,8,9) (0,1,3,7,9) (0,3,4,5) (2,3,5) (0,3,4,8) (2,3,4,5,6,7,8,9) (2,6) (0,1,2,3,6,8,9) {55,28,32,72,32,43,27,34,73,75}"), 8);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 7);
    }

    #[test]
    fn test_part_2_single_line_1() {
        assert_eq!(joltages("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"), 10.0);
    }

    #[test]
    fn test_part_2_single_line_2() {
        assert_eq!(joltages("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"), 12.0);
    }

    #[test]
    fn test_part_2_single_line_3() {
        assert_eq!(joltages("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"), 11.0);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 33.0);
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