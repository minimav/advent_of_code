import itertools


def dice_generator():
    it = itertools.cycle(range(1, 101))
    while True:
        yield tuple(itertools.islice(it, 0, 3))


def main():
    positions = {1: 4, 2: 6}
    scores = {1: 0, 2: 0}
    dice_rolled = 0
    player_turns = itertools.cycle([1, 2])

    for dice, player in zip(dice_generator(), player_turns):
        dice_rolled += 3

        raw_position = positions[player] + sum(dice)
        if raw_position % 10 == 0:
            position = 10
        else:
            position = raw_position % 10
        positions[player] = position
        scores[player] += positions[player]

        # print(
        #    f"Player {player} rolls {dice=}, sum={sum(dice)}, moves to {positions[player]} "
        #    f"for total score {scores[player]}"
        # )

        if scores[player] >= 1000:
            break

    print(f"Player {player} won, scores were {scores}")
    losing_player = 1 if player == 2 else 2
    print(scores[losing_player] * dice_rolled)


if __name__ == "__main__":
    main()
