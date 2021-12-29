def make_vertical_fold(dots, value):
    max_x = max(x for x, _ in dots)
    max_y = max(y for _, y in dots)
    no_overlap = max_x - value <= value
    new_dots = set()
    for x, y in dots:
        if x < value:
            if no_overlap:
                shift = 0
            else:
                shift = max_x - value
        else:
            if no_overlap:
                shift = -2 * (x - value)
            else:
                shift = -2 * x + max_x

        new_dots.add((x + shift, y))
        # print((x, y), (x, y + shift))
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

    for fold in folds[:1]:
        dots = make_vertical_fold(dots, fold["value"])

    print(len(dots))


if __name__ == "__main__":
    main()
