import heapq


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
        initial_risks = {}
        for row, line in enumerate(f.readlines()):
            for column, value in enumerate(line.rstrip("\n")):
                initial_risks[column, row] = int(value)

    # add in additional 'boards'
    risks = {}
    initial_max_x = len({x for x, _ in initial_risks})
    initial_max_y = len({y for _, y in initial_risks})
    for row in range(5):
        for col in range(5):
            value_shift = col + row
            for (x, y), value in initial_risks.items():
                y_shift = row * initial_max_y + y
                x_shift = col * initial_max_x + x
                new_value = value + value_shift
                # no zeros!
                if new_value > 9:
                    new_value -= 9
                risks[x_shift, y_shift] = new_value

    max_x = max(x for x, _ in risks)
    max_y = max(y for _, y in risks)
    start = (0, 0)
    end = (max_x, max_y)
    best_risks = {start: 0}
    paths = [(0, start)]
    iteration = 1
    while True:
        if iteration % 100 == 0:
            print(
                f"Iteration {iteration} evaluating {len(paths)} paths (up to {max(best_risks)})"
            )

        if not paths:
            break

        new_paths = []
        heapq.heapify(new_paths)
        for risk, (x, y) in paths:
            for nx, ny in get_neighbours(x, y, max_x + 1, max_y + 1):
                new_risk = risk + risks[nx, ny]
                if (nx, ny) not in best_risks or new_risk < best_risks[nx, ny]:
                    heapq.heappush(new_paths, (new_risk, (nx, ny)))
                    best_risks[nx, ny] = new_risk
        paths = new_paths
        iteration += 1

    print(best_risks[end])


if __name__ == "__main__":
    main()
