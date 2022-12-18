use parse_display::{Display, FromStr};
use std::collections::HashSet;
use std::time::Instant;

#[derive(Clone, Copy, Debug, Display, FromStr, PartialEq, Eq, Hash)]
#[display(r"{x},{y},{z}")]
struct Cube {
    x: i8,
    y: i8,
    z: i8,
}

impl Cube {
    fn manhattan_distance(&self, other: &Cube) -> u8 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }

    fn get_adjacent_face(&self, other: &Cube) -> Face {
        // assume that manhattan distance is 1
        if self.x - other.x == 1 {
            Face {
                min_x: self.x,
                max_x: self.x,
                min_y: self.y,
                max_y: self.y + 1,
                min_z: self.z,
                max_z: self.z + 1,
            }
        } else if self.x - other.x == -1 {
            Face {
                min_x: self.x + 1,
                max_x: self.x + 1,
                min_y: self.y,
                max_y: self.y + 1,
                min_z: self.z,
                max_z: self.z + 1,
            }
        } else if self.y - other.y == 1 {
            Face {
                min_x: self.x,
                max_x: self.x + 1,
                min_y: self.y,
                max_y: self.y,
                min_z: self.z,
                max_z: self.z + 1,
            }
        } else if self.y - other.y == -1 {
            Face {
                min_x: self.x,
                max_x: self.x + 1,
                min_y: self.y + 1,
                max_y: self.y + 1,
                min_z: self.z,
                max_z: self.z + 1,
            }
        } else if self.z - other.z == 1 {
            Face {
                min_x: self.x,
                max_x: self.x + 1,
                min_y: self.y,
                max_y: self.y + 1,
                min_z: self.z,
                max_z: self.z,
            }
        } else if self.z - other.z == -1 {
            Face {
                min_x: self.x,
                max_x: self.x + 1,
                min_y: self.y,
                max_y: self.y + 1,
                min_z: self.z + 1,
                max_z: self.z + 1,
            }
        } else {
            panic!("Manhattan distance was not 1!")
        }
    }
}

fn parse_cubes(contents: &str) -> Vec<Cube> {
    let mut cubes: Vec<Cube> = Vec::new();
    for line in contents.lines() {
        let cube = line.parse().unwrap();
        cubes.push(cube);
    }
    cubes
}

fn part_1(contents: &str) -> u64 {
    let cubes = parse_cubes(contents);
    let mut answer = 0;
    for cube in cubes.iter() {
        let mut num_covered = 0;
        for other_cube in cubes.iter() {
            if cube.manhattan_distance(other_cube) == 1 {
                num_covered += 1;
            }
        }
        answer += 6 - num_covered;
    }
    answer
}

#[derive(Clone, Copy, Debug, Display, FromStr, PartialEq, Eq, Hash)]
#[display(r"{min_x},{max_x},{min_y},{max_y},{min_z},{max_z}")]
struct Face {
    min_x: i8,
    max_x: i8,
    min_y: i8,
    max_y: i8,
    min_z: i8,
    max_z: i8,
}

impl Face {
    fn bounding_lines(&self) -> HashSet<((i8, i8, i8), (i8, i8, i8))> {
        let mut lines: HashSet<((i8, i8, i8), (i8, i8, i8))> = HashSet::new();
        if self.min_x == self.max_x {
            lines.insert((
                (self.min_x, self.min_y, self.min_z),
                (self.min_x, self.max_y, self.min_z),
            ));
            lines.insert((
                (self.min_x, self.min_y, self.min_z),
                (self.min_x, self.min_y, self.max_z),
            ));
            lines.insert((
                (self.min_x, self.max_y, self.min_z),
                (self.min_x, self.max_y, self.max_z),
            ));
            lines.insert((
                (self.min_x, self.min_y, self.max_z),
                (self.min_x, self.max_y, self.max_z),
            ));
        } else if self.min_y == self.max_y {
            lines.insert((
                (self.min_x, self.min_y, self.min_z),
                (self.max_x, self.min_y, self.min_z),
            ));
            lines.insert((
                (self.min_x, self.min_y, self.min_z),
                (self.min_x, self.min_y, self.max_z),
            ));
            lines.insert((
                (self.max_x, self.min_y, self.min_z),
                (self.max_x, self.min_y, self.max_z),
            ));
            lines.insert((
                (self.min_x, self.min_y, self.max_z),
                (self.max_x, self.min_y, self.max_z),
            ));
        } else if self.min_z == self.max_z {
            lines.insert((
                (self.min_x, self.min_y, self.min_z),
                (self.min_x, self.max_y, self.min_z),
            ));
            lines.insert((
                (self.min_x, self.min_y, self.min_z),
                (self.max_x, self.min_y, self.min_z),
            ));
            lines.insert((
                (self.min_x, self.max_y, self.min_z),
                (self.max_x, self.max_y, self.min_z),
            ));
            lines.insert((
                (self.max_x, self.min_y, self.min_z),
                (self.max_x, self.max_y, self.min_z),
            ));
        }
        lines
    }

    fn adjacent(&self, other: &Face) -> bool {
        let own_bounds = self.bounding_lines();
        let other_bounds = other.bounding_lines();
        let common_lines = own_bounds.intersection(&other_bounds);
        common_lines.count() > 0
    }

    fn faces_from_cube(cube: &Cube) -> HashSet<Self> {
        HashSet::from_iter(vec![
            Face {
                min_x: cube.x,
                max_x: cube.x + 1,
                min_y: cube.y,
                max_y: cube.y + 1,
                min_z: cube.z,
                max_z: cube.z,
            },
            Face {
                min_x: cube.x,
                max_x: cube.x + 1,
                min_y: cube.y,
                max_y: cube.y + 1,
                min_z: cube.z + 1,
                max_z: cube.z + 1,
            },
            Face {
                min_x: cube.x,
                max_x: cube.x,
                min_y: cube.y,
                max_y: cube.y + 1,
                min_z: cube.z,
                max_z: cube.z + 1,
            },
            Face {
                min_x: cube.x + 1,
                max_x: cube.x + 1,
                min_y: cube.y,
                max_y: cube.y + 1,
                min_z: cube.z,
                max_z: cube.z + 1,
            },
            Face {
                min_x: cube.x,
                max_x: cube.x + 1,
                min_y: cube.y,
                max_y: cube.y,
                min_z: cube.z,
                max_z: cube.z + 1,
            },
            Face {
                min_x: cube.x,
                max_x: cube.x + 1,
                min_y: cube.y + 1,
                max_y: cube.y + 1,
                min_z: cube.z,
                max_z: cube.z + 1,
            },
        ])
    }
}

fn part_2_no_diagonals(contents: &str) -> usize {
    /* Interior cubes do not have to be completely surrounded, they can
       have 'diagonal' adjacent cubes which this algorithm fails to deal
       with.
    */
    let cubes = parse_cubes(contents);
    let mut faces: HashSet<Face> = HashSet::new();
    for cube in cubes.iter() {
        let mut uncovered_cube_faces: HashSet<Face> = Face::faces_from_cube(cube);
        for other_cube in cubes.iter() {
            if cube.manhattan_distance(other_cube) == 1 {
                let covered_face = cube.get_adjacent_face(other_cube);
                uncovered_cube_faces.remove(&covered_face);
            }
        }

        for face in uncovered_cube_faces.into_iter() {
            faces.insert(face);
        }
    }

    let mut connected_components: Vec<Vec<&Face>> = Vec::new();
    for face in faces.iter() {
        let mut components_to_merge: HashSet<usize> = HashSet::new();
        for (component_index, component) in connected_components.iter().enumerate() {
            for other_face in component.iter() {
                if face.adjacent(other_face) {
                    components_to_merge.insert(component_index);
                    break;
                }
            }
        }

        // create new component, or merge existing ones
        if components_to_merge.len() == 0 {
            connected_components.push(vec![face]);
        } else {
            let mut new_connected_components: Vec<Vec<&Face>> = Vec::new();
            let mut merged_component: Vec<&Face> = Vec::new();
            for (component_index, component) in connected_components.into_iter().enumerate() {
                if components_to_merge.contains(&component_index) {
                    for component_face in component.iter() {
                        merged_component.push(component_face);
                    }
                } else {
                    new_connected_components.push(component);
                }
            }
            merged_component.push(face);
            new_connected_components.push(merged_component);
            connected_components = new_connected_components
        }
    }
    connected_components.iter().map(|v| v.len()).max().unwrap()
}

fn part_2(contents: &str) -> u64 {
    let cubes: HashSet<Cube> = HashSet::from_iter(parse_cubes(contents));
    let mut faces: HashSet<Face> = HashSet::new();
    for cube in cubes.iter() {
        let mut uncovered_cube_faces: HashSet<Face> = Face::faces_from_cube(cube);
        for other_cube in cubes.iter() {
            if cube.manhattan_distance(other_cube) == 1 {
                let covered_face = cube.get_adjacent_face(other_cube);
                uncovered_cube_faces.remove(&covered_face);
            }
        }

        for face in uncovered_cube_faces.into_iter() {
            faces.insert(face);
        }
    }

    /* Idea here is to iterate over cubes in a confined search space. If a cube is
       occupied via puzzle input or we've visited before we skip it. Otherwise we
       check if any of the cubes faces are exposed faces of the input. Since we're
       walking around the 'outside' of the input cubes, we'll only count the
       exposed ones. We then walk only to the 6 face-adjacent cubes to avoid the
       diagonally-adjacent cube case.
    */

    let mut answer = 0;
    // dumb search space that only works for small cube-like areas
    let min_bound = cubes.iter().map(|c| c.x.min(c.y.min(c.z))).min().unwrap() - 2;
    let max_bound = cubes.iter().map(|c| c.x.max(c.y.max(c.z))).max().unwrap() + 2;

    let mut walking_cubes = vec![Cube {
        x: min_bound,
        y: min_bound,
        z: min_bound,
    }];
    let mut visited_cubes: HashSet<Cube> = HashSet::new();

    while walking_cubes.len() > 0 {
        let next_cube = walking_cubes.pop().unwrap();

        if cubes.contains(&next_cube) || visited_cubes.contains(&next_cube) {
            continue;
        } else if next_cube.x < min_bound
            || next_cube.x > max_bound
            || next_cube.y < min_bound
            || next_cube.y > max_bound
            || next_cube.z < min_bound
            || next_cube.z > max_bound
        {
            // out of bounds
            continue;
        }
        for face in Face::faces_from_cube(&next_cube).iter() {
            if faces.contains(&face) {
                answer += 1;
            }
        }
        visited_cubes.insert(next_cube);

        // nearby cubes
        walking_cubes.push(Cube {
            x: next_cube.x + 1,
            y: next_cube.y,
            z: next_cube.z,
        });
        walking_cubes.push(Cube {
            x: next_cube.x - 1,
            y: next_cube.y,
            z: next_cube.z,
        });
        walking_cubes.push(Cube {
            x: next_cube.x,
            y: next_cube.y + 1,
            z: next_cube.z,
        });
        walking_cubes.push(Cube {
            x: next_cube.x,
            y: next_cube.y - 1,
            z: next_cube.z,
        });
        walking_cubes.push(Cube {
            x: next_cube.x,
            y: next_cube.y,
            z: next_cube.z + 1,
        });
        walking_cubes.push(Cube {
            x: next_cube.x,
            y: next_cube.y,
            z: next_cube.z - 1,
        });
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("0,1,0,1,0,0", "0,1,0,1,1,1", false)]
    #[case("0,1,0,1,0,0", "0,0,0,1,0,1", true)]
    #[case("0,1,0,1,0,0", "1,1,0,1,0,1", true)]
    #[case("0,1,0,1,0,0", "0,1,0,0,0,1", true)]
    #[case("0,1,0,1,0,0", "0,1,1,1,0,1", true)]
    #[case("0,1,0,1,0,0", "0,1,-1,0,0,0", true)]
    #[case("0,1,0,1,0,0", "0,1,1,2,0,0", true)]
    #[case("0,1,0,1,0,0", "0,1,3,4,0,0", false)]
    fn test_adjacency(#[case] face_1: &str, #[case] face_2: &str, #[case] expected: bool) {
        let f_1: Face = face_1.parse().unwrap();
        let f_2: Face = face_2.parse().unwrap();
        assert_eq!(f_1.adjacent(&f_2), expected);
    }

    #[rstest]
    #[case("0,0,0", "1,0,0", "1,1,0,1,0,1")]
    #[case("0,0,0", "0,1,0", "0,1,1,1,0,1")]
    #[case("0,0,0", "0,0,1", "0,1,0,1,1,1")]
    #[case("0,0,0", "0,0,-1", "0,1,0,1,0,0")]
    #[case("0,0,0", "0,-1,0", "0,1,0,0,0,1")]
    #[case("0,0,0", "-1,0,0", "0,0,0,1,0,1")]
    fn test_get_adjacent_face(#[case] cube_1: &str, #[case] cube_2: &str, #[case] face: &str) {
        let c_1: Cube = cube_1.parse().unwrap();
        let c_2: Cube = cube_2.parse().unwrap();
        let f: Face = face.parse().unwrap();
        assert_eq!(c_1.get_adjacent_face(&c_2), f);
    }

    #[test]
    fn test_part_1_small_example() {
        assert_eq!(part_1("1,1,1\n2,1,1"), 10);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 64);
    }

    #[test]
    fn test_part_2_non_diagonal_example() {
        // single cube completely enclosed, no diagonal case (easy!)
        // connected components approach works here
        assert_eq!(
            part_2_no_diagonals(include_str!("./non_diagonal_example.txt")),
            6 * 9
        );
    }

    #[test]
    fn test_part_2_minimal_example() {
        // 1 interior square, 6 exterior cubes attached to each face
        // 6 faces should be interior, 6 * 5 exposed
        assert_eq!(part_2(include_str!("./minimal_example.txt")), 30);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 58);
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
