use std::collections::BTreeMap;
use std::fmt::Display;
use std::time::Instant;

#[derive(Debug)]
enum Command {
    NOOP,
    ADDX(i32),
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let components = s.split_whitespace().collect::<Vec<&str>>();
        match components.len() {
            1 => Command::NOOP,
            2 => Command::ADDX(components[1].parse::<i32>().unwrap()),
            _ => panic!("Cannot parse this line!"),
        }
    }
}

fn part_1(contents: &str, cycles_to_sum: Vec<i32>) -> i32 {
    let mut register: BTreeMap<i32, i32> = BTreeMap::from([(0, 1)]);
    let mut current_value: i32 = 1;
    let mut cycle: i32 = 1;
    for line in contents.lines() {
        match Command::from(line) {
            Command::NOOP => {
                register.entry(cycle).or_insert(current_value);
                cycle += 1;
            }
            Command::ADDX(value) => {
                register.entry(cycle).or_insert(current_value);
                cycle += 1;
                register.entry(cycle).or_insert(current_value);
                current_value += value;
                cycle += 1;
            }
        };
    }

    cycles_to_sum.iter().map(|x| register[x] * x).sum()
}

struct CathodeRayTube([[char; 40]; 6]);

impl Default for CathodeRayTube {
    fn default() -> Self {
        CathodeRayTube([['.'; 40]; 6])
    }
}

impl Display for CathodeRayTube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0 {
            write!(f, "{}\n", String::from_iter(row));
        }
        write!(f, "")
    }
}

impl CathodeRayTube {
    fn num_columns(&self) -> usize {
        self.0[0].len()
    }

    fn to_crt_index(&self, cycle: i32) -> (usize, usize) {
        let row_index = (cycle as usize) / self.num_columns();
        let column_index = (cycle as usize) % self.num_columns();
        (row_index as usize, column_index)
    }

    fn update(&mut self, cycle: i32, current_register_value: i32) {
        let (sprite_row_index, sprite_column_index) = self.to_crt_index(cycle - 1);
        if ((sprite_column_index as i32) - current_register_value).abs() <= 1 {
            self.0[sprite_row_index][sprite_column_index] = '#';
        }
    }
}

fn part_2(contents: &str) -> String {
    let mut crt = CathodeRayTube::default();

    let mut register: BTreeMap<i32, i32> = BTreeMap::from([(0, 1)]);
    let mut current_value: i32 = 1;
    let mut cycle: i32 = 1;
    for line in contents.lines() {
        match Command::from(line) {
            Command::NOOP => {
                register.entry(cycle).or_insert(current_value);
                crt.update(cycle, current_value);
                cycle += 1;
            }
            Command::ADDX(value) => {
                // first cycle of command
                register.entry(cycle).or_insert(current_value);
                crt.update(cycle, current_value);
                cycle += 1;

                // second cycle of command
                register.entry(cycle).or_insert(current_value);
                crt.update(cycle, current_value);
                current_value += value;
                cycle += 1;
            }
        };
    }
    format!("{}", crt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_minimal_example() {
        let example: &str = "noop
        addx 3
        addx -5";
        let cycles_to_sum: Vec<i32> = Vec::from([1, 3, 5]);
        assert_eq!(part_1(example, cycles_to_sum), 1 * 1 + 3 * 1 + 5 * 4);
    }

    #[test]
    fn test_part_1_example() {
        let cycles_to_sum: Vec<i32> = Vec::from([20, 60, 100, 140, 180, 220]);
        assert_eq!(part_1(include_str!("./example.txt"), cycles_to_sum), 13140);
    }

    #[rstest]
    #[case(40, (1, 0))]
    #[case(39, (0, 39))]
    #[case(60, (1, 20))]
    fn test_to_crt_index(#[case] cycle: i32, #[case] expected_index: (usize, usize)) {
        let mut crt = CathodeRayTube::default();
        assert_eq!(crt.to_crt_index(cycle), expected_index);
    }

    #[test]
    fn test_part_2_example() {
        let answer = String::from(
            "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n",
        );
        assert_eq!(part_2(include_str!("./example.txt")), answer);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let cycles_to_sum: Vec<i32> = Vec::from([20, 60, 100, 140, 180, 220]);
    let part_1_answer = part_1(contents, cycles_to_sum);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer = part_2(contents);
    println!("Answer for part 2 is:\n{}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
