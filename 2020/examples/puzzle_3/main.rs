use std::cmp;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn num_trees_for_slope(
    trees: &HashSet<(usize, usize)>,
    step: (usize, usize),
    num_rows: usize,
    num_cols: usize,
) -> usize {
    let mut location: (usize, usize) = (0, 0);
    let mut num_trees: usize = 0;
    loop {
        location = (location.0 + step.0, location.1 + step.1);

        // deal with right checkerboard-ing when seeing if there is a tree at location
        let truncated_location = (location.0, location.1 % num_cols);
        if trees.contains(&truncated_location) {
            num_trees += 1;
        }
        // off bottom of the grid
        if location.0 >= num_rows {
            break;
        }
    }
    num_trees
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string("examples/puzzle_3/input.txt").expect("Could not read file");
    let mut trees = HashSet::<(usize, usize)>::new();
    let mut max_row_index: usize = 0;
    let mut max_col_index: usize = 0;
    for (row_index, line) in contents.lines().enumerate() {
        max_row_index = cmp::max(row_index, max_row_index);
        if row_index == 0 {
            max_col_index = line.len() - 1;
        }

        for (col_index, location) in line.chars().enumerate() {
            if location == '#' {
                trees.insert((row_index, col_index));
            }
        }
    }

    let slopes: [(usize, usize); 5] = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let mut answer: usize = 1;
    for slope in slopes.iter() {
        let num_trees: usize =
            num_trees_for_slope(&trees, *slope, max_row_index + 1, max_col_index + 1);
        println!("{:?} -> {}", slope, num_trees);
        answer *= num_trees;
    }
    println!("{}", answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
