use std::time::Instant;

fn solve_line(line: &str, sizes: &Vec<u32>) -> u32 {
    let comps: Vec<&str> = line.split(": ").collect();
    let grid_size: Vec<u32> = comps[0].split("x").map(|x| x.parse::<u32>().unwrap()).collect();
    let counts: Vec<u32> = comps[1].split(" ").map(|x| x.parse::<u32>().unwrap()).collect();
    
    // Too small check - if there are more squares required than the grid size
    // then even perfect tesselatation into the grid would not give enough space
    let area = grid_size[0] * grid_size[1];
    let minimal_required_area: u32 = counts.iter().enumerate().map(|(i, c)| c * sizes[i]).sum();
    if minimal_required_area > area {
        return 0;
    }

    // Big enough check - the grid is naively big enough if we can put each
    // block into a separate 3x3 space without having to do any tessalation 
    let total_tiles = counts.iter().sum();
    if (grid_size[0] / 3) * (grid_size[1] / 3) >= total_tiles {
        return 1;
    }
    panic!("This is too hard to solve...");
}

fn part_1(contents: &str) -> u32 {
    let lines: Vec<&str> = contents.lines().collect();
    let mut sizes: Vec<u32> = Vec::new();
    let mut current_size = 0;
    let mut first_solve = false;
    let mut answer = 0;
    for line in lines.iter().skip(1) {
        if line.is_empty() { continue }
        else if line.contains('x') {
            if !first_solve {
                sizes.push(current_size);
                first_solve = true;
            }
            answer += solve_line(line, &sizes);
        } else if line.contains(':') {
            sizes.push(current_size);
            current_size = 0;
        } else {
            for c in line.chars() {
                if c == '#' {
                    current_size += 1;
                }
            }
        }     
    }
    return answer;
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let part_1_answer = part_1(contents);
    println!("Answer for part 1 is: {}", part_1_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve puzzle", duration);
}
