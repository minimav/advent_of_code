from collections import defaultdict


def main():
    counts = defaultdict(lambda: {"0": 0, "1": 0})
    with open("data/input_3.txt", "r") as f:
        for line in f.readlines():
            for i, char in enumerate(line.rstrip("\n")):
                counts[i][char] += 1

    gamma = ""
    epsilon = ""
    num_chars = max(counts)
    for i in range(num_chars + 1):
        if counts[i]["0"] < counts[i]["1"]:
            gamma += "1"
            epsilon += "0"
        else:
            gamma += "0"
            epsilon += "1"

    print(gamma, epsilon)
    print(int(gamma, base=2))
    answer = int(gamma, base=2) * int(epsilon, base=2)
    print(answer)


if __name__ == "__main__":
    main()
