use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug)]
enum Status {
    FLOOR,
    WALL,
    MOVE_OK,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn down(&mut self) -> Status {
        if self.y > 0 {
            self.y -= 1;
            Status::MOVE_OK
        } else {
            Status::FLOOR
        }
    }

    fn up(&mut self) -> Status {
        self.y += 1;
        Status::MOVE_OK
    }

    fn left(&mut self) -> Status {
        if self.x > 0 {
            self.x -= 1;
            Status::MOVE_OK
        } else {
            Status::WALL
        }
    }

    fn right(&mut self) -> Status {
        if self.x < 6 {
            self.x += 1;
            Status::MOVE_OK
        } else {
            Status::WALL
        }
    }
}

#[derive(Debug)]
enum BlockType {
    Horizontal,
    Cross,
    ReverseL,
    Vertical,
    Square,
}

#[derive(Debug)]
struct Block {
    block_type: BlockType,
    points: Vec<Point>,
}

impl Block {
    fn bottom_at_height_for_index(height: i32, index: usize) -> Self {
        if index % 5 == 0 {
            Block {
                block_type: BlockType::Horizontal,
                points: vec![
                    Point::new(2, height),
                    Point::new(3, height),
                    Point::new(4, height),
                    Point::new(5, height),
                ],
            }
        } else if index % 5 == 1 {
            Block {
                block_type: BlockType::Cross,
                points: vec![
                    Point::new(2, height + 1),
                    Point::new(3, height),
                    Point::new(3, height + 1),
                    Point::new(3, height + 2),
                    Point::new(4, height + 1),
                ],
            }
        } else if index % 5 == 2 {
            Block {
                block_type: BlockType::ReverseL,
                points: vec![
                    Point::new(2, height),
                    Point::new(3, height),
                    Point::new(4, height),
                    Point::new(4, height + 1),
                    Point::new(4, height + 2),
                ],
            }
        } else if index % 5 == 3 {
            Block {
                block_type: BlockType::Vertical,
                points: vec![
                    Point::new(2, height),
                    Point::new(2, height + 1),
                    Point::new(2, height + 2),
                    Point::new(2, height + 3),
                ],
            }
        } else {
            Block {
                block_type: BlockType::Square,
                points: vec![
                    Point::new(2, height),
                    Point::new(3, height),
                    Point::new(2, height + 1),
                    Point::new(3, height + 1),
                ],
            }
        }
    }

    fn down(&mut self) -> Status {
        let mut new_points: Vec<Point> = Vec::new();
        for point in self.points.iter() {
            let mut new_point = point.clone();
            match new_point.down() {
                Status::MOVE_OK => new_points.push(new_point),
                other_status => return other_status,
            };
        }
        self.points = new_points;
        Status::MOVE_OK
    }

    fn up(&mut self) -> Status {
        let mut new_points: Vec<Point> = Vec::new();
        for point in self.points.iter() {
            let mut new_point = point.clone();
            match new_point.up() {
                Status::MOVE_OK => new_points.push(new_point),
                other_status => return other_status,
            };
        }
        self.points = new_points;
        Status::MOVE_OK
    }

    fn left(&mut self) -> Status {
        let mut new_points: Vec<Point> = Vec::new();
        for point in self.points.iter() {
            let mut new_point = point.clone();
            match new_point.left() {
                Status::MOVE_OK => new_points.push(new_point),
                other_status => return other_status,
            };
        }
        self.points = new_points;
        Status::MOVE_OK
    }

    fn right(&mut self) -> Status {
        let mut new_points: Vec<Point> = Vec::new();
        for point in self.points.iter() {
            let mut new_point = point.clone();
            match new_point.right() {
                Status::MOVE_OK => new_points.push(new_point),
                other_status => return other_status,
            };
        }
        self.points = new_points;
        Status::MOVE_OK
    }
}

fn insert_points(points: Vec<Point>, rocks: &mut HashSet<Point>, highest_rock: &mut i32) {
    for point in points.iter() {
        if point.y > *highest_rock {
            *highest_rock = point.y;
        }
        rocks.insert(*point);
    }
}

fn print_rocks(rocks: &HashSet<Point>, block: &Block, highest_rock: &i32) {
    let mut rocks_fmt: Vec<Vec<char>> = Vec::new();

    // 10 buffer needed for block height + start above current highest block
    let num_rows = (*highest_rock + 10).max(10);
    for _ in 0..num_rows {
        rocks_fmt.push(vec!['.'; 7]);
    }
    for rock in rocks.iter() {
        rocks_fmt[rock.y as usize][rock.x as usize] = '#'
    }
    for point in block.points.iter() {
        rocks_fmt[point.y as usize][point.x as usize] = '@'
    }
    for row in rocks_fmt.iter().rev() {
        for char in row.iter() {
            print!("{char}");
        }
        print!("\n");
    }
}

fn part_1(contents: &str) -> i32 {
    // 7 wide, appear left 2 in from edge, bottom 4 (in my index units) above tallest
    let mut highest_rock = -1;
    let mut rocks: HashSet<Point> = HashSet::new();
    let mut rock_index = 0;
    let mut block = Block::bottom_at_height_for_index(highest_rock + 4, rock_index);
    for jet in contents.replace(" \n", "").chars().cycle() {
        if rock_index == 2022 {
            break;
        }
        // pushed by rock
        let status = if jet == '<' {
            block.left()
        } else {
            block.right()
        };

        match status {
            Status::MOVE_OK => {
                if block.points.iter().any(|p| rocks.contains(p)) {
                    // undo the move and continue
                    if jet == '<' {
                        block.right();
                    } else {
                        block.left();
                    }
                }
            }
            Status::WALL => {}
            Status::FLOOR => panic!("Moving down should not hit a wall!"),
        }

        // fall
        match block.down() {
            Status::MOVE_OK => {
                if block.points.iter().any(|p| rocks.contains(p)) {
                    block.up();
                    insert_points(block.points, &mut rocks, &mut highest_rock);
                    rock_index += 1;
                    block = Block::bottom_at_height_for_index(highest_rock + 4, rock_index);
                    continue;
                }
            }
            Status::FLOOR => {
                insert_points(block.points, &mut rocks, &mut highest_rock);
                rock_index += 1;
                block = Block::bottom_at_height_for_index(highest_rock + 4, rock_index);
                continue;
            }
            Status::WALL => panic!("Moving down should not hit a wall!"),
        }
    }
    highest_rock + 1
}

fn part_2(contents: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 3068);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 1514285714288);
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
