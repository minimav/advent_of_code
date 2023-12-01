substrings = {
    s: i + 1
    for i, s in enumerate(
        ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
    )
}


def get_digits(line):
    digits = []
    for i in range(len(line)):
        if line[i].isdigit():
            digits.append(int(line[i]))
        else:
            for substring, value in substrings.items():
                if line[i:].startswith(substring):
                    digits.append(value)
                    break
    return digits[0] * 10 + digits[-1]


assert get_digits("6") == 66
assert get_digits("11") == 11
assert get_digits("1o1") == 11
assert get_digits("eighthree") == 83


with open("input_1.txt") as f:
    print(sum(get_digits(line) for line in f))

with open("example_1_2.txt") as f:
    print(sum(get_digits(line) for line in f))
