from collections import defaultdict
import copy
from dataclasses import dataclass
import itertools


@dataclass(frozen=True)
class Player:
    position: int
    score: int

    def move(self, roll):
        raw_position = self.position + roll
        if raw_position % 10 == 0:
            position = 10
        else:
            position = raw_position % 10
        return position, self.score + position

    def won(self):
        return self.score >= 21


@dataclass(frozen=True)
class Game:
    to_play: int
    player_1: Player
    player_2: Player

    def move(self, roll):
        if self.to_play == 1:
            position, score = self.player_1.move(roll)
            to_play = 2
            players = [Player(position, score), copy.copy(self.player_2)]
        else:
            position, score = self.player_2.move(roll)
            to_play = 1
            players = [copy.copy(self.player_1), Player(position, score)]
        return Game(to_play, *players)

    def finished(self):
        return self.player_1.won() or self.player_2.won()


dice_combinations_per_score = defaultdict(int)
for roll in itertools.product(range(1, 4), repeat=3):
    dice_combinations_per_score[sum(roll)] += 1


def main():
    game_transitions = defaultdict(lambda: defaultdict(int))

    # for example
    # start_game = Game(1, Player(4, 0), Player(8, 0))
    # for full puzzle
    start_game = Game(1, Player(4, 0), Player(6, 0))

    games = {start_game}
    iteration = 1
    while len(games) > 0:
        print(f"{iteration=}, {len(games)} games to consider")
        next_games = set()
        for game in games:
            for roll, num_games in dice_combinations_per_score.items():
                new_game = game.move(roll)
                key = (game, new_game)
                if (
                    game not in game_transitions
                    or new_game not in game_transitions[game]
                ):
                    game_transitions[game][new_game] = num_games
                    if not new_game.finished():
                        next_games.add(new_game)

        games = next_games
        iteration += 1

    # count games
    # find all paths to each terminal node taking product along the way
    won_games = {1: 0, 2: 0}
    path_counts = {start_game: 1}
    while len(path_counts) > 0:
        next_path_counts = defaultdict(int)
        for game, num_paths in path_counts.items():
            if game.finished():
                if game.player_1.won():
                    won_games[1] += num_paths
                else:
                    won_games[2] += num_paths
                continue

            # game isn't finished, so incorporate all possible transitions
            for next_game, num_transitions in game_transitions[game].items():
                next_path_counts[next_game] += num_paths * num_transitions

        path_counts = next_path_counts

    print(won_games[1], won_games[2])


if __name__ == "__main__":
    main()
