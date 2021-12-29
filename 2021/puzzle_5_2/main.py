from collections import defaultdict
from operator import add, sub


def null_op(value, increment):
    return value


def get_op(start, end):
    if start == end:
        return null_op
    elif start < end:
        return add
    else:
        return sub


def main():
    counts = defaultdict(int)
    with open("data/input_5.txt", "r") as f:
        for line in f.readlines():
            raw_start, raw_end = line.split(" -> ")
            row_start, col_start = map(int, raw_start.split(","))
            row_end, col_end = map(int, raw_end.split(","))

            row_op = get_op(row_start, row_end)
            col_op = get_op(col_start, col_end)
            max_diff = max(abs(col_start - col_end), abs(row_start - row_end))
            for diff in range(max_diff + 1):
                counts[row_op(row_start, diff), col_op(col_start, diff)] += 1

    answer = sum(1 for c in counts.values() if c > 1)
    print(answer)


if __name__ == "__main__":
    main()
