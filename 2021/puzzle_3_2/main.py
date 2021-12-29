from collections import defaultdict
from operator import eq, ne

def main():
    """
    To find oxygen generator rating, determine the most common value (0 or 1) in the current
    bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally
    common, keep values with a 1 in the position being considered.
    
    To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit
    position, and keep only numbers with that bit in that position. If 0 and 1 are equally common,
    keep values with a 0 in the position being considered.
    """

    def make_counts(inputs, char_index):
        counts = {"0": 0, "1": 0}
        for input in inputs:
            counts[input[char_index]] += 1
        return counts


    def get_most_common_bit(counts):
        if counts["0"] < counts["1"]:
            return "1"
        elif counts["1"] < counts["0"]:
            return "0"
        else:
            return "="


    def filter(inputs, char_index, most_common=True):
        if len(inputs) == 1:
            return inputs.pop()

        counts = make_counts(inputs, char_index)
        most_common_bit = get_most_common_bit(counts)

        operator = eq if most_common else ne
        new_inputs = []
        for input in inputs:
            if most_common_bit == "=":
                if most_common and input[char_index] == "1":
                    new_inputs.append(input)
                elif not most_common and input[char_index] == "0":
                    new_inputs.append(input)
            elif operator(input[char_index], most_common_bit):
                new_inputs.append(input) 

        return filter(new_inputs, char_index + 1, most_common=most_common)


    with open("data/input_3.txt", "r") as f:
        inputs = [line.rstrip("\n") for line in f.readlines()]

    oxygen_generator = filter(inputs, 0, most_common=True)
    c02_scrubber = filter(inputs, 0, most_common=False)
    
    answer = int(oxygen_generator, base=2) * int(c02_scrubber, base=2)
    print(answer)


if __name__ == "__main__":
    main()
