from dataclasses import dataclass
from itertools import pairwise


@dataclass
class Chunk:
    values_with_repeats: list[tuple[int, int]]
    index: int

    @property
    def size(self) -> int:
        return sum(r for _, r in self.values_with_repeats)

    @property
    def next_start(self) -> int:
        return self.index + self.size


def densify(disk_map: list[Chunk]) -> list[Chunk]:
    new_map = []
    current_chunk = disk_map[0]
    for chunk in disk_map[1:]:
        if chunk.index == current_chunk.next_start:
            current_chunk.values_with_repeats += chunk.values_with_repeats
        else:
            new_map.append(current_chunk)
            current_chunk = chunk
    new_map.append(current_chunk)
    return new_map


def gaps(disk_map: list[Chunk], prior_to: int):
    for i, (chunk_1, chunk_2) in enumerate(pairwise(disk_map)):
        if chunk_1.index >= prior_to:
            break
        if chunk_2.index == chunk_1.next_start:
            pass
        else:
            yield i, chunk_2.index - chunk_1.next_start


def parse_disk_map(line) -> list[Chunk]:
    chunks = []
    disk_index = 0
    for char_index, char in enumerate(line.rstrip("\n")):
        repeats = int(char)
        free_space = char_index % 2 != 0
        if free_space:
            disk_index += repeats
            continue

        value = int(char_index / 2)
        chunk = Chunk(
            values_with_repeats=[(value, repeats)],
            index=disk_index,
        )
        chunks.append(chunk)
        disk_index = chunk.next_start
    return chunks


def print_disk_map(disk_map: list[Chunk]):
    output = ""
    for chunk in disk_map:
        diff = chunk.index - len(output)
        if diff > 0:
            output += "." * diff
        for value, repeats in chunk.values_with_repeats:
            output += str(value) * repeats
    print(output)


def check_sum(disk_map):
    output = 0
    for chunk in disk_map:
        index = chunk.index
        for value, repeats in chunk.values_with_repeats:
            for i in range(index, index + repeats):
                output += i * value
            index += repeats
    return output


def part_1(line: str):
    disk_map = parse_disk_map(line)
    disk_map = densify(disk_map)

    while len(disk_map) > 1:
        # print_disk_map(disk_map)
        last_folder = disk_map[-1]
        gap_size = disk_map[1].index - disk_map[0].next_start

        last_value, last_repeats = last_folder.values_with_repeats[-1]
        disk_map[0].values_with_repeats.append(
            (last_value, min(last_repeats, gap_size))
        )
        if last_repeats <= gap_size:
            if len(last_folder.values_with_repeats) == 1:
                disk_map = disk_map[:-1]
            else:
                disk_map[-1].values_with_repeats = disk_map[-1].values_with_repeats[:-1]
        else:
            disk_map[-1].values_with_repeats[-1] = (last_value, last_repeats - gap_size)

        disk_map = densify(disk_map)

    print(check_sum(disk_map))


def part_2(line: str):
    disk_map = parse_disk_map(line)
    value_to_chunk = {
        chunk.values_with_repeats[0][0]: chunk for chunk in reversed(disk_map)
    }

    for value, chunk in value_to_chunk.items():
        # print("Doing ", value)
        # print_disk_map(disk_map)
        last_repeats = chunk.values_with_repeats[0][1]
        possible_gaps = list(gaps(disk_map, prior_to=chunk.index))
        for gap_index, gap_size in possible_gaps:
            if gap_size >= last_repeats:

                disk_map = (
                    disk_map[: gap_index + 1]
                    + [
                        Chunk(
                            values_with_repeats=chunk.values_with_repeats,
                            index=disk_map[gap_index].next_start,
                        )
                    ]
                    + disk_map[gap_index + 1 :]
                )

                # remove this chunk
                for i, chunk in enumerate(reversed(disk_map), start=1):
                    if chunk.values_with_repeats[0][0] == value:
                        if i == 1:
                            disk_map = disk_map[:-1]
                        else:
                            disk_map = disk_map[:-i] + disk_map[-i + 1 :]
                        break
                # print_disk_map(disk_map)
                break

    print(check_sum(disk_map))


def disk_map_from_string(line: str) -> list[Chunk]:
    disk_map = []
    for i, value in enumerate(line):
        if value == ".":
            continue
        disk_map.append(Chunk(values_with_repeats=[(int(value), 1)], index=i))
    return disk_map


if __name__ == "__main__":
    lines = [
        "0099811188827773336446555566",
        "00992111777.44.333....5555.6666.....8888..",
    ]
    for line in lines:
        print(check_sum(disk_map_from_string(line)))

    with open("puzzle_9/example.txt") as f:
        example = f.read()

    part_1(example)
    part_2(example)

    with open("puzzle_9/input.txt") as f:
        input_ = f.read()

    part_1(input_)
    part_2(input_)
