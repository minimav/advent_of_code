use std::time::Instant;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// Disjoint set union with sizes
struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        let mut parent = vec![0usize; n];
        for i in 0..n { parent[i] = i; }
        DSU { parent, size: vec![1usize; n] }
    }
    fn find(&mut self, mut x: usize) -> usize {
        let mut root = x;
        while self.parent[root] != root { root = self.parent[root]; }
        // path compression
        while self.parent[x] != root {
            let next = self.parent[x];
            self.parent[x] = root;
            x = next;
        }
        root
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb { return false; }

        if self.size[ra] < self.size[rb] {
            self.parent[ra] = rb;
            self.size[rb] += self.size[ra];
        } else {
            self.parent[rb] = ra;
            self.size[ra] += self.size[rb];
        }
        return true
    }

    fn comp_size(&mut self, x: usize) -> usize {
        let r = self.find(x);
        return self.size[r]
    }
}


fn get_coords_and_distances(contents: &str) -> (Vec<[i64; 3]>, BinaryHeap<Reverse<(i64, (usize, usize))>>) {
    let lines: Vec<&str> = contents.lines().collect();
    let mut coords: Vec<[i64; 3]> = Vec::new();
    for line in lines {
        let coord_v = line.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
        let coord: [i64; 3] = [coord_v[0], coord_v[1], coord_v[2]];
        coords.push(coord);
    }

    let mut distances = BinaryHeap::new();
    for i in 0..coords.len() {
        let coord_1 = &coords[i];
        for j in i+1..coords.len() {
            let coord_2 = &coords[j];
            let dist = (coord_1[0] - coord_2[0]).pow(2) + (coord_1[1] - coord_2[1]).pow(2) + (coord_1[2] - coord_2[2]).pow(2);
            distances.push(Reverse((dist, (i, j))));
        }
    }
    return (coords, distances);
}

fn part_1(
    coords: Vec<[i64; 3]>,
    mut distances: BinaryHeap<Reverse<(i64, (usize, usize))>>,
    num_pairs: usize
) -> u64 {
    let num_coords = coords.len();
    let mut groups = DSU::new(num_coords);
    for _ in 0..num_pairs {
        let Reverse(item) = distances.pop().unwrap();
        let (i, j) = item.1;
        groups.union(i, j);
    }

    let mut group_sizes: HashMap<usize, usize> = HashMap::new();
    for g in (0..num_coords).map(|v| groups.find(v)) {
        if g == 0 {
            continue;
        }
        *group_sizes.entry(g).or_default() += 1;
    }
    let mut largest: Vec<_> = group_sizes.into_iter().map(|p| p.1).collect();
    largest.sort_by(|a, b| b.cmp(&a));

    return (largest[0] as u64) * (largest[1] as u64) * (largest[2] as u64);
}

fn part_2(
    coords: Vec<[i64; 3]>,
    mut distances: BinaryHeap<Reverse<(i64, (usize, usize))>>,
) -> u64 {
    let num_coords = coords.len();
    let mut groups = DSU::new(num_coords);
    while distances.peek().is_some() {
        let Reverse(item) = distances.pop().unwrap();
        let (i, j) = item.1;
        groups.union(i, j);
        if groups.comp_size(j) == num_coords {
            return (coords[i][0] * coords[j][0]) as u64;
        } 
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt"), 10), 40);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 25272);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let (coords, distances) = get_coords_and_distances(contents);
    let part_1_answer = part_1(coords.clone(), distances.clone(), 1000);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer = part_2(coords.clone(), distances.clone());
    println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve puzzle", duration);
}