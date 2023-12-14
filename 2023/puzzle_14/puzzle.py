def get_score(grid):
    num_rows = len(grid)
    num_cols = len(grid[0])
    score = 0
    for row in range(num_rows):
        for col in range(num_cols):
            if grid[row][col] == "O":
                score += num_rows - row
    return score


score_test = """OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....""".split()


def tilt(grid):
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            if grid[row][col] != "O":
                continue

            new_row = row
            while new_row - 1 >= 0 and grid[new_row - 1][col] == ".":
                new_row -= 1
            if new_row != row:
                grid[row] = grid[row][:col] + "." + grid[row][col + 1 :]
                grid[new_row] = grid[new_row][:col] + "O" + grid[new_row][col + 1 :]
    return grid


assert get_score(score_test) == 136


def rotate(grid):
    # clockwise(!) each time
    new_grid = []
    for col in range(len(grid[0])):
        new_row = ""
        for row in range(len(grid) - 1, -1, -1):
            new_row += grid[row][col]
        new_grid.append(new_row)
    return new_grid


def cycle(grid):
    for _ in range(4):
        grid = tilt(grid)
        grid = rotate(grid)
    return grid


def spin_cycle(grid, num_cycles: int = 1000000000):
    # Check if state seen before after full cycle, skip ahead if so
    cache = {}
    for index in range(1, num_cycles + 1):
        grid = cycle(grid)
        key = "\n".join(grid)
        if key in cache:
            break
        else:
            cache[key] = index

    cycle_length = index - cache[key]
    remaining = (num_cycles - index) % cycle_length
    for _ in range(remaining):
        grid = cycle(grid)

    return grid


one_cycle = """.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....""".split()

two_cycles = """.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O""".split()


three_cycles = """.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O""".split()


with open("puzzle_14/example.txt") as f:
    example_grid = [line.rstrip("\n") for line in f.readlines()]

with open("puzzle_14/input.txt") as f:
    grid = [line.rstrip("\n") for line in f.readlines()]


# .O#
# #OO
assert rotate([".O#", "#OO"]) == ["#.", "OO", "O#"]
assert rotate(rotate([".O#", "#OO"])) == ["OO#", "#O."]
assert rotate(rotate(rotate(rotate([".O#", "#OO"])))) == [".O#", "#OO"]

assert cycle(example_grid) == one_cycle
assert cycle(cycle(example_grid)) == two_cycles
assert cycle(cycle(cycle(example_grid))) == three_cycles

print(get_score(tilt(example_grid)))
print(get_score(tilt(grid)))
print(get_score(spin_cycle(example_grid)))
print(get_score(spin_cycle(grid)))
