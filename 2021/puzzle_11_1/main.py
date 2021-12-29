def get_neighbours(x, y, x_size=10, y_size=10):
    neighbours = []
    if x < x_size - 1:
        neighbours.append((x + 1, y))
    if x > 0:
        neighbours.append((x - 1, y))
    if y < y_size - 1:
        neighbours.append((x, y + 1))
    if y > 0:
        neighbours.append((x, y - 1))

    # diagonal neighours
    if x < x_size - 1 and y < y_size - 1:
        neighbours.append((x + 1, y + 1))
    if x < x_size - 1 and y > 0:
        neighbours.append((x + 1, y - 1))
    if x > 0 and y < y_size - 1:
        neighbours.append((x - 1, y + 1))
    if x > 0 and y > 0:
        neighbours.append((x - 1, y - 1))
    return neighbours


def main():
    with open("data/input_11.txt", "r") as f:
        state = {}
        for y, row in enumerate(f.readlines()):
            for x, value in enumerate(row.rstrip("\n")):
                state[x, y] = int(value)

    def run_step(input_state):
        state = {k: v + 1 for k, v in input_state.items()}
        flashed = {k for k, v in state.items() if v > 9}

        def worker(state, coords, num_flashes, already_flashed):
            if not coords:
                return state, num_flashes

            next_coords = set()
            for coord in coords:
                state[coord] = 0
                num_flashes += 1
                already_flashed.add(coord)
                neighbours = [n for n in get_neighbours(*coord) if n not in coords]
                for neighbour in neighbours:
                    if neighbour not in already_flashed:
                        state[neighbour] += 1
                        if state[neighbour] > 9:
                            next_coords.add(neighbour)

            return worker(state, next_coords, num_flashes, flashed)

        state, num_flashed = worker(state, flashed, 0, set())
        assert all(0 <= v <= 9 for v in state.values())
        return state, num_flashed

    num_flashes = 0
    for step in range(100):
        state, flashes_in_step = run_step(state)
        num_flashes += flashes_in_step

    print(num_flashes)


if __name__ == "__main__":
    main()
