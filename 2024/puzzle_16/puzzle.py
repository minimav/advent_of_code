import heapq
import time
from collections import defaultdict
from pathlib import Path


def timer(name):
    def decorator(f):
        def wrapped(*args, **kwargs):
            start = time.time()
            output = f(*args, **kwargs)
            print(f"{name} took {time.time() - start:.3f} seconds")
            return output

        return wrapped

    return decorator


def print_grid(
    grid: dict[complex, str],
    start: complex,
    end: complex,
    best: set[complex] | None = None,
):
    num_rows = max(c.imag for c in grid) + 1
    num_columns = max(c.real for c in grid) + 1
    for row in range(int(num_rows)):
        for column in range(int(num_columns)):
            coord = complex(column, row)
            value = grid.get(coord)
            if coord == start:
                print("S", end="")
            elif coord == end:
                print("E", end="")
            elif best is not None and coord in best:
                print("O", end="")
            elif value is not None:
                print(value, end="")
            else:
                print(".", end="")
        print()


rotations = {
    "E": ("N", "S"),
    "W": ("N", "S"),
    "S": ("E", "W"),
    "N": ("E", "W"),
}

moves = {
    "N": complex(0, 1),
    "E": complex(1, 0),
    "W": complex(-1, 0),
    "S": complex(0, -1),
}


def parse_grid(input_: str) -> tuple[dict[complex, str], complex, complex]:
    grid = {}
    start = end = None
    for y, row in enumerate(input_.split("\n")):
        for x, value in enumerate(row):
            coords = complex(x, y)
            if value == "S":
                start = coords
            elif value == "E":
                end = coords
            elif value == "#":
                grid[coords] = value
    return grid, start, end


@timer("part_1")
def part_1(input_: str):
    grid, start, end = parse_grid(input_)

    # print_grid(grid, start, end)

    current_best = {}
    queue = [(0, 0, start, "E")]
    counter = 1
    while queue:
        cost, _, location, direction = heapq.heappop(queue)
        key = (location, direction)
        if key in current_best and current_best[key] < cost:
            continue

        current_best[key] = cost
        for rotate_direction in rotations[direction]:
            heapq.heappush(queue, (cost + 1000, counter, location, rotate_direction))
            counter += 1

        new_location = location + moves[direction]
        if new_location in grid:
            continue
        elif new_location == end:
            print(cost + 1)
            return
        else:
            heapq.heappush(queue, (cost + 1, counter, new_location, direction))
            counter += 1


@timer("part_2")
def part_2(input_: str):
    grid, start, end = parse_grid(input_)
    # print_grid(grid, start, end)

    best_route_locations = defaultdict(lambda: {start, end})
    current_best = {}
    queue = [(0, 0, start, "E", {start})]
    counter = 1
    while queue:
        cost, _, location, direction, route = heapq.heappop(queue)
        key = (location, direction)
        if key in current_best and current_best[key] < cost:
            continue

        current_best[key] = cost
        for rotate_direction in rotations[direction]:
            heapq.heappush(
                queue, (cost + 1000, counter, location, rotate_direction, route)
            )
            counter += 1

        new_location = location + moves[direction]
        new_cost = cost + 1
        if new_location in grid:
            continue
        elif new_location == end:
            current_best[key] = new_cost
            best_route_locations[new_cost].update(list(route))
            continue
        else:
            new_route = {*list(route), new_location}
            heapq.heappush(
                queue, (new_cost, counter, new_location, direction, new_route)
            )
            counter += 1

    best_cost = min(best_route_locations)
    # print_grid(grid, start, end, best=best_route_locations[best_cost])
    print(len(best_route_locations[best_cost]))


if __name__ == "__main__":
    folder = Path("puzzle_16")
    file_names = [
        "example_1.txt",
        "example_2.txt",
        "input.txt",
    ]
    for file_name in file_names:
        path = folder / file_name
        print(path)
        print("-" * len(str(path)))
        with open(path, "r") as f:
            input_ = f.read()
            part_1(input_)
            part_2(input_)
        print()
