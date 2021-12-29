from collections import defaultdict


def main():
    mappings = {
        1: frozenset("cf"),
        7: frozenset("acf"),
        4: frozenset("bcdf"),
        2: frozenset("acdeg"),
        3: frozenset("acdfg"),
        5: frozenset("abdfg"),
        0: frozenset("abcefg"),
        6: frozenset("abdefg"),
        9: frozenset("abcdfg"),
        8: frozenset("abcdefg"),
    }
    num_segments = {k: len(v) for k, v in mappings.items()}
    easy_numbers = (1, 4, 7, 8)
    easy_lengths = {num_segments[k]: k for k in easy_numbers}

    answer = 0
    with open("data/input_8.txt", "r") as f:
        for line in f.readlines():
            inputs, outputs = line.split(" | ")
            inputs = inputs.split()
            outputs = [frozenset(output) for output in outputs.split()]

            mixed_up_mappings = {}
            inputs_by_length = defaultdict(list)

            # assign the easy cases
            for input in inputs:
                length = len(input)
                if length in easy_lengths:
                    number = easy_lengths[length]
                    mixed_up_mappings[number] = frozenset(input)
                else:
                    inputs_by_length[length].append(frozenset(input))

            # 1 in 3 not 2 or 5
            for input in inputs_by_length[5]:
                if input.issuperset(mixed_up_mappings[1]):
                    mixed_up_mappings[3] = input
                    break

            # 1 in 0 and 9 but not 6
            for input in inputs_by_length[6]:
                if not input.issuperset(mixed_up_mappings[7]):
                    mixed_up_mappings[6] = input
                    break

            # 4 intersect 2 share 2, share 3 with 3 and 5
            for input in inputs_by_length[5]:
                if len(input.intersection(mixed_up_mappings[4])) == 2:
                    mixed_up_mappings[2] = input
                    break

            # 4 in 9 but not 0 or 6
            for input in inputs_by_length[6]:
                if input.issuperset(mixed_up_mappings[4]):
                    mixed_up_mappings[9] = input
                    break

            # clean up last number of each length with multiple options
            assigned = frozenset(mixed_up_mappings.values())

            for input in inputs_by_length[6]:
                if input not in assigned:
                    mixed_up_mappings[0] = input

            for input in inputs_by_length[5]:
                if input not in assigned:
                    mixed_up_mappings[5] = input

            # decode the output
            wires_to_numbers = {v: str(k) for k, v in mixed_up_mappings.items()}

            decode = ""
            for output in outputs:
                decode += wires_to_numbers[output]

            answer += int(decode)

    print(answer)


if __name__ == "__main__":
    main()
