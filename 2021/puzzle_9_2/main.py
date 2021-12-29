import numpy as np


def get_neighbours(x, y, x_size, y_size):
    neighbours = []
    if x < x_size - 1:
        neighbours.append((x + 1, y))
    if x > 0:
        neighbours.append((x - 1, y))
    if y < y_size - 1:
        neighbours.append((x, y + 1))
    if y > 0:
        neighbours.append((x, y - 1))
    return neighbours


def get_basin(x, y, heights):
    y_size, x_size = heights.shape
    basin = {(x, y)}

    def worker(coords, basin, iteration):
        # print(f"Iteration {iteration} with {coords} coords to look at")
        # print(f"Basin currently has {len(basin)} coords")
        next_coords = set()
        for cx, cy in coords:
            for nx, ny in get_neighbours(cx, cy, x_size, y_size):
                if (nx, ny) in basin:
                    continue

                if heights[ny, nx] != 9:
                    next_coords.add((nx, ny))
                    basin.add((nx, ny))

        if next_coords:
            return worker(next_coords, basin, iteration + 1)
        return basin

    return worker({(x, y)}, basin, 0)


def main():
    with open("data/input_9.txt", "r") as f:
        heights = np.array(
            [[int(v) for v in line.strip("\n")] for line in f.readlines()]
        )

    y_size, x_size = heights.shape
    minimums = []
    for y, row in enumerate(heights):
        for x, value in enumerate(row):
            if all(
                value < heights[ny, nx]
                for nx, ny in get_neighbours(x, y, x_size, y_size)
            ):
                minimums.append((x, y))

    basins = {(x, y): get_basin(x, y, heights) for x, y in minimums}
    # for k, v in basins.items():
    #    print(k, len(v), v)
    basins = sorted(basins.values(), key=len)
    print(len(basins[-1]) * len(basins[-2]) * len(basins[-3]))


if __name__ == "__main__":
    main()
