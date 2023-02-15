use std::time::Instant;

#[derive(Debug)]
struct VerticalLine {
    x: i64,
    y1: i64,
    y2: i64,
    steps: u64,
}

#[derive(Debug)]
struct HorizontalLine {
    y: i64,
    x1: i64,
    x2: i64,
    steps: u64,
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    steps: u64,
}

impl Point {
    fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    fn new(x: i64, y: i64, steps: u64) -> Self {
        Point { x, y, steps }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point::new(0, 0, 0)
    }
}

impl VerticalLine {
    fn intersection(&self, hline: &HorizontalLine) -> Option<Point> {
        let vertical_hit = (self.y1 <= hline.y && self.y2 >= hline.y)
            || (self.y1 >= hline.y && self.y2 <= hline.y);
        let horizontal_hit = (hline.x1 <= self.x && hline.x2 >= self.x)
            || (hline.x1 >= self.x && hline.x2 <= self.x);
        if vertical_hit && horizontal_hit {
            // lines counts steps till their end, so we need to subtract the
            // portion of them that is not traversed, i.e. after the intersection
            Some(Point {
                x: self.x,
                y: hline.y,
                steps: self.steps - self.y2.abs_diff(hline.y) + hline.steps
                    - hline.x2.abs_diff(self.x),
            })
        } else {
            None
        }
    }
}

fn parse_lines(instructions: &str) -> (Vec<VerticalLine>, Vec<HorizontalLine>) {
    let mut vertical_lines: Vec<VerticalLine> = vec![];
    let mut horizontal_lines: Vec<HorizontalLine> = vec![];

    let mut current_point = Point::default();
    for command in instructions.split(",") {
        let steps = command[1..].parse::<i64>().unwrap();
        match command.chars().nth(0) {
            Some('U') => {
                let vline = VerticalLine {
                    x: current_point.x,
                    y1: current_point.y,
                    y2: current_point.y + steps,
                    steps: current_point.steps + steps as u64,
                };
                vertical_lines.push(vline);
                current_point = Point {
                    x: current_point.x,
                    y: current_point.y + steps,
                    steps: current_point.steps + steps as u64,
                };
            }
            Some('D') => {
                let vline = VerticalLine {
                    x: current_point.x,
                    y1: current_point.y,
                    y2: current_point.y - steps,
                    steps: current_point.steps + steps as u64,
                };
                vertical_lines.push(vline);
                current_point = Point {
                    x: current_point.x,
                    y: current_point.y - steps,
                    steps: current_point.steps + steps as u64,
                };
            }
            Some('L') => {
                let hline = HorizontalLine {
                    y: current_point.y,
                    x1: current_point.x,
                    x2: current_point.x - steps,
                    steps: current_point.steps + steps as u64,
                };
                horizontal_lines.push(hline);
                current_point = Point {
                    x: current_point.x - steps,
                    y: current_point.y,
                    steps: current_point.steps + steps as u64,
                };
            }
            Some('R') => {
                let hline = HorizontalLine {
                    y: current_point.y,
                    x1: current_point.x,
                    x2: current_point.x + steps,
                    steps: current_point.steps + steps as u64,
                };
                horizontal_lines.push(hline);
                current_point = Point {
                    x: current_point.x + steps,
                    y: current_point.y,
                    steps: current_point.steps + steps as u64,
                };
            }
            _ => {
                panic!("First character of command should be one of LRUD")
            }
        }
    }

    (vertical_lines, horizontal_lines)
}

fn part_1(contents: &str) -> i64 {
    let instructions: Vec<&str> = contents.split("\n").collect();
    let wire_1 = instructions[0];
    let wire_2 = instructions[1];
    let (wire_1_vertical, wire_1_horizontal) = parse_lines(wire_1);
    let (wire_2_vertical, wire_2_horizontal) = parse_lines(wire_2);

    let mut minimal_distance = i64::MAX;
    for vline in wire_1_vertical.iter() {
        for hline in wire_2_horizontal.iter() {
            match vline.intersection(hline) {
                Some(p) => {
                    let distance = p.manhattan_distance();
                    if distance > 0 && distance < minimal_distance {
                        minimal_distance = distance;
                    }
                }
                None => {}
            }
        }
    }
    for vline in wire_2_vertical.iter() {
        for hline in wire_1_horizontal.iter() {
            match vline.intersection(hline) {
                Some(p) => {
                    let distance = p.manhattan_distance();
                    if distance > 0 && distance < minimal_distance {
                        minimal_distance = distance;
                    }
                }
                None => {}
            }
        }
    }
    minimal_distance
}

fn part_2(contents: &str) -> u64 {
    let instructions: Vec<&str> = contents.split("\n").collect();
    let wire_1 = instructions[0];
    let wire_2 = instructions[1];
    let (wire_1_vertical, wire_1_horizontal) = parse_lines(wire_1);
    let (wire_2_vertical, wire_2_horizontal) = parse_lines(wire_2);

    let mut minimal_steps = u64::MAX;
    for vline in wire_1_vertical.iter() {
        for hline in wire_2_horizontal.iter() {
            match vline.intersection(hline) {
                Some(p) => {
                    if p.steps > 0 && p.steps < minimal_steps {
                        minimal_steps = p.steps;
                    }
                }
                None => {}
            }
        }
    }
    for vline in wire_2_vertical.iter() {
        for hline in wire_1_horizontal.iter() {
            match vline.intersection(hline) {
                Some(p) => {
                    if p.steps > 0 && p.steps < minimal_steps {
                        minimal_steps = p.steps;
                    }
                }
                None => {}
            }
        }
    }
    minimal_steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("R8,U5,L5,D3\nU7,R6,D4,L4", 6)]
    #[case(
        "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
        159
    )]
    #[case(
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        135
    )]
    fn test_part_1_examples(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(part_1(input), expected);
    }

    #[rstest]
    #[case("R8,U5,L5,D3\nU7,R6,D4,L4", 30)]
    #[case(
        "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
        610
    )]
    #[case(
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        410
    )]
    fn test_part_2_examples(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(part_2(input), expected);
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
