use std::collections::HashMap;
use std::fmt;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct Tile([[char; 10]; 10]);

impl Default for Tile {
    fn default() -> Self {
        Tile([['.'; 10]; 10])
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.0 {
            write!(f, "{:?}\n", row.iter().cloned().collect::<String>());
        }
        write!(f, "")
    }
}

impl Tile {
    fn flip_vertical(&self) -> Tile {
        let mut tile = [['.'; 10]; 10];
        for (row_index, row) in self.0.iter().enumerate() {
            for (column_index, char) in row.iter().enumerate() {
                tile[row_index][9 - column_index] = *char
            }
        }
        Tile(tile)
    }

    fn flip_horizontal(&self) -> Tile {
        let mut tile = [['.'; 10]; 10];
        for (row_index, row) in self.0.iter().enumerate() {
            for (column_index, char) in row.iter().enumerate() {
                tile[9 - row_index][column_index] = *char
            }
        }
        Tile(tile)
    }

    fn flip_diagonal(&self) -> Tile {
        let mut tile = [['.'; 10]; 10];
        for (row_index, row) in self.0.iter().enumerate() {
            for (column_index, char) in row.iter().enumerate() {
                tile[column_index][row_index] = *char
            }
        }
        Tile(tile)
    }

    fn rotate(&self) -> Tile {
        self.flip_diagonal().flip_horizontal()
    }

    fn variants(&self) -> Vec<Tile> {
        let mut tiles: Vec<Tile> = Vec::new();
        tiles.push(*self);
        tiles.push(self.flip_vertical());
        tiles.push(self.flip_horizontal());
        tiles.push(self.rotate());
        tiles.push(self.rotate().rotate());
        tiles.push(self.rotate().rotate().rotate());
        tiles
    }

    fn top(&self) -> [char; 10] {
        self.0[0]
    }

    fn bottom(&self) -> [char; 10] {
        self.0[9]
    }

    fn left(&self) -> [char; 10] {
        let mut arr = ['.'; 10];
        for (row_index, row) in self.0.iter().enumerate() {
            arr[row_index] = row[0];
        }
        arr
    }

    fn right(&self) -> [char; 10] {
        let mut arr = ['.'; 10];
        for (row_index, row) in self.0.iter().enumerate() {
            arr[row_index] = row[9];
        }
        arr
    }

    fn matches(arr_1: [char; 10], arr_2: [char; 10]) -> bool {
        arr_1.iter().zip(arr_2.iter()).all(|(a, b)| a == b)
    }

    fn match_top(&self, other: Tile) -> bool {
        Tile::matches(self.top(), other.bottom())
    }
    fn match_bottom(&self, other: Tile) -> bool {
        Tile::matches(self.bottom(), other.top())
    }
    fn match_left(&self, other: Tile) -> bool {
        Tile::matches(self.left(), other.right())
    }
    fn match_right(&self, other: Tile) -> bool {
        Tile::matches(self.right(), other.left())
    }
}

fn parse_tiles(contents: &str) -> HashMap<u32, Tile> {
    let mut tiles: HashMap<u32, Tile> = HashMap::new();
    let mut current_tile_number: u32 = 0;
    let mut current_tile = Tile::default();
    let mut current_row: usize = 0;
    for line in contents.lines() {
        if line.contains("Tile") {
            let raw_tile_number = line
                .split_at(line.len() - 1)
                .0
                .split_whitespace()
                .collect::<Vec<&str>>()[1];

            current_tile_number = raw_tile_number.parse::<u32>().unwrap();
            current_tile = Tile::default();
            current_row = 0;
        } else if line.is_empty() {
            tiles.insert(current_tile_number, current_tile);
        } else {
            for (column, char) in line.chars().enumerate() {
                current_tile.0[current_row][column] = char;
            }
            current_row += 1;
        }
    }
    tiles
}

#[derive(Debug, Clone, Copy)]
struct TileWithNeighbours<'a> {
    tile: &'a Tile,
    left: Option<u32>,
    right: Option<u32>,
    top: Option<u32>,
    bottom: Option<u32>,
}

impl<'a> TileWithNeighbours<'a> {
    fn init(tile: &'a Tile) -> Self {
        TileWithNeighbours {
            tile,
            left: None,
            right: None,
            top: None,
            bottom: None,
        }
    }
}

#[derive(Debug, Clone)]
struct Arrangement<'a> {
    tiles: HashMap<&'a u32, TileWithNeighbours<'a>>,
}

fn create_new_arrangments<'a>(
    arrangements: Vec<Arrangement<'a>>,
    key: &'a u32,
    tile: &'a Tile,
) -> Vec<Arrangement<'a>> {
    let mut new_arrangements: Vec<Arrangement> = Vec::new();

    for variant in tile.variants().iter() {
        for arrangement in arrangements.iter() {
            for (other_key, other_tile) in arrangement.tiles.iter() {
                if variant.match_top(*other_tile.tile) {
                    let mut new_arrangement = arrangement.clone();
                    new_arrangement.tiles.insert(
                        key,
                        TileWithNeighbours {
                            tile: variant,
                            top: Some(**other_key),
                            left: None,
                            right: None,
                            bottom: None,
                        },
                    );
                    new_arrangement.tiles.entry(other_key).and_modify(|v| {
                        v.bottom = Some(*key);
                    });
                    new_arrangements.push(new_arrangement);
                }
                if variant.match_bottom(*other_tile.tile) {
                    let mut new_arrangement = arrangement.clone();

                    new_arrangements.push(new_arrangement);
                }
                if variant.match_left(*other_tile.tile) {
                    let mut new_arrangement = arrangement.clone();

                    new_arrangements.push(new_arrangement);
                }
                if variant.match_right(*other_tile.tile) {
                    let mut new_arrangement = arrangement.clone();

                    new_arrangements.push(new_arrangement);
                }
            }
        }
    }

    new_arrangements
}

fn part_1(contents: &str) -> u64 {
    let tiles: HashMap<u32, Tile> = parse_tiles(contents);

    let mut arrangements: Vec<Arrangement> = Vec::new();
    for (key, tile) in tiles.iter() {
        if arrangements.len() == 0 {
            let tile_with_neighbours = TileWithNeighbours::init(tile);
            arrangements.push(Arrangement {
                tiles: HashMap::from([(key, tile_with_neighbours)]),
            });
            continue;
        }
        arrangements = create_new_arrangments(arrangements, key, tile);
    }

    println!("{:?}", arrangements);
    0
}

fn part_2(contents: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn tiles() -> HashMap<u32, Tile> {
        parse_tiles(include_str!("./example.txt"))
    }

    #[rstest]
    #[case(1951, 2311, true)]
    fn test_match_right(
        tiles: HashMap<u32, Tile>,
        #[case] id_1: u32,
        #[case] id_2: u32,
        #[case] expected_output: bool,
    ) {
        let tile_1 = tiles.get(&id_1).unwrap();
        let tile_2 = tiles.get(&id_2).unwrap();
        assert_eq!(tile_1.match_right(*tile_2), expected_output)
    }

    #[rstest]
    #[case(2311, 1951, true)]
    fn test_match_left(
        tiles: HashMap<u32, Tile>,
        #[case] id_1: u32,
        #[case] id_2: u32,
        #[case] expected_output: bool,
    ) {
        let tile_1 = tiles.get(&id_1).unwrap();
        let tile_2 = tiles.get(&id_2).unwrap();
        assert_eq!(tile_1.match_left(*tile_2), expected_output)
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 20899048083289);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./example.txt");
    let part_1_answer = part_1(contents);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer = part_2(contents);
    println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
