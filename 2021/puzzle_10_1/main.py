scores = {")": 3, "]": 57, "}": 1197, ">": 25137}

complements = {
    # "{": "}",
    "}": "{",
    # "[": "]",
    "]": "[",
    # "<": ">",
    ">": "<",
    # "(": ")",
    ")": "(",
}


def main():
    with open("data/input_10.txt", "r") as f:
        lines = f.readlines()

    answer = 0
    for line in lines:
        stack = []
        for char in line:
            if not stack or char not in complements:
                stack.append(char)
            elif stack[-1] == complements[char]:
                stack.pop()
            else:
                answer += scores[char]
                break

    print(answer)


if __name__ == "__main__":
    main()
