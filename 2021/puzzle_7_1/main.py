import numpy as np


def main():
    with open("data/input_7.txt", "r") as f:
        positions = list(map(int, f.read().split(",")))

    best = np.median(positions)
    answer = sum(abs(v - best) for v in positions)
    print(answer)


if __name__ == "__main__":
    main()
