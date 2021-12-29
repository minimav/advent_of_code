from collections import Counter, defaultdict


def main():
    with open("data/input_6.txt", "r") as f:
        day_counts = Counter(map(int, f.read().split(",")))

    for _ in range(256):
        new_day_counts = defaultdict(int)
        for k, v in day_counts.items():
            if k > 0:
                new_day_counts[k - 1] += v
            else:
                new_day_counts[6] += v
                new_day_counts[8] = v

        day_counts = new_day_counts

    answer = sum(day_counts.values())
    print(answer)


if __name__ == "__main__":
    main()
