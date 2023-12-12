from collections import Counter
from functools import lru_cache


def clean(puzzle):
    # Remove redundant dots at the beginning and end of the puzzle
    puzzle = puzzle.lstrip(".").rstrip(".")
    # Remove redundant dots between the springs
    no_repeated_dots = ""
    previous = ""
    for c in puzzle:
        if c != "." or previous != ".":
            no_repeated_dots += c
        previous = c
    return no_repeated_dots


def preprocess_part_1(line):
    puzzle, springs = line.split()
    springs = tuple([int(spring) for spring in springs.split(",")])
    return clean(puzzle), springs


def preprocess_part_2(line):
    puzzle, springs = line.split()
    springs = tuple([int(spring) for spring in springs.split(",")])
    puzzle = "?".join([puzzle] * 5)
    springs = springs * 5
    return clean(puzzle), springs


@lru_cache(maxsize=None)
def solve(puzzle, springs):
    if len(springs) == 0:
        if "#" in puzzle:
            return 0
        else:
            # All ? turn to .
            # Trimmed .s case with empty string is covered here
            return 1
    elif not puzzle:
        # e.g. "" [1, 2] case
        return 0

    first_spring, *remaining_springs = springs
    if len(puzzle) < first_spring:
        # Not long enough
        return 0

    num_springs = sum(springs)
    counts = Counter(puzzle)
    if counts.get("?", 0) + counts.get("#", 0) < num_springs:
        return 0

    if puzzle.startswith("#"):
        # Remove first group if possible
        if "." in puzzle[:first_spring]:
            # Not enough space for first spring, e.g. #.??? [2, ...]
            return 0
        elif len(puzzle) == first_spring:
            if not remaining_springs:
                # Case like ???#, [4]
                return 1
            else:
                return 0
        elif "#" == puzzle[first_spring]:
            # No gap case, ##?? [1, ...] case
            return 0
        return solve(clean(puzzle[first_spring + 1 :]), tuple(remaining_springs))
    elif puzzle.startswith("?"):
        return solve(clean("#" + puzzle[1:]), springs) + solve(
            clean(puzzle[1:]), springs
        )


with open("puzzle_12/input.txt", "r") as file:
    lines = file.read().splitlines()
    answer_part_1 = answer_part_2 = 0
    for line in lines:
        answer_part_1 += solve(*preprocess_part_1(line))
        answer_part_2 += solve(*preprocess_part_2(line))
    print(answer_part_1, answer_part_2)
