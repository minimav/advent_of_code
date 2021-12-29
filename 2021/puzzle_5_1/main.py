from collections import defaultdict


def main():
    counts = defaultdict(int)
    with open("data/input_5.txt", "r") as f:
        for line in f.readlines():
            raw_start, raw_end = line.split(" -> ")
            row_start, col_start = map(int, raw_start.split(","))
            row_end, col_end = map(int, raw_end.split(","))

            if row_start == row_end:
                col_min = min(col_start, col_end)
                col_max = max(col_start, col_end)
                for col in range(col_min, col_max + 1):
                    counts[row_start, col] += 1
            elif col_start == col_end:
                row_min = min(row_start, row_end)
                row_max = max(row_start, row_end)
                for row in range(row_min, row_max + 1):
                    counts[row, col_start] += 1
            else:
                pass

    answer = sum(1 for c in counts.values() if c > 1)
    print(answer)


if __name__ == "__main__":
    main()
