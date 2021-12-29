def main():

    mappings = {
        0: set("abcefg"),
        1: set("cf"),
        2: set("acdeg"),
        3: set("acdfg"),
        4: set("bcdf"),
        5: set("abdfg"),
        6: set("abdefg"),
        7: set("acf"),
        8: set("abcdefg"),
        9: set("abcdfg"),
    }
    num_segments = {k: len(v) for k, v in mappings.items()}
    easy_lengths = {num_segments[k] for k in (1, 4, 7, 8)}

    count = 0
    with open("data/input_8.txt", "r") as f:
        for line in f.readlines():
            inputs, outputs = line.split(" | ")

            inputs = inputs.split()
            outputs = outputs.split()
            for output in outputs:
                if len(output) in easy_lengths:
                    count += 1

    print(count)


if __name__ == "__main__":
    main()
