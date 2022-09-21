use std::fmt;
use std::time::Instant;

static FLOOR: char = '.';
static EMPTY_SEAT: char = 'L';
static OCCUPIED_SEAT: char = '#';

#[derive(Debug)]
struct Grid {
    size: usize,
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(size: usize, data: &str) -> Grid {
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in data.lines() {
            let mut row: Vec<char> = Vec::new();
            for char in line.chars() {
                row.push(char);
            }
            grid.push(row)
        }
        Grid { size, grid }
    }

    fn up(&mut self, row: usize, col: usize) -> Option<char> {
        if row <= 0 {
            return None;
        }
        Some(self.grid[row - 1][col])
    }

    fn down(&mut self, row: usize, col: usize) -> Option<char> {
        if row >= self.size - 1 {
            return None;
        }
        Some(self.grid[row + 1][col])
    }

    fn left(&mut self, row: usize, col: usize) -> Option<char> {
        if col <= 0 {
            return None;
        }
        Some(self.grid[row][col - 1])
    }

    fn right(&mut self, row: usize, col: usize) -> Option<char> {
        if col >= self.size - 1 {
            return None;
        }
        Some(self.grid[row][col + 1])
    }

    fn up_left(&mut self, row: usize, col: usize) -> Option<char> {
        if col <= 0 || row <= 0 {
            return None;
        }
        Some(self.grid[row - 1][col - 1])
    }

    fn up_right(&mut self, row: usize, col: usize) -> Option<char> {
        if col >= self.size - 1 || row <= 0 {
            return None;
        }
        Some(self.grid[row - 1][col + 1])
    }

    fn down_left(&mut self, row: usize, col: usize) -> Option<char> {
        if col <= 0 || row >= self.size - 1 {
            return None;
        }
        Some(self.grid[row + 1][col - 1])
    }

    fn down_right(&mut self, row: usize, col: usize) -> Option<char> {
        if col >= self.size - 1 || row >= self.size - 1 {
            return None;
        }
        Some(self.grid[row + 1][col + 1])
    }

    fn adjacency_count(&mut self, row: usize, col: usize) -> u32 {
        let mut count: u32 = 0;
        match self.down(row, col) {
            Some(c) if c == OCCUPIED_SEAT => count += 1,
            _ => {}
        }
        match self.up(row, col) {
            Some(c) if c == OCCUPIED_SEAT => count += 1,
            _ => {}
        }
        match self.right(row, col) {
            Some(c) if c == OCCUPIED_SEAT => count += 1,
            _ => {}
        }
        match self.left(row, col) {
            Some(c) if c == OCCUPIED_SEAT => count += 1,
            _ => {}
        }
        match self.down_left(row, col) {
            Some(c) if c == OCCUPIED_SEAT => count += 1,
            _ => {}
        }
        match self.down_right(row, col) {
            Some(c) if c == OCCUPIED_SEAT => count += 1,
            _ => {}
        }
        match self.up_left(row, col) {
            Some(c) if c == OCCUPIED_SEAT => count += 1,
            _ => {}
        }
        match self.up_right(row, col) {
            Some(c) if c == OCCUPIED_SEAT => count += 1,
            _ => {}
        }
        count
    }

    fn num_seats_of_type(&mut self, seat_type: &char) -> u32 {
        let mut count: u32 = 0;
        for row in self.grid.iter() {
            for char in row.iter() {
                if char == seat_type {
                    count += 1
                }
            }
        }
        count
    }

    fn next_iteration(&mut self, row: usize, col: usize) -> char {
        let count: u32 = self.adjacency_count(row, col);
        let current: char = self.grid[row][col];
        return {
            if current == EMPTY_SEAT && count == 0 {
                OCCUPIED_SEAT
            } else if current == OCCUPIED_SEAT && count >= 4 {
                EMPTY_SEAT
            } else {
                current
            }
        };
    }

    fn iteration(&mut self) -> (Grid, bool) {
        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut changes = false;
        for row_index in 0..self.size {
            let mut row: Vec<char> = Vec::new();
            for col_index in 0..self.size {
                let new_char = self.next_iteration(row_index, col_index);
                row.push(new_char);
                let old_char = self.grid[row_index][col_index];
                if old_char != new_char {
                    changes = true
                }
            }
            grid.push(row)
        }
        let new_grid = Grid {
            size: self.size,
            grid,
        };
        (new_grid, changes)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string_rep: String = String::new();
        for row in self.grid.iter() {
            for char in row.iter() {
                string_rep.push(*char);
            }
            string_rep.push_str("\n");
        }
        write!(f, "{}", string_rep)
    }
}

fn part_1(contents: &str) {
    let size = contents.lines().count();
    let mut grid = Grid::new(size, contents);

    let mut iteration = 0;
    loop {
        let (new_grid, changes) = grid.iteration();
        grid = new_grid;
        //println!("{}\n{}", iteration, grid);
        if !changes {
            break;
        }
        iteration += 1;
    }

    println!(
        "Answer for part 1 is:\n{}",
        grid.num_seats_of_type(&OCCUPIED_SEAT)
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
