"""Short script to help figure out where repeats start."""
from collections import defaultdict, Counter


with open("2022/examples/puzzle_17/example_output.txt") as f:
    lines = list(reversed([l.strip("\n") for l in f.readlines()]))
    lines_to_index = defaultdict(list)

    repeating_every = 7
    for index in range(len(lines)):
        chunk = tuple(lines[index : index + repeating_every])
        if len(chunk) == repeating_every:
            lines_to_index[chunk].append(index)

for chunk, indexes in lines_to_index.items():
    print(min(indexes))
    print(Counter([b - a for a, b in zip(indexes, indexes[1:])]))
