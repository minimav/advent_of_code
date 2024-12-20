import heapq
from collections import Counter


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
    num_rows = max(int(coord.imag) for coord in boundary) + 1
    num_columns = max(int(coord.real) for coord in boundary) + 1

    counter = 0
    results = {}
    current_best = {}
    # Include has cheated status to force the no-cheat path to be explored first
    queue = [(False, 0, counter, start, None)]
    while queue:
        has_cheated, cost, *_, loc, cheat = heapq.heappop(queue)
        if loc == end:
            results[cheat] = cost
            continue

        # Having cheated there is only one route to the finish
        if has_cheated:
            remainder = results[None] - current_best[(loc, None)]
            final_cost = remainder + cost
            if cheat in results and results[cheat] < final_cost:
                continue
            results[cheat] = final_cost
            continue

        # No cheat moves
        for move in (complex(0, 1), complex(1, 0), complex(-1, 0), complex(0, -1)):
            new_loc = loc + move
            if new_loc in boundary:
                # Invalid move case
                continue
            elif new_loc in non_boundary_walls:
                continue

            key = (new_loc, None)
            if key in current_best and current_best[key] < cost:
                continue

            counter += 1
            current_best[key] = cost + 1
            item = (False, cost + 1, counter, new_loc, None)
            heapq.heappush(queue, item)

        # Cheat moves
        # Can teleport to any square within skip_length Manhattan distance
        for total in range(2, skip_length + 1):
            for horiz in range(total + 1):
                vert = total - horiz
                for move in (
                    complex(horiz, vert),
                    complex(-horiz, vert),
                    complex(horiz, -vert),
                    complex(-horiz, -vert),
                ):
                    new_loc = loc + move
                    if (not (0 < new_loc.real < num_columns - 1)) or (
                        not (0 < new_loc.imag < num_rows - 1)
                    ):
                        # Invalid move case
                        continue
                    elif new_loc in non_boundary_walls:
                        continue

                    cheat = (loc, new_loc)
                    key = (new_loc, cheat)
                    new_cost = cost + total
                    if key in current_best and current_best[key] < new_cost:
                        continue

                    counter += 1
                    current_best[key] = new_cost
                    item = (True, new_cost, counter, new_loc, cheat)
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

    # print_grid(non_boundary_walls, boundary, start, end)
    best_cost = solve(non_boundary_walls, boundary, start, end, skip_length=0)[None]
    print(f"Best cost: {best_cost}")

    costs_with_skips = solve(
        non_boundary_walls, boundary, start, end, skip_length=skip_length
    )

    savings = {k: best_cost - v for k, v in costs_with_skips.items() if best_cost > v}
    print(f"Number cheats: {len(savings)}")
    # print(sorted(Counter(v for v in savings.values() if v >= min_saving).items()))
    answer = sum(1 for v in savings.values() if v >= min_saving)
    print(f"Answer: {answer}")


if __name__ == "__main__":
    with open("puzzle_20/example.txt", "r") as f:
        example = f.read()
        find_cheats(example, skip_length=2, min_saving=100)
        find_cheats(example, skip_length=20, min_saving=50)

    with open("puzzle_20/input.txt", "r") as f:
        input = f.read()
        find_cheats(input, skip_length=2, min_saving=100)
        find_cheats(input, skip_length=20, min_saving=100)
