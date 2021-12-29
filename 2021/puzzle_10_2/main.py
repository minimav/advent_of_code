scores = {")": 1, "]": 2, "}": 3, ">": 4}

end_complements = {"}": "{", "]": "[", ">": "<", ")": "("}

start_complements = {
    "{": "}",
    "[": "]",
    "<": ">",
    "(": ")",
}


def main():
    with open("data/input_10.txt", "r") as f:
        lines = [l.rstrip("\n") for l in f.readlines()]

    line_scores = []
    for line in lines:
        stack = []
        corrupted = False
        for char in line:
            if not stack or char not in end_complements:
                stack.append(char)
            elif stack[-1] == end_complements[char]:
                stack.pop()
            else:
                corrupted = True
                break

        if corrupted:
            continue

        line_score = 0
        for char in reversed(stack):
            line_score *= 5
            line_score += scores[start_complements[char]]
        line_scores.append(line_score)

    num_incomplete = len(line_scores)
    middle_index = int((num_incomplete - 1) / 2)
    answer = sorted(line_scores)[middle_index]
    print(answer)


if __name__ == "__main__":
    main()
