import itertools


def parse_input(path):
    with open(path) as f:
        instructions = f.readline().rstrip("\n")
        choices = {}
        for line in f.readlines():
            try:
                node, raw_choice = line.rstrip("\n").split(" = ")
                choices[node] = raw_choice.strip()[1:-1].split(", ")
            except ValueError:
                continue
    return instructions, choices


def part_1_traverse(instructions, choices):
    steps = 0
    node = "AAA"
    end = "ZZZ"
    for char in itertools.cycle(instructions):
        if node == end:
            print(steps)
            break
        left, right = choices[node]
        if char == "L":
            node = left
        else:
            node = right
        steps += 1


def part_2(instructions, choices):
    starts = [n for n in choices if n.endswith("A")]
    ends = {n for n in choices if n.endswith("Z")}
    # 0 47, 1 71, 2 79, 3 67, 4 59, 5 61
    # num_instructions = len(instructions)
    # reccurrences = [47, 71, 79, 67, 59, 61]
    # import math

    # print(len(instructions) * math.lcm(*reccurrences))

    print("starts", starts)
    node = starts[5]
    print(node)
    for index, char in enumerate(itertools.cycle(instructions)):
        if node in ends:
            print(index // len(instructions), index % len(instructions), node)
        left, right = choices[node]
        if char == "L":
            node = left
        else:
            node = right
        if index > 100_000:
            break


# part_1_traverse(*parse_input("puzzle_8/example.txt"))
# part_1_traverse(*parse_input("puzzle_8/input.txt"))
part_2(*parse_input("puzzle_8/input.txt"))
