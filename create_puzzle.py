import argparse

from pathlib import Path


MAIN_TEMPLATE = """use std::time::Instant;

fn part_1(contents: &str) -> u64 {
    0
}

fn part_2(contents: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("<input>", 1)]
    fn test_(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(part_1(input), expected);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 1);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 1);
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
"""

CARGO_TEMPLATE = """
[[example]]
name = "puzzle_{puzzle_number}"
"""

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--year", type=int, default=None)
    parser.add_argument("--puzzle_number", type=int, default=None)
    args = parser.parse_args()

    if args.year is None:
        args.year = max(
            int(str(p)) for p in Path(".").iterdir() if str(p).startswith("20")
        )
        print(f"No year supplied, using {args.year} based on " "existing folders")
    if args.puzzle_number is None:
        try:
            examples_folder = Path(str(args.year)) / "examples"
            max_puzzle_number = max(
                int(str(p).split("_")[-1]) for p in examples_folder.iterdir()
            )
        except ValueError:
            max_puzzle_number = 0
        args.puzzle_number = max_puzzle_number + 1
        print(
            f"No puzzle number supplied, using {args.puzzle_number} based "
            "on existing folders"
        )

    puzzle_folder = Path(str(args.year)) / "examples" / f"puzzle_{args.puzzle_number}"

    if puzzle_folder.exists():
        raise ValueError(f"Path {puzzle_folder} already exists!")

    puzzle_folder.mkdir(parents=True, exist_ok=True)

    files = {"input.txt": "", "example.txt": "", "main.rs": MAIN_TEMPLATE}
    for file_name, content in files.items():
        file_path = puzzle_folder / file_name
        with file_path.open("w", encoding="utf-8") as f:
            f.write(content)

    with open(Path(str(args.year)) / "Cargo.toml", "a") as f:
        f.write(CARGO_TEMPLATE.format(puzzle_number=args.puzzle_number))
