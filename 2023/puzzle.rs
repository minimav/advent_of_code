fn puzzle(input: &str) {
    println!("{}", input);
}

fn main() {
    let example = include_str!("example.txt");
    puzzle(example);
    let input = include_str!("input.txt");
    puzzle(input);
}
