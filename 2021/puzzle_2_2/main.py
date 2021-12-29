import itertools


def main():
    """
    In addition to horizontal position and depth, you'll also need to track a third value, aim, which
    also starts at 0. The commands also mean something entirely different than you first thought:

        down X increases your aim by X units.
        up X decreases your aim by X units.
        forward X does two things:
        It increases your horizontal position by X units.
        It increases your depth by your aim multiplied by X.
    """
    with open("data/input_2.txt", "r") as f:
        inputs = [l.split() for l in f.readlines()]

    horiz, depth, aim = 0, 0, 0
    for direction, units in inputs:
        units = int(units)
        if direction == "forward":
            horiz += units
            depth += units * aim
        elif direction == "up":
            aim -= units
        elif direction == "down":
            aim += units

    answer = horiz * depth
    print(answer)


if __name__ == "__main__":
    main()
