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


def main():
    with open("data/input_9.txt", "r") as f:
        heights = np.array(
            [[int(v) for v in line.strip("\n")] for line in f.readlines()]
        )

    y_size, x_size = heights.shape
    answer = 0
    for y, row in enumerate(heights):
        for x, value in enumerate(row):
            if all(
                value < heights[ny, nx]
                for nx, ny in get_neighbours(x, y, x_size, y_size)
            ):
                answer += 1 + value
    print(answer)


if __name__ == "__main__":
    main()
