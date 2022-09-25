import sys

from pathlib import Path


MAIN_TEMPLATE = """use std::time::Instant;

fn part_1(contents: &str) {
    println!("Answer for part 1 is: {}", 0);
}

fn part_2(contents: &str) {
    println!("Answer for part 2 is: {}", 0);
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    part_1(contents);
    part_2(contents);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
"""

CARGO_TEMPLATE = """
[[example]]
name = "puzzle_{puzzle_number}"
"""

if __name__ == "__main__":
    try:
        puzzle_number = int(sys.argv[1])
    except IndexError:
        try:
            max_puzzle_number = max(
                int(str(p).split("_")[-1]) for p in Path("examples").iterdir()
            )
        except ValueError:
            max_puzzle_number = 0
        puzzle_number = max_puzzle_number + 1
        print(
            f"No puzzle number supplied, using {puzzle_number} based "
            "on existing folders"
        )

    puzzle_folder = Path(f"examples/puzzle_{puzzle_number}/")

    if puzzle_folder.exists():
        raise ValueError(f"Path {puzzle_folder} already exists!")

    puzzle_folder.mkdir(parents=True, exist_ok=True)

    files = {"input.txt": "", "example.txt": "", "main.rs": MAIN_TEMPLATE}
    for file_name, content in files.items():
        file_path = puzzle_folder / file_name
        with file_path.open("w", encoding="utf-8") as f:
            f.write(content)

    with open("Cargo.toml", "a") as f:
        f.write(CARGO_TEMPLATE.format(puzzle_number=puzzle_number))
