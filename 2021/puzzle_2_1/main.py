import itertools


def main():
    with open("data/input_2.txt", "r") as f:
        inputs = [l.split() for l in f.readlines()]

    start_horiz, start_depth = 0, 0
    for direction, units in inputs:
        print(direction, units)
        units = int(units)
        if direction == "forward":
            start_horiz += units
        elif direction == "up":
            start_depth -= units
        elif direction == "down":
            start_depth += units

    answer = start_horiz * start_depth
    print(answer)


if __name__ == "__main__":
    main()
