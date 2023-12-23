use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point3D {
    x: u16,
    y: u16,
    z: u16,
}

impl Point3D {
    fn new(s: &str) -> Self {
        let coords: Vec<u16> = s.split(",").map(|s| s.parse::<u16>().unwrap()).collect();
        let x = coords[0];
        let y = coords[1];
        let z = coords[2];
        Self { x, y, z }
    }
    fn drop(&mut self, n: u16) {
        self.z -= n;
    }
    fn up(&mut self, n: u16) {
        self.z += n;
    }
}

#[derive(Hash, Copy, Clone, Eq, PartialEq)]
struct Block {
    start: Point3D,
    end: Point3D,
}

impl Block {
    fn new(line: &str) -> Self {
        let mut parts = line.split("~");
        let first = Point3D::new(parts.next().unwrap());
        let second = Point3D::new(parts.next().unwrap());
        // Always orient so start z is the smallest for vertical block case
        if first.z > second.z {
            return Self {
                start: second,
                end: first,
            };
        } else {
            return Self {
                start: first,
                end: second,
            };
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Self {
            start: Point3D { x: 0, y: 0, z: 0 },
            end: Point3D { x: 0, y: 0, z: 0 },
        }
    }
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({},{},{})->({},{},{})",
            self.start.x, self.start.y, self.start.z, self.end.x, self.end.y, self.end.z
        )
    }
}

impl Block {
    fn z(&self) -> u16 {
        self.start.z
    }
    fn flat_extent(&self) -> Vec<Point3D> {
        let mut points = Vec::new();
        for x in self.start.x..=self.end.x {
            for y in self.start.y..=self.end.y {
                points.push(Point3D { x, y, z: self.z() });
            }
        }
        points
    }
    fn top(&self) -> Vec<Point3D> {
        if self.start.z != self.end.z {
            // Vertical block case
            return vec![Point3D {
                x: self.end.x,
                y: self.end.y,
                z: self.end.z,
            }];
        } else {
            // Flat block case
            return self.flat_extent();
        }
    }
    fn bottom(&self) -> Vec<Point3D> {
        if self.start.z != self.end.z {
            // Vertical block case
            return vec![Point3D {
                x: self.start.x,
                y: self.start.y,
                z: self.start.z,
            }];
        } else {
            // Flat block case
            return self.flat_extent();
        }
    }
    fn drop(&mut self, n: u16) {
        self.start.drop(n);
        self.end.drop(n);
    }
    fn up(&mut self, n: u16) {
        self.start.up(n);
        self.end.up(n);
    }
}

struct DroppedBlocks {
    blocks: Vec<Block>,
    supported_by: HashMap<Block, HashSet<Block>>,
}

fn let_drop(blocks: Vec<Block>) -> DroppedBlocks {
    let mut dropped_blocks: Vec<Block> = Vec::new();
    let mut current_floor: HashMap<Point3D, Block> = HashMap::new();
    let mut supported_by: HashMap<Block, HashSet<Block>> = HashMap::new();

    for mut block in blocks {
        loop {
            if block.z() == 1 {
                for point in block.top() {
                    *current_floor.entry(point).or_insert(Block::default()) = block;
                }
                dropped_blocks.push(block);
                // Not supported by anything but the floor
                //supported_by.entry(block).or_insert(HashSet::new());
                break;
            }

            // Block might be able to drop, possibly to the floor
            block.drop(1);

            // Check if anything is in the current floor, using bottom of block
            let collisions: Vec<_> = block
                .bottom()
                .iter()
                .filter_map(|point| current_floor.get(&point))
                .collect();
            if collisions.is_empty() {
                // Still space to move down
                continue;
            }

            // Hit an obstruction, first shift back up to resting place
            block.up(1);

            // Update current floor and supported by
            collisions.iter().for_each(|other_block| {
                supported_by
                    .entry(block)
                    .or_insert(HashSet::new())
                    .insert(**other_block);
            });
            // Update floor with the top of a block
            block.top().iter().for_each(|point| {
                *current_floor.entry(*point).or_insert(Block::default()) = block;
            });
            dropped_blocks.push(block);
            break;
        }
    }

    DroppedBlocks {
        blocks: dropped_blocks,
        supported_by: supported_by,
    }
}

fn puzzle(input: &str) {
    let mut blocks = input
        .lines()
        .map(|line| Block::new(line))
        .collect::<Vec<Block>>();
    blocks.sort_by(|a, b| a.start.z.cmp(&b.start.z));

    // Part 1
    let dropped_blocks: DroppedBlocks = let_drop(blocks);

    // Can disintegrate if whenever it supports something, there is another
    // block supporting it
    let can_distintegrate = dropped_blocks
        .blocks
        .iter()
        .filter(|b| {
            dropped_blocks
                .supported_by
                .iter()
                .filter(|(_k, v)| v.contains(b) && v.len() == 1)
                .count()
                == 0
        })
        .collect::<Vec<&Block>>();
    println!("{}", can_distintegrate.len());

    // Part 2 - this repeats a lot of work
    // Better solution removes each block and counts number of falls and which
    // ones fall, then recursively walk through
    let mut total_drops = 0;
    for block in dropped_blocks.blocks.iter() {
        if can_distintegrate.contains(&block) {
            continue;
        }
        let mut supported_by = dropped_blocks.supported_by.clone();
        let mut to_remove: HashSet<Block> = HashSet::new();
        to_remove.insert(*block);
        loop {
            if to_remove.is_empty() {
                break;
            }
            let remove_block = to_remove.iter().next().cloned().unwrap();
            to_remove.remove(&remove_block);
            supported_by.remove(&remove_block);

            let mut to_remove_keys = Vec::new();
            for (k, v) in supported_by.iter_mut() {
                v.remove(&remove_block);
                if v.is_empty() {
                    to_remove_keys.push(*k);
                    to_remove.insert(*k);
                    total_drops += 1;
                }
            }
            for k in to_remove_keys {
                supported_by.remove(&k);
            }
        }
    }
    println!("{}", total_drops);
}

fn main() {
    let example = include_str!("example.txt");
    puzzle(example);
    let input = include_str!("input.txt");
    puzzle(input);
}
