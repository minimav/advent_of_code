def part_1(path):
    with open(path) as f:
        games = f.readlines()
    answer = 0
    for game in games:
        start, end = game.split(": ")

        winning, hand = end.split("|")
        winning = set(int(x) for x in winning.split(" ") if x)
        hand = set(int(x) for x in hand.split(" ") if x)
        matches = len(winning.intersection(hand))
        if matches:
            answer += 2 ** (matches - 1)
    print(answer)


def part_2(path):
    with open(path) as f:
        raw_games = f.readlines()

    games = {}
    for game in raw_games:
        start, end = game.split(": ")
        game_number = int(start.split(" ")[-1])

        winning, hand = end.split("|")
        winning = set(int(x) for x in winning.split(" ") if x)
        hand = set(int(x) for x in hand.split(" ") if x)
        matches = len(winning.intersection(hand))
        games[game_number] = matches

    def traverse(game_number):
        matches = games[game_number]
        return 1 + sum(
            traverse(i) for i in range(game_number + 1, game_number + matches + 1)
        )

    print(sum(traverse(game_number) for game_number in games.keys()))


part_1("puzzle_4/example.txt")
part_1("puzzle_4/input.txt")
part_2("puzzle_4/example.txt")
part_2("puzzle_4/input.txt")
