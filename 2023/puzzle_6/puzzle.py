def solve_for_times_and_distances(times, distances):
    answer = 1
    for t, d in zip(times, distances):
        # p = milliseconds of power, 0 < p < t
        # d < p * (t - p)
        # p ** 2 - p * t + d < 0
        # p = (t +/- sqrt(t ** 2 - 4d)) / 2
        low = (t - (t**2 - 4 * d) ** 0.5) / 2
        high = (t + (t**2 - 4 * d) ** 0.5) / 2

        # Always next integer
        low = int(low) + 1

        if high % 1 == 0:
            high = int(high) - 1
        else:
            high = int(high)
        num_ways = high - low + 1
        answer *= num_ways
    print(answer)


def solve(path):
    with open(path) as f:
        *_, raw_times = f.readline().partition(":")
        *_, raw_distances = f.readline().partition(":")
        times = list(map(int, raw_times.split()))
        distances = list(map(int, raw_distances.split()))
        solve_for_times_and_distances(times, distances)

        time = int(raw_times.replace(" ", ""))
        distance = int(raw_distances.replace(" ", ""))
        solve_for_times_and_distances([time], [distance])


solve("puzzle_6/example.txt")
solve("puzzle_6/input.txt")
