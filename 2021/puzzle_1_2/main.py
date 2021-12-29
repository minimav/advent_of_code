import itertools


def main():
    with open("data/input_1.txt", "r") as f:
        inputs = [int(l.rstrip(" ")) for l in f.readlines()]

    def triples():
        return zip(inputs, inputs[1:], inputs[2:])

    answer = sum(
        sum(a) < sum(b) for a, b in zip(triples(), itertools.islice(triples(), 1, None))
    )
    print(answer)


if __name__ == "__main__":
    main()
