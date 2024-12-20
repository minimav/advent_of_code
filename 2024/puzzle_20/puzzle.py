import heapq
from collections import defaultdict


def print_grid(non_boundary_walls, boundary, start, end):
    num_rows = max(int(coord.imag) for coord in boundary) + 1
    num_columns = max(int(coord.real) for coord in boundary) + 1
    for row in range(num_rows):
        for column in range(num_columns):
            coord = complex(column, row)
            if coord in non_boundary_walls:
                print("#", end="")
            elif coord in boundary:
                print("@", end="")
            elif coord == start:
                print("S", end="")
            elif coord == end:
                print("E", end="")
            else:
                print(".", end="")
        print()


def solve(
    non_boundary_walls: set[complex],
    boundary: set[complex],
    start: complex,
    end: complex,
    skip_length: int = 0,
) -> int:
    counter = 0
    results = {}
    current_best = {}
    queue = [(0, counter, start, 0, [None, None])]
    while queue:
        cost, *_, loc, cheat_cost, cheat_history = heapq.heappop(queue)
        if loc == end:
            # Cheat is uniquely defined by start and end - need length?
            cheat_key = (cheat_history[0], cheat_history[1])
            results[cheat_key] = cost
            continue

        for move in (complex(0, 1), complex(1, 0), complex(-1, 0), complex(0, -1)):
            new_loc = loc + move
            if new_loc in boundary:
                # Invalid move case
                continue

            new_cheat_history = [None, None]
            new_cheat_cost = cheat_cost

            if new_loc not in non_boundary_walls:
                # No need to cheat case
                if new_loc == cheat_history[0]:
                    # Prevent going back to start of the cheat
                    continue
                if loc == cheat_history[1]:
                    # Record correct end location of the cheat
                    # Already during the cheat that the transition keeps us
                    # under the skip length
                    new_cheat_history = [cheat_history[0], new_loc]
            else:
                # Need to cheat case
                if skip_length == 0:
                    # Cheating not allowed for this param
                    continue
                if cheat_cost >= skip_length - 1:
                    # Another step into a wall would be mean we'd need a further
                    # step to get out of the wall
                    # print("Can't continue cheating")
                    continue
                elif cheat_cost == 0:
                    # Starting a cheat
                    new_cheat_history = [loc, new_loc]
                    # print("Starting a cheat")
                else:
                    # Continuing a cheat
                    new_cheat_history = [cheat_history[0], new_loc]

                new_cheat_cost += 1

            key = (new_loc, tuple(new_cheat_history))
            if key in current_best and current_best[key] < cost + 1:
                continue

            counter += 1
            current_best[key] = cost + 1
            item = (cost + 1, counter, new_loc, new_cheat_cost, new_cheat_history)
            # print(f"Queueing {item}")
            heapq.heappush(queue, item)
    return results


def find_cheats(input: str, skip_length: int, min_saving: int):
    lines = input.split("\n")
    boundary = set()
    non_boundary_walls = set()
    start = end = None
    for row, line in enumerate(lines):
        for column, char in enumerate(line):
            coord = complex(column, row)
            if char == "#":
                if (
                    row == 0
                    or row == len(lines) - 1
                    or column == 0
                    or column == len(line) - 1
                ):
                    boundary.add(coord)
                else:
                    non_boundary_walls.add(coord)
            elif char == "S":
                start = coord
            elif char == "E":
                end = coord

    print_grid(non_boundary_walls, boundary, start, end)
    best_cost = solve(non_boundary_walls, boundary, start, end, skip_length=0)[
        None, None
    ]
    print(f"Best cost: {best_cost}")

    costs_with_skips = solve(
        non_boundary_walls, boundary, start, end, skip_length=skip_length
    )

    savings = {k: best_cost - v for k, v in costs_with_skips.items() if best_cost > v}
    print(f"Number cheats: {len(savings)}")
    answer = sum(v for v in costs_with_skips.values() if v >= min_saving)
    print(f"Answer: {answer}")


def part_2(input: str):
    pass


if __name__ == "__main__":
    with open("puzzle_20/example.txt", "r") as f:
        example = f.read()
        find_cheats(example, skip_length=2, min_saving=100)
        # find_cheats(example, skip_length=20, min_saving=50)

    with open("puzzle_20/input.txt", "r") as f:
        input = f.read()
        find_cheats(input, skip_length=2, min_saving=100)
        # find_cheats(input, skip_length=20, min_saving=100)
