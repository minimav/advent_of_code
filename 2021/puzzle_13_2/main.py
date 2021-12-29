import numpy as np


def make_vertical_fold(dots, value):
    max_x = 2 * value  # max(x for x, _ in dots)
    new_dots = set()
    for x, y in dots:
        if x < value:
            shift = 0
        else:
            shift = -2 * (x - value)
        new_dots.add((x + shift, y))
    return new_dots


def make_horizontal_fold(dots, value):
    max_y = 2 * value  # max(y for _, y in dots)
    new_dots = set()
    for x, y in dots:
        if y < value:
            shift = 0
        elif y > value:
            shift = -2 * (y - value)
        new_dots.add((x, y + shift))
    return new_dots


def main():
    with open("data/input_13.txt", "r") as f:
        folds = []
        dots = set()

        for line in f.readlines():
            line = line.rstrip("\n")
            if line.startswith("fold along"):
                fold = line.split(" ")[-1]
                axis, value = fold.split("=")
                folds.append({"axis": axis, "value": int(value)})
            elif line:
                x, y = map(int, line.split(","))
                dots.add((x, y))

    for fold in folds:
        if fold["axis"] == "x":
            dots = make_vertical_fold(dots, fold["value"])
        else:
            dots = make_horizontal_fold(dots, fold["value"])

    max_x = max(x for x, _ in dots) + 1
    max_y = max(y for _, y in dots) + 1
    output = np.zeros((max_y, max_x))
    for x, y in dots:
        output[y, x] = 1

    # convert array to readable text
    code = "\n".join(["".join([str(int(v)) for v in row]) for row in output])
    print(code.replace("0", " ").replace("1", "#"))


if __name__ == "__main__":
    main()
