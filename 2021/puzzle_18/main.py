from dataclasses import dataclass, field
import typing as tp


@dataclass
class Pair:
    left: tp.Union[int, "Pair"]
    right: tp.Union[int, "Pair"]
    depth: int = 0
    terminal: bool = field(init=False)
    parent: tp.Optional["Pair"] = field(default=None, compare=False, repr=False)
    left_child: tp.Optional[bool] = field(default=None, compare=False)

    def left_int(self):
        return isinstance(self.left, int)

    def right_int(self):
        return isinstance(self.right, int)

    def __post_init__(self):
        self.terminal = self.left_int() and self.right_int()
        if not self.left_int():
            self.left.left_child = True
        if not self.right_int():
            self.right.left_child = False

    def __iter__(self):
        yield self
        if not self.left_int():
            yield from self.left
        if not self.right_int():
            yield from self.right

    def left_terminal_traverse(self):
        """Traverse over terminal pairs from left to right."""
        if self.left_int():
            pass
        elif self.left.terminal:
            yield self.left
        else:
            yield from self.left.left_terminal_traverse()
        if self.right_int():
            pass
        elif self.right.terminal:
            yield self.right
        else:
            yield from self.right.left_terminal_traverse()

    def left_value_traverse(self):
        """Traverse over values left to right with their containing pair."""
        if self.left_int():
            yield (self, "left")
        elif self.left.terminal:
            yield from [(self.left, "left"), (self.left, "right")]
        else:
            yield from self.left.left_value_traverse()
        if self.right_int():
            yield (self, "right")
        elif self.right.terminal:
            yield from [(self.right, "left"), (self.right, "right")]
        else:
            yield from self.right.left_value_traverse()

    def magnitude(self) -> int:
        value = 0
        if self.left_int():
            value += 3 * self.left
        else:
            value += 3 * self.left.magnitude()
        if self.right_int():
            value += 2 * self.right
        else:
            value += 2 * self.right.magnitude()
        return value

    def __add__(self, other):
        new_depth = self.depth

        # increase depth of all children, including themselves
        for pair in self:
            pair.depth += 1
        for pair in other:
            pair.depth += 1

        new_pair = Pair(self, other, depth=new_depth)
        new_pair.left.parent = new_pair
        new_pair.right.parent = new_pair
        return new_pair

    @classmethod
    def parse(cls, input, depth=0):
        if isinstance(input[0], int):
            left = input[0]
        else:
            left = cls.parse(input[0], depth=depth + 1)
        if isinstance(input[-1], int):
            right = input[-1]
        else:
            right = cls.parse(input[-1], depth=depth + 1)
        new_pair = cls(left, right, depth)
        if isinstance(left, Pair):
            new_pair.left.parent = new_pair
        if isinstance(right, Pair):
            new_pair.right.parent = new_pair
        return new_pair

    def to_list(self):
        if self.left_int():
            left = self.left
        else:
            left = self.left.to_list()
        if self.right_int():
            right = self.right
        else:
            right = self.right.to_list()
        return [left, right]

    def __iadd__(self, other):
        return self + other

    def split(self, attr="left"):
        value = getattr(self, attr)
        if not isinstance(value, int):
            raise ValueError("Cannot split a Pair, only ints can be split")

        left = value / 2 if value % 2 == 0 else (value - 1) / 2
        right = value / 2 if value % 2 == 0 else (value + 1) / 2
        self.terminal = False
        left_child = attr == "left"
        split_pair = Pair(
            int(left),
            int(right),
            depth=self.depth + 1,
            parent=self,
            left_child=left_child,
        )
        setattr(self, attr, split_pair)

    def explode(self):
        if not self.terminal:
            raise ValueError("Cannot explode non-terminal pair")
        left = self.left
        right = self.right

        # find left if exists and add
        # first find first parent where this is not the left child
        current = self
        found_left = False
        while True:
            if current.depth == 0:
                break
            right_child = not current.left_child
            current = current.parent
            if right_child:
                found_left = True
                break
        # print(f"Left parent is {current.to_list()}")

        # if at the root we can skip as no left value exists
        # otherwise, shift to left child of parent, then traverse right
        if found_left:
            if current.left_int():
                current.left += left
            else:
                current = current.left
                while True:
                    if current.right_int():
                        current.right += left
                        break

                    current = current.right
        else:
            # print("Left explode reached root")
            pass

        # find right if exists and add (same logic as above but reversed)
        current = self
        found_right = False
        while True:
            if current.depth == 0:
                break
            left_child = current.left_child
            current = current.parent
            if left_child:
                found_right = True
                break
        # print(f"Right parent is {current.to_list()}")

        if found_right:
            if current.right_int():
                current.right += right
            else:
                current = current.right
                while True:
                    if current.left_int():
                        current.left += right
                        break

                    current = current.left
        else:
            # print("Right explode reached root")
            pass

        # remove self from parent and mark new terminal pairs
        if self.left_child:
            self.parent.left = 0
            if self.parent.right_int():
                self.parent.terminal = True
        else:
            self.parent.right = 0
            if self.parent.left_int():
                self.parent.terminal = True

    def reduce(self, max_iterations=None):
        """Apply reductions until there are none to make."""
        iteration = 0
        while True:
            iteration += 1
            made_reduction = False
            if max_iterations is not None and max_iterations < iteration:
                raise RuntimeError

            # look for explosions
            for pair in self.left_terminal_traverse():
                if pair.depth >= 4:
                    pair.explode()
                    made_reduction = True
                    break

            if made_reduction:
                continue

            # look for splits
            for pair, attr in self.left_value_traverse():
                value = getattr(pair, attr)
                if value >= 10:
                    pair.split(attr=attr)
                    made_reduction = True
                    break

            if not made_reduction:
                break


# check parsing and reverse parsing
pair = Pair.parse([[[[[9, 8], 1], 2], 3], 4])
assert pair == Pair(
    Pair(Pair(Pair(Pair(9, 8, depth=4), 1, depth=3), 2, depth=2), 3, depth=1),
    4,
    depth=0,
)
assert pair.to_list() == [[[[[9, 8], 1], 2], 3], 4]

# traversal
assert list(pair.left_terminal_traverse()) == [Pair(9, 8, depth=4)]
assert list(pair.left_value_traverse()) == [
    (Pair(9, 8, depth=4), "left"),
    (Pair(9, 8, depth=4), "right"),
    (Pair(Pair(9, 8, depth=4), 1, depth=3), "right"),
    (Pair(Pair(Pair(9, 8, depth=4), 1, depth=3), 2, depth=2), "right"),
    (
        Pair(Pair(Pair(Pair(9, 8, depth=4), 1, depth=3), 2, depth=2), 3, depth=1),
        "right",
    ),
    (
        Pair(
            Pair(Pair(Pair(Pair(9, 8, depth=4), 1, depth=3), 2, depth=2), 3, depth=1),
            4,
            depth=0,
        ),
        "right",
    ),
]

# check addition
assert Pair(1, 1) + Pair(2, 2) == Pair(Pair(1, 1, 1), Pair(2, 2, 1), 0)

# check magnitude calculation
assert Pair(Pair(1, 2), Pair(Pair(3, 4), 5)).magnitude() == 143
assert (
    Pair(Pair(Pair(Pair(1, 1), Pair(2, 2)), Pair(3, 3)), Pair(4, 4)).magnitude() == 445
)
assert (
    Pair(Pair(Pair(Pair(3, 0), Pair(5, 3)), Pair(4, 4)), Pair(5, 5)).magnitude() == 791
)
assert (
    Pair.parse(
        [[[[6, 6], [7, 6]], [[7, 7], [7, 0]]], [[[7, 7], [7, 7]], [[7, 8], [9, 9]]]]
    ).magnitude()
    == 4140
)

# check split
pair = Pair(10, Pair(1, 2, 1))
pair.split("left")
assert pair == Pair(Pair(5, 5, 1), Pair(1, 2, 1), 0)

# check reduction
pair = Pair.parse([[[[[9, 8], 1], 2], 3], 4])
pair.reduce()
assert pair == Pair.parse([[[[0, 9], 2], 3], 4])

# check addition + reduce examples
examples = [
    {
        "inputs": [[[[[4, 3], 4], 4], [7, [[8, 4], 9]]], [1, 1]],
        "expected": [[[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]],
    },
    {
        "inputs": [[1, 1], [2, 2], [3, 3], [4, 4]],
        "expected": [[[[1, 1], [2, 2]], [3, 3]], [4, 4]],
    },
    {
        "inputs": [[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]],
        "expected": [[[[3, 0], [5, 3]], [4, 4]], [5, 5]],
    },
    {
        "inputs": [[1, 1], [2, 2], [3, 3], [4, 4], [5, 5], [6, 6]],
        "expected": [[[[5, 0], [7, 4]], [5, 5]], [6, 6]],
    },
    {
        "inputs": [
            [[[0, [4, 5]], [0, 0]], [[[4, 5], [2, 6]], [9, 5]]],
            [7, [[[3, 7], [4, 3]], [[6, 3], [8, 8]]]],
            [[2, [[0, 8], [3, 4]]], [[[6, 7], 1], [7, [1, 6]]]],
            [[[[2, 4], 7], [6, [0, 5]]], [[[6, 8], [2, 8]], [[2, 1], [4, 5]]]],
            [7, [5, [[3, 8], [1, 4]]]],
            [[2, [2, 2]], [8, [8, 1]]],
            [2, 9],
            [1, [[[9, 3], 9], [[9, 0], [0, 7]]]],
            [[[5, [7, 4]], 7], 1],
            [[[[4, 2], 2], 6], [8, 7]],
        ],
        "expected": [[[[8, 7], [7, 7]], [[8, 6], [7, 7]]], [[[0, 7], [6, 6]], [8, 7]]],
    },
    {
        "inputs": [
            [[[0, [5, 8]], [[1, 7], [9, 6]]], [[4, [1, 2]], [[1, 4], 2]]],
            [[[5, [2, 8]], 4], [5, [[9, 9], 0]]],
            [6, [[[6, 2], [5, 6]], [[7, 6], [4, 7]]]],
            [[[6, [0, 7]], [0, 9]], [4, [9, [9, 0]]]],
            [[[7, [6, 4]], [3, [1, 3]]], [[[5, 5], 1], 9]],
            [[6, [[7, 3], [3, 2]]], [[[3, 8], [5, 7]], 4]],
            [[[[5, 4], [7, 7]], 8], [[8, 3], 8]],
            [[9, 3], [[9, 9], [6, [4, 9]]]],
            [[2, [[7, 7], 7]], [[5, 8], [[9, 3], [0, 2]]]],
            [[[[5, 2], 5], [8, [3, 7]]], [[5, [7, 5]], [4, 4]]],
        ],
        "expected": [
            [[[6, 6], [7, 6]], [[7, 7], [7, 0]]],
            [[[7, 7], [7, 7]], [[7, 8], [9, 9]]],
        ],
    },
]

for example in examples:
    current = None
    for raw_pair in example["inputs"]:
        pair = Pair.parse(raw_pair)
        if current is None:
            current = pair
        else:
            # print(f"Adding {current.to_list()} and {pair.to_list()}")
            current += pair
        # print(f"Pre reduce {current.to_list()}")
        current.reduce(max_iterations=None)
    assert current == Pair.parse(example["expected"])


def part_1(pairs):
    pair = Pair.parse(pairs[0])
    for next_pair in pairs[1:]:
        pair += Pair.parse(next_pair)
        pair.reduce()
    return pair.magnitude()


def part_2(pairs):
    max_magnitude = 0
    for pair_1 in pairs:
        for pair_2 in pairs:
            added_pair = Pair.parse(pair_1) + Pair.parse(pair_2)
            added_pair.reduce()
            magnitude = added_pair.magnitude()
            if magnitude > max_magnitude:
                max_magnitude = magnitude
    return max_magnitude


def main():
    with open("data/input_18.txt", "r") as f:
        pairs = [eval(line.rstrip("\n")) for line in f.readlines()]

    print(f"Part 1 solution: {part_1(pairs)}")
    print(f"Part 2 solution: {part_2(pairs)}")


if __name__ == "__main__":
    main()
