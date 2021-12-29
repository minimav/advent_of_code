def main():
    with open("data/input_1.txt", "r") as f:
        inputs = [int(l.rstrip(" ")) for l in f.readlines()]

    answer = sum(a < b for a, b in zip(inputs, inputs[1:]))
    print(answer)


if __name__ == "__main__":
    main()
