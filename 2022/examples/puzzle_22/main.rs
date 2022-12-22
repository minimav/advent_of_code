use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Facing {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

enum Direction {
    LEFT,
    RIGHT,
}

impl Default for Facing {
    fn default() -> Self {
        Facing::RIGHT
    }
}

impl Facing {
    fn value(&self) -> usize {
        match self {
            Facing::LEFT => 2,
            Facing::RIGHT => 0,
            Facing::UP => 3,
            Facing::DOWN => 1,
        }
    }
    fn rotate(&self, rotation_direction: Direction) -> Facing {
        match (rotation_direction, self) {
            (Direction::LEFT, Facing::LEFT) => Facing::DOWN,
            (Direction::LEFT, Facing::RIGHT) => Facing::UP,
            (Direction::LEFT, Facing::DOWN) => Facing::RIGHT,
            (Direction::LEFT, Facing::UP) => Facing::LEFT,
            (Direction::RIGHT, Facing::LEFT) => Facing::UP,
            (Direction::RIGHT, Facing::RIGHT) => Facing::DOWN,
            (Direction::RIGHT, Facing::DOWN) => Facing::LEFT,
            (Direction::RIGHT, Facing::UP) => Facing::RIGHT,
        }
    }
}

fn move_around_maze(
    position: (usize, usize),
    move_by: usize,
    facing: &Facing,
    maze: &HashMap<(usize, usize), char>,
    maze_bounds: &MazeBounds,
) -> (usize, usize) {
    match facing {
        Facing::LEFT => {
            let mut new_x = position.0;
            let mut left_to_move = move_by;
            let mut previous_x = position.0;
            while left_to_move > 0 {
                left_to_move -= 1;
                if maze_bounds.min_per_row.get(&position.1).unwrap() == &new_x {
                    previous_x = new_x;
                    new_x = *maze_bounds.max_per_row.get(&position.1).unwrap();
                } else {
                    previous_x = new_x;
                    new_x -= 1;
                }
                match maze.get(&(new_x, position.1)) {
                    Some('.') => {
                        continue;
                    }
                    Some('#') => return (previous_x, position.1),
                    _ => panic!("Should not occur!"),
                }
            }
            (new_x, position.1)
        }
        Facing::RIGHT => {
            let mut new_x = position.0;
            let mut left_to_move = move_by;
            let mut previous_x = position.0;
            while left_to_move > 0 {
                left_to_move -= 1;
                if maze_bounds.max_per_row.get(&position.1).unwrap() == &new_x {
                    previous_x = new_x;
                    new_x = *maze_bounds.min_per_row.get(&position.1).unwrap();
                } else {
                    previous_x = new_x;
                    new_x += 1;
                }
                match maze.get(&(new_x, position.1)) {
                    Some('.') => {
                        continue;
                    }
                    Some('#') => return (previous_x, position.1),
                    _ => panic!("Should not occur!"),
                }
            }
            (new_x, position.1)
        }
        Facing::UP => {
            let mut new_y = position.1;
            let mut left_to_move = move_by;
            let mut previous_y = position.1;
            while left_to_move > 0 {
                left_to_move -= 1;
                if maze_bounds.min_per_column.get(&position.0).unwrap() == &new_y {
                    previous_y = new_y;
                    new_y = *maze_bounds.max_per_column.get(&position.0).unwrap();
                } else {
                    previous_y = new_y;
                    new_y -= 1;
                }
                match maze.get(&(position.0, new_y)) {
                    Some('.') => {
                        continue;
                    }
                    Some('#') => return (position.0, previous_y),
                    _ => panic!("Should not occur!"),
                }
            }
            (position.0, new_y)
        }
        Facing::DOWN => {
            let mut new_y = position.1;
            let mut left_to_move = move_by;
            let mut previous_y = position.1;
            while left_to_move > 0 {
                left_to_move -= 1;
                if maze_bounds.max_per_column.get(&position.0).unwrap() == &new_y {
                    previous_y = new_y;
                    new_y = *maze_bounds.min_per_column.get(&position.0).unwrap();
                } else {
                    previous_y = new_y;
                    new_y += 1;
                }
                match maze.get(&(position.0, new_y)) {
                    Some('.') => {
                        continue;
                    }
                    Some('#') => return (position.0, previous_y),
                    _ => panic!("Should not occur!"),
                }
            }
            (position.0, new_y)
        }
    }
}

fn password(row_index: usize, column_index: usize, facing: Facing) -> u64 {
    ((row_index + 1) * 1000 + (column_index + 1) * 4 + facing.value()) as u64
}

#[derive(Debug)]
struct MazeBounds {
    max_per_row: HashMap<usize, usize>,
    max_per_column: HashMap<usize, usize>,
    min_per_row: HashMap<usize, usize>,
    min_per_column: HashMap<usize, usize>,
}

fn part_1(contents: &str) -> u64 {
    let mut maze: HashMap<(usize, usize), char> = HashMap::new();
    let mut max_per_row: HashMap<usize, usize> = HashMap::new();
    let mut max_per_column: HashMap<usize, usize> = HashMap::new();
    let mut min_per_row: HashMap<usize, usize> = HashMap::new();
    let mut min_per_column: HashMap<usize, usize> = HashMap::new();
    for (row_index, line) in contents.lines().take_while(|x| !x.is_empty()).enumerate() {
        for (column_index, char) in line.chars().enumerate() {
            if char != ' ' {
                maze.insert((column_index, row_index), char);
                max_per_column
                    .entry(column_index)
                    .and_modify(|x| {
                        if x < &mut row_index.clone() {
                            *x = row_index
                        }
                    })
                    .or_insert(row_index);
                min_per_column
                    .entry(column_index)
                    .and_modify(|x| {
                        if x > &mut row_index.clone() {
                            *x = row_index
                        }
                    })
                    .or_insert(row_index);
                max_per_row
                    .entry(row_index)
                    .and_modify(|x| {
                        if x < &mut column_index.clone() {
                            *x = column_index
                        }
                    })
                    .or_insert(column_index);
                min_per_row
                    .entry(row_index)
                    .and_modify(|x| {
                        if x > &mut column_index.clone() {
                            *x = column_index
                        }
                    })
                    .or_insert(column_index);
            }
        }
    }
    let maze_bounds = MazeBounds {
        min_per_column,
        min_per_row,
        max_per_column,
        max_per_row,
    };

    let mut facing = Facing::default();
    let mut position = (*maze_bounds.min_per_row.get(&0).unwrap(), 0);

    let mut instructions = contents
        .lines()
        .skip_while(|x| !x.is_empty())
        .skip(1)
        .next()
        .unwrap()
        .chars()
        .peekable();

    loop {
        match instructions.peek() {
            Some('L') => {
                facing = facing.rotate(Direction::LEFT);
                instructions.next();
            }
            Some('R') => {
                facing = facing.rotate(Direction::RIGHT);
                instructions.next();
            }
            Some(digit) => {
                let mut move_by: usize = digit.to_digit(10).unwrap() as usize;
                instructions.next();
                match instructions.peek() {
                    Some('L') | Some('R') | None => {
                        position =
                            move_around_maze(position, move_by, &facing, &maze, &maze_bounds);
                        println!("At {position:?} after move");
                    }
                    Some(next_digit) => {
                        move_by *= 10;
                        move_by += next_digit.to_digit(10).unwrap() as usize;
                        position =
                            move_around_maze(position, move_by, &facing, &maze, &maze_bounds);
                        instructions.next();
                    }
                    _ => break,
                }
            }
            _ => break,
        }
    }
    password(position.1, position.0, facing)
}

fn move_around_maze_cube(
    position: (usize, usize),
    facing: &Facing,
    maze: &HashMap<(usize, usize), char>,
    maze_bounds: &MazeBounds,
) -> Option<((usize, usize), Facing)> {
    Some(((0, 0), Facing::RIGHT))
}

fn part_2(contents: &str, cube_size: usize) -> u64 {
    let mut maze: HashMap<(usize, usize), char> = HashMap::new();
    let mut max_per_row: HashMap<usize, usize> = HashMap::new();
    let mut max_per_column: HashMap<usize, usize> = HashMap::new();
    let mut min_per_row: HashMap<usize, usize> = HashMap::new();
    let mut min_per_column: HashMap<usize, usize> = HashMap::new();
    for (row_index, line) in contents.lines().take_while(|x| !x.is_empty()).enumerate() {
        for (column_index, char) in line.chars().enumerate() {
            if char != ' ' {
                maze.insert((column_index, row_index), char);
                max_per_column
                    .entry(column_index)
                    .and_modify(|x| {
                        if x < &mut row_index.clone() {
                            *x = row_index
                        }
                    })
                    .or_insert(row_index);
                min_per_column
                    .entry(column_index)
                    .and_modify(|x| {
                        if x > &mut row_index.clone() {
                            *x = row_index
                        }
                    })
                    .or_insert(row_index);
                max_per_row
                    .entry(row_index)
                    .and_modify(|x| {
                        if x < &mut column_index.clone() {
                            *x = column_index
                        }
                    })
                    .or_insert(column_index);
                min_per_row
                    .entry(row_index)
                    .and_modify(|x| {
                        if x > &mut column_index.clone() {
                            *x = column_index
                        }
                    })
                    .or_insert(column_index);
            }
        }
    }
    let maze_bounds = MazeBounds {
        min_per_column,
        min_per_row,
        max_per_column,
        max_per_row,
    };

    let mut facing = Facing::default();
    let mut position = (*maze_bounds.min_per_row.get(&0).unwrap(), 0);

    let mut instructions = contents
        .lines()
        .skip_while(|x| !x.is_empty())
        .skip(1)
        .next()
        .unwrap()
        .chars()
        .peekable();

    loop {
        match instructions.peek() {
            Some('L') => {
                facing = facing.rotate(Direction::LEFT);
                instructions.next();
            }
            Some('R') => {
                facing = facing.rotate(Direction::RIGHT);
                instructions.next();
            }
            Some(digit) => {
                let mut move_by: usize = digit.to_digit(10).unwrap() as usize;
                instructions.next();
                match instructions.peek() {
                    Some('L') | Some('R') | None => {
                        for _ in 0..move_by {
                            match move_around_maze_cube(position, &facing, &maze, &maze_bounds) {
                                Some((p, f)) => {
                                    position = p;
                                    facing = f;
                                }
                                _ => break,
                            }
                        }
                        println!("At {position:?} after move");
                    }
                    Some(next_digit) => {
                        move_by *= 10;
                        move_by += next_digit.to_digit(10).unwrap() as usize;
                        for _ in 0..move_by {
                            match move_around_maze_cube(position, &facing, &maze, &maze_bounds) {
                                Some((p, f)) => {
                                    position = p;
                                    facing = f;
                                }
                                _ => break,
                            }
                        }
                        println!("At {position:?} after move");
                        instructions.next();
                    }
                    _ => break,
                }
            }
            _ => break,
        }
    }
    password(position.1, position.0, facing)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 6032);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt"), 4), 5031);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let part_1_answer = part_1(contents);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer = part_2(contents, 50);
    println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
