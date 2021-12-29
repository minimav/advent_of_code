from dataclasses import dataclass, field
from collections import defaultdict
import typing as tp

Position = tp.Tuple[int, int]


@dataclass
class BingoBoard:
    raw_board: str
    board_size: int
    positions: tp.Dict[int, Position] = field(init=False)
    marked: tp.Dict[Position, bool] = field(init=False)

    def __post_init__(self):
        self.marked = {
            (row, column): False
            for row in range(self.board_size)
            for column in range(self.board_size)
        }
        self.positions = {}
        for row_index, row in enumerate(self.raw_board):
            for col_index, number in enumerate(row.rstrip("\n").split()):
                self.positions[int(number)] = (row_index, col_index)

    def mark_called(self, called: int):
        if called in self.positions:
            self.marked[self.positions[called]] = True

    def bingo(self):
        for row in range(self.board_size):
            if all(self.marked[row, i] for i in range(self.board_size)):
                return True
        for column in range(self.board_size):
            if all(self.marked[i, column] for i in range(self.board_size)):
                return True
        return False

    def score(self, last_called):
        return (
            sum(n for n, p in self.positions.items() if not self.marked[p])
            * last_called
        )


def main():

    board_size = 5
    with open("data/input_4.txt", "r") as f:
        lines = [line for line in f.readlines() if line != "\n"]
        numbers_called = [int(n) for n in lines[0].split(",")]

        boards = {}
        num_boards = int((len(lines) - 1) / board_size)
        for index in range(num_boards):
            raw_board = lines[1 + index * board_size : 1 + (index + 1) * board_size]
            boards[index] = BingoBoard(raw_board, board_size)

        for called in numbers_called:
            new_boards = {}
            for index, board in boards.items():
                board.mark_called(called)
                if not board.bingo():
                    new_boards[index] = board
                elif len(boards) > 1 and board.bingo():
                    print(f"Index {index} board won, not the last board though!")
                elif board.bingo():
                    print(board.score(called))
                    return
            boards = new_boards


if __name__ == "__main__":
    main()
