def main():
    with open("data/input_7.txt", "r") as f:
        positions = list(map(int, f.read().split(",")))

    best_score = float("inf")

    def dist(position, centre):
        d = abs(position - centre)
        return int(0.5 * d * (d + 1))

    for centre in range(max(positions) + 1):
        score = sum(dist(v, centre) for v in positions)
        if score < best_score:
            best_score = score
        else:
            break

    print(best_score)


if __name__ == "__main__":
    main()
