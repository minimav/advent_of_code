import heapq


def print_bytes(num_rows: int, num_columns: int, byte_locs: dict[complex, int]):
    for row in range(num_rows):
        print(f"{row:2d}|", end="")
        for column in range(num_columns):
            if row == column == 0:
                print("S", end="")
            elif row == num_rows - 1 and column == num_columns - 1:
                print("E", end="")
            elif complex(column, row) in byte_locs:
                print("#", end="")
            else:
                print(".", end="")
        print()


def part_1(input: str, num_rows: int, num_columns: int, num_bytes: int):
    lines = input.split("\n")
    byte_locs = {}
    for i, line in enumerate(lines):
        if i >= num_bytes:
            break
        column, row = map(int, line.split(","))
        byte_locs[complex(column, row)] = i

    print_bytes(num_rows, num_columns, byte_locs)
    start = complex(0, 0)
    end = complex(num_columns - 1, num_rows - 1)
    counter = 0
    current_best = {}
    on_queue = {(0, start)}
    queue = [(0, counter, start)]
    while queue:
        cost, *_, loc = heapq.heappop(queue)
        on_queue.remove((cost, loc))
        if loc == end:
            print(cost)
            break

        for move in (complex(0, 1), complex(1, 0), complex(-1, 0), complex(0, -1)):
            new_loc = loc + move
            if new_loc in byte_locs:
                continue
            elif not (0 <= new_loc.real < num_columns) or not (
                0 <= new_loc.imag < num_rows
            ):
                continue
            elif new_loc in current_best and current_best[new_loc] < cost + 1:
                continue
            elif (cost + 1, new_loc) in on_queue:
                continue

            counter += 1
            current_best[new_loc] = cost + 1
            on_queue.add((cost + 1, new_loc))
            heapq.heappush(queue, (cost + 1, counter, new_loc))


def part_2(input: str, num_rows: int, num_columns: int, byte_index_start: int):
    lines = input.split("\n")
    all_byte_locs = {}
    for i, line in enumerate(lines):
        column, row = map(int, line.split(","))
        all_byte_locs[complex(column, row)] = i

    start = complex(0, 0)
    end = complex(num_columns - 1, num_rows - 1)
    for byte_index in range(byte_index_start, len(lines)):
        byte_locs = {k: v for k, v in all_byte_locs.items() if v <= byte_index}

        counter = 0
        current_best = {}
        on_queue = {(0, start)}
        queue = [(0, counter, start)]
        suceeded = False
        while queue:
            cost, *_, loc = heapq.heappop(queue)
            on_queue.remove((cost, loc))
            if loc == end:
                suceeded = True
                break

            for move in (complex(0, 1), complex(1, 0), complex(-1, 0), complex(0, -1)):
                new_loc = loc + move
                if new_loc in byte_locs:
                    continue
                elif not (0 <= new_loc.real < num_columns) or not (
                    0 <= new_loc.imag < num_rows
                ):
                    continue
                elif new_loc in current_best and current_best[new_loc] < cost + 1:
                    continue
                elif (cost + 1, new_loc) in on_queue:
                    continue

                counter += 1
                current_best[new_loc] = cost + 1
                on_queue.add((cost + 1, new_loc))
                heapq.heappush(queue, (cost + 1, counter, new_loc))

        if not suceeded:
            print(lines[byte_index])
            break


if __name__ == "__main__":
    with open("puzzle_18/example.txt", "r") as f:
        example = f.read()
        part_1(example, 7, 7, 12)
        part_2(example, 7, 7, byte_index_start=13)

    with open("puzzle_18/input.txt", "r") as f:
        input = f.read()
        part_1(input, 71, 71, 1024)
        part_2(input, 71, 71, byte_index_start=1025)
