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
    with open("data/input_15.txt", "r") as f:
        risks = {}
        for row, line in enumerate(f.readlines()):
            for column, value in enumerate(line.rstrip("\n")):
                risks[row, column] = int(value)

    max_x = max(x for x, _ in risks)
    max_y = max(y for _, y in risks)
    start = (0, 0)
    end = (max_x, max_y)
    best_risks = {start: 0}
    paths = [([start], 0)]
    while True:
        if not paths:
            break

        new_paths = []
        for path, risk in paths:
            x, y = path[-1]
            for nx, ny in get_neighbours(x, y, max_x + 1, max_y + 1):
                new_risk = risk + risks[nx, ny]
                if (nx, ny) not in best_risks or new_risk < best_risks[nx, ny]:
                    new_path = path.copy() + [(nx, ny)]
                    new_paths.append((new_path, new_risk))
                    best_risks[nx, ny] = new_risk
        paths = new_paths

    print(best_risks[end])


if __name__ == "__main__":
    main()
