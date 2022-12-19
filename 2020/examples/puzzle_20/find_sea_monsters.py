from copy import deepcopy

import numpy as np


with open("2020/examples/puzzle_20/grid.txt") as f:
    grid = []
    for line in f.read().splitlines():
        grid.append(list(line))

total_count = sum(sum(1 for char in row if char == "#") for row in grid)
print(f"{total_count} #s in the grid")

monster_1 = "                  # "
monster_2 = "#    ##    ##    ###"
monster_3 = " #  #  #  #  #  #   "

offsets_1 = [i for i, char in enumerate(monster_1) if char == "#"]
offsets_2 = [i for i, char in enumerate(monster_2) if char == "#"]
offsets_3 = [i for i, char in enumerate(monster_3) if char == "#"]

total_monster_chars = len(offsets_1) + len(offsets_2) + len(offsets_3)

grid_size = len(grid)

# create transformations
flip_vertical = deepcopy(grid)
for row_index in range(grid_size):
    flip_vertical[row_index] = list(reversed(flip_vertical[row_index]))

flip_horizontal = deepcopy(grid)
for row_index in range(grid_size):
    flip_horizontal[row_index] = grid[grid_size - row_index - 1]

grid_transformations = [
    grid,
    np.rot90(np.array(deepcopy(grid))),
    np.rot90(np.rot90(np.array(deepcopy(grid)))),
    np.rot90(np.rot90(np.rot90(np.array(deepcopy(grid))))),
    flip_horizontal,
    np.rot90(np.array(deepcopy(flip_horizontal))),
    np.rot90(np.rot90(np.array(deepcopy(flip_horizontal)))),
    np.rot90(np.rot90(np.rot90(np.array(deepcopy(flip_horizontal))))),
]

for grid_trans in grid_transformations:
    to_remove = set()
    num_removed = 0
    for row_index in range(grid_size - 3):
        for column_index in range(grid_size - len(monster_1)):
            if not all(
                grid_trans[row_index][column_index + offset] == "#"
                for offset in offsets_1
            ):
                continue
            if not all(
                grid_trans[row_index + 1][column_index + offset] == "#"
                for offset in offsets_2
            ):
                continue
            if not all(
                grid_trans[row_index + 2][column_index + offset] == "#"
                for offset in offsets_3
            ):
                continue

            print(row_index, column_index)
            print(
                "".join(
                    grid_trans[row_index, column_index : column_index + len(monster_1)]
                )
            )
            print(
                "".join(
                    grid_trans[
                        row_index + 1, column_index : column_index + len(monster_2)
                    ]
                )
            )
            print(
                "".join(
                    grid_trans[
                        row_index + 2, column_index : column_index + len(monster_3)
                    ]
                )
            )
            print()
            for offset in offsets_1:
                to_remove.add((row_index, column_index + offset))
            for offset in offsets_2:
                to_remove.add((row_index + 1, column_index + offset))
            for offset in offsets_3:
                to_remove.add((row_index + 2, column_index + offset))
            num_removed += len(offsets_1) + len(offsets_2) + len(offsets_3)

    if to_remove:
        print(f"{num_removed} #s removed not counting duplicates")
        print(f"{total_count - len(to_remove)} remaining #s")
