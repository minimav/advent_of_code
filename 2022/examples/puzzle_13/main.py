from functools import cmp_to_key
from itertools import zip_longest

import time


def compare(left, right) -> bool:
    for left_part, right_part in zip_longest(left, right):
        if left_part is None:
            return -1
        elif right_part is None:
            return 1

        left_int = isinstance(left_part, int)
        right_int = isinstance(right_part, int)
        if left_int and right_int:
            if left_part < right_part:
                return -1
            elif left_part == right_part:
                continue
            else:
                return 1
        elif left_int:
            return compare([left_part], right_part)
        elif right_int:
            return compare(left_part, [right_part])
        else:
            cmp = compare(left_part, right_part)
            if cmp is 0:
                continue
            return cmp
    return 0


def compare_pairs(input_: str) -> int:
    answer: int = 0
    for index, raw_pair in enumerate(input_.split("\n\n"), start=1):
        left, right = map(eval, raw_pair.split("\n"))
        if compare(left, right) == -1:
            answer += index
    return answer


def part_2(input_) -> int:
    packets = [[[2]], [[6]]]
    for line in input_.split("\n"):
        if not line:
            continue
        packet = eval(line)
        packets.append(packet)

    sorted_packets = sorted(packets, key=cmp_to_key(compare))

    answer = 1
    for index, packet in enumerate(sorted_packets, start=1):
        if packet == [[2]] or packet == [[6]]:
            answer *= index
    return answer


if __name__ == "__main__":

    # example test cases
    assert compare([1, 1, 3, 1, 1], [1, 1, 5, 1, 1]) == -1
    assert compare([[1], [2, 3, 4]], [[1], 4]) == -1
    assert compare([9], [[8, 7, 6]]) == 1
    assert compare([[4, 4], 4, 4], [[4, 4], 4, 4, 4]) == -1
    assert compare([7, 7, 7, 7], [7, 7, 7]) == 1
    assert compare([], [3]) == -1
    assert compare([[[]]], [[]]) == 1
    assert (
        compare(
            [1, [2, [3, [4, [5, 6, 7]]]], 8, 9], [1, [2, [3, [4, [5, 6, 0]]]], 8, 9]
        )
        == 1
    )

    # additional test cases
    assert compare([[]], []) == 1
    assert compare([], [[]]) == -1
    assert compare([[]], [3]) == -1
    assert compare([3], [[]]) == 1
    assert compare([[2]], [[2, 6]]) == -1
    assert compare([[2], [7]], [[2, 6]]) == -1
    assert compare([[1], [2, 3, 4]], [[1], 2, 3, 4]) == 1

    example = """[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"""
    assert part_2(example) == 140

    start_time = time.time()
    with open("input.txt") as f:
        input_ = f.read()

    part_1_answer = compare_pairs(input_)
    part_2_answer = part_2(input_)

    print(f"Part 1 answer: {part_1_answer}")
    print(f"Part 2 answer: {part_2_answer}")
    print(f"Took {time.time() - start_time:.5f} seconds")
