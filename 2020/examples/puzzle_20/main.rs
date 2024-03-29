use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::time::Instant;

const TILE_SIZE: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Tile([[char; TILE_SIZE]; TILE_SIZE]);

impl Default for Tile {
    fn default() -> Self {
        Tile([['.'; TILE_SIZE]; TILE_SIZE])
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.0 {
            write!(f, "{}\n", row.iter().cloned().collect::<String>());
        }
        write!(f, "")
    }
}

impl Tile {
    fn flip_vertical(&self) -> Tile {
        let mut tile = [['.'; TILE_SIZE]; TILE_SIZE];
        for (row_index, row) in self.0.iter().enumerate() {
            for (column_index, char) in row.iter().enumerate() {
                tile[row_index][TILE_SIZE - 1 - column_index] = *char
            }
        }
        Tile(tile)
    }

    fn flip_horizontal(&self) -> Tile {
        let mut tile = [['.'; TILE_SIZE]; TILE_SIZE];
        for (row_index, row) in self.0.iter().enumerate() {
            for (column_index, char) in row.iter().enumerate() {
                tile[TILE_SIZE - 1 - row_index][column_index] = *char
            }
        }
        Tile(tile)
    }

    fn flip_diagonal(&self) -> Tile {
        let mut tile = [['.'; TILE_SIZE]; TILE_SIZE];
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

    /// 8 variants through transformations
    fn variants(&self) -> Vec<Tile> {
        let mut tiles: Vec<Tile> = Vec::new();
        tiles.push(*self);
        tiles.push(self.rotate());
        tiles.push(self.rotate().rotate());
        tiles.push(self.rotate().rotate().rotate());
        tiles.push(self.flip_horizontal());
        tiles.push(self.flip_horizontal().rotate());
        tiles.push(self.flip_horizontal().rotate().rotate());
        tiles.push(self.flip_horizontal().rotate().rotate().rotate());
        tiles
    }

    fn top(&self) -> [char; TILE_SIZE] {
        self.0[0]
    }

    fn bottom(&self) -> [char; TILE_SIZE] {
        self.0[9]
    }

    fn left(&self) -> [char; TILE_SIZE] {
        let mut arr = ['.'; TILE_SIZE];
        for (row_index, row) in self.0.iter().enumerate() {
            arr[row_index] = row[0];
        }
        arr
    }

    fn right(&self) -> [char; TILE_SIZE] {
        let mut arr = ['.'; TILE_SIZE];
        for (row_index, row) in self.0.iter().enumerate() {
            arr[row_index] = row[TILE_SIZE - 1];
        }
        arr
    }

    fn matches(arr_1: &[char; TILE_SIZE], arr_2: &[char; TILE_SIZE]) -> bool {
        arr_1.iter().zip(arr_2.iter()).all(|(a, b)| a == b)
    }

    fn match_top(&self, other: &Tile) -> bool {
        Tile::matches(&self.top(), &other.bottom())
    }
    fn match_bottom(&self, other: &Tile) -> bool {
        Tile::matches(&self.bottom(), &other.top())
    }
    fn match_left(&self, other: &Tile) -> bool {
        Tile::matches(&self.left(), &other.right())
    }
    fn match_right(&self, other: &Tile) -> bool {
        Tile::matches(&self.right(), &other.left())
    }
}

fn parse_tiles(contents: &str) -> BTreeMap<u64, Tile> {
    let mut tiles: BTreeMap<u64, Tile> = BTreeMap::new();
    let mut current_tile_number: u64 = 0;
    let mut current_tile = Tile::default();
    let mut current_row: usize = 0;
    for line in contents.lines() {
        if line.contains("Tile") {
            let raw_tile_number = line
                .split_at(line.len() - 1)
                .0
                .split_whitespace()
                .collect::<Vec<&str>>()[1];

            current_tile_number = raw_tile_number.parse::<u64>().unwrap();
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct TileWithId {
    id: u64,
    tile: Tile,
}

impl TileWithId {
    fn new(id: u64, tile: Tile) -> Self {
        TileWithId { id, tile }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Arrangement {
    tiles: BTreeMap<(i8, i8), TileWithId>,
    outside: BTreeSet<(i8, i8)>,
    ids: BTreeSet<u64>,
}

impl fmt::Display for Arrangement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (coords, tile) in self.tiles.iter() {
            write!(f, "Tile {} @ {coords:?}\n", tile.id);
            write!(f, "{}\n", tile.tile);
        }
        write!(f, "")
    }
}

impl Arrangement {
    fn get_num_neighbours(&self, coords: &(i8, i8)) -> i8 {
        let mut num_neighbours = 0;
        if self.tiles.contains_key(&(coords.0 + 1, coords.1)) {
            num_neighbours += 1;
        }
        if self.tiles.contains_key(&(coords.0, coords.1 + 1)) {
            num_neighbours += 1;
        }
        if self.tiles.contains_key(&(coords.0 - 1, coords.1)) {
            num_neighbours += 1;
        }
        if self.tiles.contains_key(&(coords.0, coords.1 - 1)) {
            num_neighbours += 1;
        }
        num_neighbours
    }

    fn is_valid_square(&self) -> bool {
        let mut neighbour_counts: BTreeMap<i8, i8> = BTreeMap::new();
        for (coords, tile) in self.tiles.iter() {
            let num_neighbours = self.get_num_neighbours(coords);
            neighbour_counts
                .entry(num_neighbours)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        let num_tiles = self.tiles.len();
        let side_length = (num_tiles as f64).sqrt() as i8;

        if neighbour_counts.len() != 3 {
            // tiles should only have 2, 3 or 4 neighbours
            return false;
        } else if neighbour_counts.get(&2) != Some(&4) {
            // only 4 corners should have 2 neighbours
            return false;
        } else if neighbour_counts.get(&3) != Some(&(4 * side_length - 8)) {
            // edge but not corners should have 3 neighbours
            return false;
        } else if neighbour_counts.get(&4) != Some(&(side_length - 2).pow(2)) {
            // inner grid should all have 4 neighbours
            return false;
        }
        true
    }

    fn corner_product(&self) -> u64 {
        let mut product = 1;
        for (coords, tile) in self.tiles.iter() {
            if self.get_num_neighbours(coords) == 2 {
                product *= tile.id;
            }
        }
        product
    }
}

fn create_new_arrangements(
    arrangements: BTreeSet<Arrangement>,
    tile: TileWithId,
    grid_size: i8,
) -> BTreeSet<Arrangement> {
    let mut new_arrangements: BTreeSet<Arrangement> = BTreeSet::new();

    for arrangement in arrangements.iter() {
        if arrangement.ids.contains(&tile.id) {
            continue;
        }

        for variant in tile.tile.variants().iter() {
            for coords in arrangement.outside.iter() {
                match arrangement.tiles.get(&(coords.0 + 1, coords.1)) {
                    Some(other_tile) => {
                        if !variant.match_right(&other_tile.tile) {
                            continue;
                        }
                    }
                    _ => (),
                }
                match arrangement.tiles.get(&(coords.0 - 1, coords.1)) {
                    Some(other_tile) => {
                        if !variant.match_left(&other_tile.tile) {
                            continue;
                        }
                    }
                    _ => (),
                }
                match arrangement.tiles.get(&(coords.0, coords.1 + 1)) {
                    Some(other_tile) => {
                        if !variant.match_top(&other_tile.tile) {
                            continue;
                        }
                    }
                    _ => (),
                }
                match arrangement.tiles.get(&(coords.0, coords.1 - 1)) {
                    Some(other_tile) => {
                        if !variant.match_bottom(&other_tile.tile) {
                            continue;
                        }
                    }
                    _ => (),
                }

                // passed all checks, so this is a valid arrangement
                let mut new_arrangement = arrangement.clone();
                new_arrangement.tiles.insert(
                    *coords,
                    TileWithId {
                        id: tile.id,
                        tile: *variant,
                    },
                );
                new_arrangement.ids.insert(tile.id.clone());
                new_arrangement.outside.remove(coords);
                let above = (coords.0, coords.1 + 1);
                if !new_arrangement.tiles.contains_key(&above) {
                    new_arrangement.outside.insert(above);
                }
                let below = (coords.0, coords.1 - 1);
                if !new_arrangement.tiles.contains_key(&below) {
                    new_arrangement.outside.insert(below);
                }
                let left = (coords.0 - 1, coords.1);
                if !new_arrangement.tiles.contains_key(&left) {
                    new_arrangement.outside.insert(left);
                }
                let right = (coords.0, coords.1 + 1);
                if !new_arrangement.tiles.contains_key(&right) {
                    new_arrangement.outside.insert(right);
                }
                new_arrangements.insert(new_arrangement);
            }
        }
    }
    new_arrangements
}

fn find_corners(tiles: Vec<TileWithId>) -> Vec<u64> {
    let mut edge_patterns: BTreeMap<[char; TILE_SIZE], u8> = BTreeMap::new();
    for tile in tiles.iter() {
        edge_patterns
            .entry(tile.tile.top())
            .and_modify(|x| *x += 1)
            .or_insert(1);
        edge_patterns
            .entry(tile.tile.bottom())
            .and_modify(|x| *x += 1)
            .or_insert(1);
        edge_patterns
            .entry(tile.tile.left())
            .and_modify(|x| *x += 1)
            .or_insert(1);
        edge_patterns
            .entry(tile.tile.right())
            .and_modify(|x| *x += 1)
            .or_insert(1);

        let tv = tile.tile.flip_vertical();
        edge_patterns
            .entry(tv.top())
            .and_modify(|x| *x += 1)
            .or_insert(1);
        edge_patterns
            .entry(tv.bottom())
            .and_modify(|x| *x += 1)
            .or_insert(1);

        let th = tile.tile.flip_horizontal();
        edge_patterns
            .entry(th.left())
            .and_modify(|x| *x += 1)
            .or_insert(1);
        edge_patterns
            .entry(th.right())
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    let mut corners: Vec<u64> = Vec::new();
    for tile in tiles.iter() {
        let mut num_with_one_match = 0;
        if edge_patterns.get(&tile.tile.top()).unwrap() == &1 {
            num_with_one_match += 1;
        }
        if edge_patterns.get(&tile.tile.bottom()).unwrap() == &1 {
            num_with_one_match += 1;
        }
        if edge_patterns.get(&tile.tile.left()).unwrap() == &1 {
            num_with_one_match += 1;
        }
        if edge_patterns.get(&tile.tile.right()).unwrap() == &1 {
            num_with_one_match += 1;
        }

        if num_with_one_match == 2 {
            corners.push(tile.id);
        }
    }
    println!("{corners:?}");
    corners
}

fn part_1(contents: &str) -> u64 {
    let mut tiles: Vec<TileWithId> = parse_tiles(contents)
        .into_iter()
        .map(|(i, t)| TileWithId::new(i, t))
        .collect::<Vec<_>>();

    let corners = find_corners(tiles);
    corners.iter().product()
}

fn create_mega_grid(arrangement: Arrangement, grid_size: i8) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mega_grid_size = grid_size as usize * (TILE_SIZE - 2);
    for _ in 0..mega_grid_size {
        grid.push(vec!['.'; mega_grid_size]);
    }
    for (coords, tile) in arrangement.tiles.into_iter() {
        let tile_arr = tile.tile.0;
        for (row_index, row) in tile_arr[1..TILE_SIZE - 1].iter().enumerate() {
            for (column_index, char) in row[1..TILE_SIZE - 1].iter().enumerate() {
                let mega_row_index = coords.1.abs() as usize * (TILE_SIZE - 2) + row_index;
                let mega_column_index =
                    (grid_size - coords.0.abs() - 1) as usize * (TILE_SIZE - 2) + column_index;
                grid[mega_row_index][mega_column_index] = *char;
            }
        }
    }
    for row in grid.iter() {
        println!("{}", row.iter().cloned().collect::<String>());
    }
    grid
}

fn part_2(contents: &str) -> u64 {
    let mut tiles: BTreeMap<u64, TileWithId> = BTreeMap::from_iter(
        parse_tiles(contents)
            .into_iter()
            .map(|(i, t)| (i, TileWithId::new(i, t)))
            .collect::<Vec<_>>(),
    );

    let num_iterations = tiles.len() - 1;
    let grid_size = (tiles.len() as f64).sqrt() as i8;
    println!("Grid size is {grid_size} ({} tiles)", tiles.len());
    let mut arrangements: BTreeSet<Arrangement> = BTreeSet::new();

    // corners are 1009, 1087, 1327, 2753
    let id = 1009;
    let corner_tile = tiles.remove(&id).unwrap();
    // only should build in negative directions
    arrangements.insert(Arrangement {
        tiles: BTreeMap::from([((0, 0), corner_tile)]),
        ids: BTreeSet::from([id]),
        outside: BTreeSet::from([(-1, 0), (0, -1)]),
    });

    for iteration in 0..num_iterations {
        let mut new_arrangements: BTreeSet<Arrangement> = BTreeSet::new();

        for (id, tile) in tiles.iter() {
            let new_arrangements_with_tile =
                create_new_arrangements(arrangements.clone(), tile.clone(), grid_size);
            if new_arrangements_with_tile.len() > 0 {
                for arrangement in new_arrangements_with_tile.into_iter() {
                    new_arrangements.insert(arrangement);
                }
                break;
            }
        }
        if arrangements.len() == 0 {
            panic!("Should never have no arrangements!");
        }
        arrangements = new_arrangements;
    }

    for arrangement in arrangements.into_iter() {
        if arrangement.is_valid_square() {
            assert_eq!(arrangement.corner_product(), 4006801655873);
            create_mega_grid(arrangement, grid_size);
        }
    }
    panic!("At least one arrangement should be a valid square!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn tiles() -> BTreeMap<u64, Tile> {
        parse_tiles(include_str!("./example.txt"))
    }

    #[rstest]
    #[case(1951)]
    #[case(2311)]
    #[case(3079)]
    #[case(2729)]
    #[case(1427)]
    #[case(2473)]
    #[case(2971)]
    #[case(1489)]
    #[case(1171)]
    fn test_transformations_are_invariant(tiles: BTreeMap<u64, Tile>, #[case] id: u64) {
        let tile = tiles.get(&id).unwrap().clone();

        assert_eq!(tile, tile.flip_diagonal().flip_diagonal());
        assert_eq!(tile, tile.flip_horizontal().flip_horizontal());
        assert_eq!(tile, tile.flip_vertical().flip_vertical());
        assert_eq!(tile, tile.rotate().rotate().rotate().rotate());

        assert_eq!(tile.match_bottom(&tile.flip_horizontal()), true);
        assert_eq!(tile.match_top(&tile.flip_horizontal()), true);
        assert_eq!(tile.match_left(&tile.flip_vertical()), true);
        assert_eq!(tile.match_right(&tile.flip_vertical()), true);
    }

    #[rstest]
    fn test_matches_in_example(tiles: BTreeMap<u64, Tile>) {
        let top_middle = tiles.get(&2311).unwrap().flip_horizontal();
        let top_left = tiles.get(&1951).unwrap().flip_horizontal();
        let middle = tiles.get(&1427).unwrap().flip_horizontal();
        assert_eq!(top_middle.match_left(&top_left), true);
        assert_eq!(top_middle.match_bottom(&middle), true);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 20899048083289);
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
