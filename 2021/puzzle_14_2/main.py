from collections import Counter, defaultdict


def pairs(x):
    for char_1, char_2 in zip(x, x[1:]):
        yield char_1 + char_2


def main():
    with open("data/input_14.txt", "r") as f:
        raw_polymer, *raw_rules = [
            line.rstrip("\n") for line in f.readlines() if line and line != "\n"
        ]

    rules = {}
    for rule in raw_rules:
        pair, replacement = rule.split(" -> ")
        rules[pair] = replacement

    start = raw_polymer[:2]
    end = raw_polymer[-2:]
    polymer = defaultdict(int)
    for pair in pairs(raw_polymer):
        polymer[pair] += 1

    steps = 40
    for step in range(1, steps + 1):
        new_polymer = defaultdict(int)
        if start in rules:
            start = start[0] + rules[start]
        if end in rules:
            end = rules[end] + end[-1]

        seen = set()
        for pair, replacement in rules.items():
            if pair not in polymer:
                continue

            seen.add(pair)
            new_polymer[pair[0] + replacement] += polymer[pair]
            new_polymer[replacement + pair[-1]] += polymer[pair]

        for pair, count in polymer.items():
            if pair not in seen:
                new_polymer[pair] += polymer[pair]

        polymer = new_polymer

    # ignore double counting for now
    raw_counts = defaultdict(int)
    for pair, count in polymer.items():
        raw_counts[pair[0]] += count
        raw_counts[pair[-1]] += count

    # #### example counting check ####
    # polymer: NHC
    # polymer pairs counts: NH - 1, HC - 1
    # raw_counts: N - 1, H - 2, C - 1
    # counts: each 1
    counts = {}
    for char, count in raw_counts.items():
        if char == start[0] and char == end[-1]:
            counts[char] = int((count - 2) / 2)
        elif char == start[0] or char == end[-1]:
            counts[char] = int((count + 1) / 2)
        else:
            counts[char] = int(count / 2)

    ordered_counts = sorted(counts.items(), key=lambda x: x[1])
    print(ordered_counts[-1][-1] - ordered_counts[0][-1])


if __name__ == "__main__":
    main()
