from collections import Counter


def pairs(x):
    return zip(x, x[1:])


def main():
    with open("data/input_14.txt", "r") as f:
        polymer, *raw_rules = [
            line.rstrip("\n") for line in f.readlines() if line and line != "\n"
        ]

    rules = {}
    for rule in raw_rules:
        pair, replacement = rule.split(" -> ")
        rules[pair] = replacement

    steps = 10
    for step in range(1, steps + 1):
        replacements = {}
        for i, (char_1, char_2) in enumerate(pairs(polymer)):
            pair = char_1 + char_2
            if pair in rules:
                replacements[i] = rules[pair]

        new_polymer = ""
        ordered_replacements = sorted(replacements.items(), key=lambda x: x[0])
        for i, (rep_1, rep_2) in enumerate(pairs(ordered_replacements)):
            rep_1_index, rep_1_char = rep_1
            rep_2_index, rep_2_char = rep_2
            if i == 0:
                new_polymer += polymer[: rep_1_index + 1]

            new_polymer += rep_1_char + polymer[rep_1_index + 1 : rep_2_index + 1]

        # final one
        new_polymer += rep_2_char + polymer[rep_2_index + 1 :]
        polymer = new_polymer

    counts = sorted(Counter(polymer).items(), key=lambda x: x[1])
    print(counts[-1][-1] - counts[0][-1])


if __name__ == "__main__":
    main()
