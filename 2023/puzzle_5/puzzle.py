def parse_int_row(row: str) -> list[int]:
    try:
        index = row.index(":") + 2
    except ValueError:
        index = 0
    return [int(s) for s in row[index:].split()]


with open("puzzle_5/input.txt") as f:
    raw_data = [l.rstrip("\n") for l in f.readlines() if l.rstrip("\n")]

    initial_seeds = parse_int_row(raw_data[0])
    raw_mappings = {}
    order = {}
    new_mapping_key = None
    for row in raw_data[1:]:
        if "map" in row:
            new_mapping_key = tuple(row[:-5].split("-to-"))
            start, end = new_mapping_key
            order[start] = end
            raw_mappings[new_mapping_key] = []
        else:
            raw_mappings[new_mapping_key].append(parse_int_row(row))

    mappings = {k: sorted(v, key=lambda x: x[1]) for k, v in raw_mappings.items()}

    def traverse(current_num, current_state):
        if current_state not in order:
            return current_num
        next_state = order[current_state]
        intervals = mappings[(current_state, next_state)]
        for start_next, start_current, length in intervals:
            if start_current <= current_num < start_current + length:
                next_num = start_next + current_num - start_current
                return traverse(next_num, next_state)
        return traverse(current_num, next_state)

    part_1_outputs = [traverse(s, "seed") for s in initial_seeds]
    print(min(part_1_outputs))

    # All interval tuples are inclusive i.e. (2, 4) means 2, 3, 4 with length 3!

    def smart_traverse(current_range, current_state):
        if current_state not in order:
            # Return minimum of the range
            return current_range[0]

        next_state = order[current_state]
        intervals = mappings[(current_state, next_state)]
        overlaps = []

        for index, (start_next, start_current, length) in enumerate(intervals):
            if (start_current <= current_range[0]) and (
                current_range[1] < start_current + length
            ):
                # Wholly within this range, can't map to any other
                overlaps.append(
                    (
                        start_next + current_range[0] - start_current,
                        start_next + current_range[1] - start_current,
                    )
                )
                break
            elif (start_current <= current_range[0] < start_current + length) and (
                start_current + length <= current_range[1]
            ):
                # Start within, end after
                overlap = (
                    start_next + current_range[0] - start_current,
                    start_next + length - 1,
                )
                overlaps.append(overlap)
                current_range = (
                    start_current + length,
                    current_range[1],
                )
            elif (current_range[0] < start_current) and (
                start_current <= current_range[1] < start_current + length
            ):
                # Start before, end within
                overlaps.append(
                    (
                        start_next,
                        start_next + current_range[1] - start_current,
                    ),
                )
                # Before part that can't overlap any remaining, maps to itself
                overlaps.append(
                    (
                        current_range[0],
                        start_current - 1,
                    )
                )
                # Can't overlap anything else as end within this interval
                break
            elif (current_range[0] < start_current) and (
                start_current + length <= current_range[1]
            ):
                # Start and end outside, so wholly cover, need the before part,
                # overlap which maps and the remainder to check against
                # remaining intervals
                overlaps.append(
                    (
                        current_range[0],
                        start_current - 1,
                    )
                )
                overlaps.append((start_next, start_next + length - 1))
                current_range = (
                    start_current + length,
                    current_range[1],
                )
            elif index == len(intervals) - 1:
                # Last interval with no overlap, so either wholly before or
                # after and we need to keep the remainder as that maps directly
                assert (start_current + length <= current_range[0]) or (
                    current_range[1] < start_current
                )
                overlaps.append(current_range)
            else:
                # Not the last interval but wholly before or wholly after
                assert (start_current + length <= current_range[0]) or (
                    current_range[1] < start_current
                )

        return min(smart_traverse(overlap, next_state) for overlap in overlaps)

    initial_seed_ranges = []
    for i in range(0, len(initial_seeds), 2):
        initial_seed_ranges.append(
            (initial_seeds[i], initial_seeds[i] + initial_seeds[i + 1] - 1)
        )

    part_2_outputs = [
        smart_traverse(seed_range, "seed") for seed_range in initial_seed_ranges
    ]
    print(min(part_2_outputs))
