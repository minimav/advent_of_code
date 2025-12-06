use std::time::Instant;

fn part_1(contents: &str) -> u64 {
    let lines: Vec<&str> = contents.lines().collect();
    let num_lines = lines.len();
    let num_eqs = lines[0].split(" ").collect::<Vec<_>>().into_iter().filter(|x| !x.is_empty()).collect::<Vec<_>>().len();
    let mut equations: Vec<Vec<u64>> = vec![Vec::new(); num_eqs];
    let mut answer = 0;

    for line in lines {
        if !line.contains('*') {
            // Add numbers to relevant equations
            line.split(" ")
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u64>().unwrap())
            .enumerate()
            .for_each(|(i, val)| {
                equations[i].push(val);
            });
        } else {
            // Resolve each equation based on the given operator
            line.split(" ")
            .collect::<Vec<_>>()
            .iter()
            .filter(|x| !x.is_empty())
            .enumerate()
            .for_each(|(i, val)| {
                if *val == "*" {
                    answer += equations[i].iter().product::<u64>();
                } else {
                    answer += equations[i].iter().sum::<u64>();
                }
            });
        }
    }
    return answer;
}

fn part_2(contents: &str) -> u64 {
    let lines: Vec<&str> = contents.lines().collect();
    let num_cols = lines[0].len();
    let num_lines = lines.len();

    let mut flipped: Vec<String> = Vec::with_capacity(num_cols);
    for col in 0..num_cols {
        let mut new_line = String::new();
        for line in &lines {
            new_line.push(line.chars().nth(col).unwrap());
        }
        flipped.push(new_line);
    }

    let mut answer = 0;
    let mut equation: Vec<u64> = vec![];
    let mut op = '+';
    let blank_line = " ".repeat(num_lines);
    for col in 0..num_cols {
        let mut row = flipped[col].as_str();
        if *row == blank_line {
            continue;
        } else if row.contains('*') {
            op = '*';
            let num = &row[..row.len()-1].trim().parse::<u64>().unwrap();
            equation.push(*num);
        } else if row.contains('+') {
            op = '+';
            let num = &row[..row.len()-1].trim().parse::<u64>().unwrap();
            equation.push(*num);
        } else {
            let num = row.trim().parse::<u64>().unwrap();
            equation.push(num);
        }
        

        if (col == num_cols - 1) || (flipped[col + 1] == blank_line) {
            // Resolve previous equation
            if op == '*' {
                answer += equation.iter().product::<u64>();
            } else {
                answer += equation.iter().sum::<u64>();
            }
            // Reset for next equation
            equation = vec![];
            continue;
        }
    }
    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 4277556);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 3263827);
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
