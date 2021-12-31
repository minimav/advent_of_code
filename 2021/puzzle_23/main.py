from collections import defaultdict
import heapq


def make_game_class(ROOM_SIZE):
    class GameState:

        room_coords = {
            "A": {(2, y) for y in range(1, 1 + ROOM_SIZE)},
            "B": {(4, y) for y in range(1, 1 + ROOM_SIZE)},
            "C": {(6, y) for y in range(1, 1 + ROOM_SIZE)},
            "D": {(8, y) for y in range(1, 1 + ROOM_SIZE)},
        }
        room_xs = {"A": 2, "B": 4, "C": 6, "D": 8}
        x_to_room = {v: k for k, v in room_xs.items()}
        outside_door_coords = {(x, 0) for x in room_xs.values()}

        # use to check if we're finished
        correct = {c: k for k, coords in room_coords.items() for c in coords}

        # build all possible transitions
        transitions = defaultdict(set)
        for x in range(10):
            transitions[(x, 0)].add((x + 1, 0))
            transitions[(x + 1, 0)].add((x, 0))
        for x in room_xs.values():
            for y in range(ROOM_SIZE):
                transitions[(x, y)].add((x, y + 1))
                transitions[(x, y + 1)].add((x, y))

        move_costs = {"A": 1, "B": 10, "C": 100, "D": 1000}

        def __init__(self, state, score):
            self.state = state
            self.score = score

        def __lt__(self, other):
            """If using queue, process lower scoring games first."""
            return self.score < other.score

        def __str__(self):
            """Pretty print the game like in the problem statement"""
            game = [[" " for _ in range(11)] for _ in range(ROOM_SIZE + 1)]
            game[0] = ["." for _ in range(11)]
            for v, coords in self.room_coords.items():
                for x, y in coords:
                    game[y][x] = "."
            for (x, y), value in self.state.items():
                game[y][x] = value

            str_rows = ["".join(row) for row in game]
            return "\n".join(str_rows)

        def __hash__(self):
            return hash(str(self))

        @classmethod
        def from_str(cls, s):
            """Build a game instance from the string representation."""
            state = {}
            for y, row in enumerate(s.split("\n")):
                for x, value in enumerate(row):
                    if value in set("ABCD"):
                        state[x, y] = value
            return cls(state, 0)

        def finished(self):
            """Are all amphipods in the correct places?"""
            for k, v in self.state.items():
                if k not in self.correct or self.correct[k] != v:
                    return False
            return True

        def in_corridor(self, coord):
            """Is an amphipod in the corridor?"""
            return coord[1] == 0

        def all_lower_correct(self, coord, value):
            return all(
                self.state.get((coord[0], y)) == value
                for y in range(coord[1] + 1, ROOM_SIZE + 1)
            )

        def outside_door(self, coord):
            return coord in self.outside_door_coords

        def correct_room_above_wrong(self, coord, value):
            return coord[0] == self.room_xs[value] and not self.all_lower_correct(
                coord, value
            )

        def one_room_to_not_own(self, start_room, dest_in_corridor, dest, value):
            return (
                start_room is not None
                and start_room != value
                and not dest_in_corridor
                and dest[0] != self.room_xs[value]
            )

        def possible_moves_for_amphipod(self, start_coord, value, verbose=False):
            # if verbose:
            #    print(f"Considering {value} at {start_coord}")
            moves = []
            # already in the correct place, either at bottom or all of this value correct lower
            if start_coord in self.room_coords[value] and self.all_lower_correct(
                start_coord, value
            ):
                # if verbose:
                #    print(f"XXX {value} is at bottom in correct door, skipping")
                return moves

            start_in_corridor = self.in_corridor(start_coord)
            start_room = None
            if not start_in_corridor and start_coord[0] in self.x_to_room:
                start_room = self.x_to_room[start_coord[0]]

            # if verbose:
            #    print(f"{start_in_corridor=}, {start_room=}")

            current_coords = [(None, start_coord, 0)]
            while current_coords:
                next_current_coords = []
                for prev, coord, score in current_coords:
                    for dest in self.transitions[coord]:
                        # if verbose:
                        #    print(f"Considering {coord} -> {dest} move")
                        # can't move through something else
                        if dest in self.state:
                            # if verbose:
                            #    print(f"XXX Can't move to {dest} as something there")
                            continue
                        # can't move backwards
                        if dest == prev:
                            # if verbose:
                            #    print(f"XXX Can't move to {dest} as was just there")
                            continue

                        dest_in_corridor = self.in_corridor(dest)

                        # moving from corridor into room, can only be its own
                        if (
                            start_in_corridor
                            and not dest_in_corridor
                            and dest[0] != self.room_xs[value]
                        ):
                            # if verbose:
                            #    print(
                            #        f"XXX Can't move to {dest} as corridor --/--> other room"
                            #    )
                            continue

                        # consider move in next iteration but check if it's a valid ending spot
                        move_score = score + self.move_costs[value]
                        # if verbose:
                        #    print(f"Allowing {coord} -> {dest} move")
                        next_current_coords.append((coord, dest, move_score))

                        # check if we can finish here
                        if (
                            not self.outside_door(dest)
                            and not (start_in_corridor and dest_in_corridor)
                            and not self.correct_room_above_wrong(dest, value)
                            and not self.one_room_to_not_own(
                                start_room, dest_in_corridor, dest, value
                            )
                            and not (start_room == value and not dest_in_corridor)
                        ):
                            moves.append((start_coord, dest, value, move_score))

                current_coords = next_current_coords

            yield from moves

        def possible_moves(self, verbose=False):
            for coord, value in self.state.items():
                yield from self.possible_moves_for_amphipod(
                    coord, value, verbose=verbose
                )

    return GameState


def main(game_cls, start_game):
    end_game = game_cls(game_cls.correct, float("inf"))
    print(start_game)
    print()
    print(end_game)
    best_scores = {str(start_game): 0}
    current_games = {start_game}

    iteration = 1
    while current_games:
        print(f"{len(current_games)} games to consider at iteration {iteration}")

        next_current_games = set()
        for game in current_games:
            for origin, dest, value, move_score in game.possible_moves():
                new_state = game.state.copy()
                del new_state[origin]
                new_state[dest] = value
                new_game = game_cls(new_state, game.score + move_score)

                if (
                    str(new_game) not in best_scores
                    or best_scores[str(new_game)] > new_game.score
                ):
                    best_scores[str(new_game)] = new_game.score
                    if not new_game.finished():
                        next_current_games.add(new_game)
                    else:
                        print(f"Finished with score {new_game.score}")

        iteration += 1
        current_games = next_current_games

    print(best_scores[str(end_game)])


def test_example_1():
    game_cls = make_game_class(ROOM_SIZE=2)
    # should be solvable from each of the steps
    steps = {
        6: """.........A.\n  .#B#C#D\n  A#B#C#D""",
        5: """.....D.D.A.\n  .#B#C#.\n  A#B#C#.#""",
        4: """.....D.....\n  .#B#C#D\n  A#B#C#A""",
        3: """.....D.....\n  B#.#C#D\n  A#B#C#A""",
        2: """...B.......\n  B#.#C#D\n  A#D#C#A""",
        1: """...B.......\n  B#C#.#D\n  A#D#C#A""",
        0: """...........\n  B#C#B#D\n  A#D#C#A""",
    }
    game = game_cls.from_str(steps[0])
    main(game_cls, game)


def test_example_2():
    game_cls = make_game_class(ROOM_SIZE=4)
    # should be solvable from each of the steps (backwards from solution)
    steps = {
        -2: """.........AD\n  .#B#C#.\n  A#B#C#D\n  A#B#C#D\n  A#B#C#D""",
        -4: """A..D.....AD\n  .#B#C#.\n  .#B#C#.\n  A#B#C#D\n  A#B#C#D""",
        -10: """AA.D......D\n  B#.#C#.\n  D#B#C#.\n  D#B#C#.\n  A#B#C#A""",
        -15: """AA...B.B.BD\n  B#.#.#.\n  D#.#C#.\n  D#.#C#C\n  A#D#C#A""",
        -16: """AA.....B.BD\n  B#.#.#.\n  D#.#C#.\n  D#B#C#C\n  A#D#C#A""",
        -18: """AA.....B.BD\n  B#C#.#.\n  D#C#.#.\n  D#B#.#C\n  A#D#C#A""",
        -20: """A........BD\n  B#C#.#.\n  D#C#B#.\n  D#B#A#C\n  A#D#C#A""",
    }
    game = game_cls.from_str(steps[-16])
    main(game_cls, game)


if __name__ == "__main__":
    state_1 = """...........
  B#A#B#C
  C#D#D#A"""

    state_2 = """...........
  B#A#B#C
  D#C#B#A
  D#B#A#C
  C#D#D#A"""

    state = state_2
    game_cls = make_game_class(ROOM_SIZE=len(state.split("\n")) - 1)
    start_game = game_cls.from_str(state)
    main(game_cls, start_game)
