import functools
import heapq
import itertools

# +---+---+---+
# | 7 | 8 | 9 |
# +---+---+---+
# | 4 | 5 | 6 |
# +---+---+---+
# | 1 | 2 | 3 |
# +---+---+---+
#     | 0 | A |
#     +---+---+
numeric_keypad = {
    "9": {
        "8": "<",
        "6": "v",
    },
    "8": {
        "9": ">",
        "7": "<",
        "5": "v",
    },
    "7": {
        "8": ">",
        "4": "v",
    },
    "6": {
        "9": "^",
        "5": "<",
        "3": "v",
    },
    "5": {
        "8": "^",
        "6": ">",
        "4": "<",
        "2": "v",
    },
    "4": {
        "7": "^",
        "5": ">",
        "1": "v",
    },
    "3": {
        "6": "^",
        "2": "<",
        "A": "v",
    },
    "2": {
        "5": "^",
        "3": ">",
        "1": "<",
        "0": "v",
    },
    "1": {
        "4": "^",
        "2": ">",
    },
    "0": {
        "2": "^",
        "A": ">",
    },
    "A": {
        "3": "^",
        "0": "<",
    },
}

#     +---+---+
#     | ^ | A |
# +---+---+---+
# | < | v | > |
# +---+---+---+
arrows_keypad = {
    "<": {
        "v": ">",
    },
    "v": {
        ">": ">",
        "^": "^",
        "<": "<",
    },
    ">": {
        "A": "^",
        "v": "<",
    },
    "A": {
        "^": "<",
        ">": "v",
    },
    "^": {
        "A": ">",
        "v": "v",
    },
}


def dijkstra_shortest_paths(
    moves: dict[str, dict[str, str]],
    start: str,
    end: str,
) -> int:
    current_best = {}
    # Include has cheated status to force the no-cheat path to be explored first
    queue = [(0, start, "")]
    routes = []
    while queue:
        cost, char, route = heapq.heappop(queue)
        if char == end:
            routes.append(route)
            continue

        for new_char, arm_move in moves[char].items():
            new_cost = cost + 1
            new_route = route + arm_move
            if new_char in current_best and current_best[new_char] < new_cost:
                continue

            current_best[new_char] = new_cost
            heapq.heappush(queue, (new_cost, new_char, new_route))

    return routes


def hop_shortest_routes(
    all_shortest_routes: dict[str, list[str]], target: str
) -> list[str]:
    current = "A"
    routes = [""]
    for char in target:
        shortest_routes = all_shortest_routes[current, char]

        new_routes = []
        for route in routes:
            for shortest_route in shortest_routes:
                # Include press of digit
                new_routes.append(route + shortest_route + "A")
        routes = new_routes
        current = char
    return routes


numeric_keypad_shortest_routes = {}
keypad_chars = "0123456789A"
for start in keypad_chars:
    for end in keypad_chars:
        numeric_keypad_shortest_routes[start, end] = dijkstra_shortest_paths(
            numeric_keypad, start, end
        )

arrows_keypad_shortest_routes = {}
arrow_chars = "^<>vA"
for start in arrow_chars:
    for end in arrow_chars:
        arrows_keypad_shortest_routes[start, end] = dijkstra_shortest_paths(
            arrows_keypad, start, end
        )


@functools.cache
def memoise(start: str, end: str, num_left: int) -> int:
    if num_left == 1:
        # Final step is human press plus A to enter
        return len(arrows_keypad_shortest_routes[start, end][0]) + 1
    return min(
        sum(
            memoise(e_start, e_end, num_left - 1)
            for e_start, e_end in itertools.pairwise("A" + expansion + "A")
            # Need to start and end at A to reflect press from previous and next
        )
        for expansion in arrows_keypad_shortest_routes[start, end]
    )


def solve(target: str, num_layers: int) -> int:
    # Need to start at A on each keypad
    input = "A" + target
    return sum(memoise(*pair, num_layers) for pair in itertools.pairwise(input))


def press_buttons(input: str, num_layers: int):
    lines = input.split("\n")

    answer = 0
    for line in lines:
        print(f"########### Solving {line}")
        intensity = int(line[:-1])

        # Solve numeric keypad problem first
        routes = hop_shortest_routes(numeric_keypad_shortest_routes, line)

        presses = min(solve(v, num_layers) for v in routes)
        print(presses, intensity)
        answer += presses * intensity

    print(answer)


if __name__ == "__main__":
    with open("puzzle_21/example.txt", "r") as f:
        example = f.read()
        press_buttons(example, 2)
        press_buttons(example, 25)

    with open("puzzle_21/input.txt", "r") as f:
        input = f.read()
        press_buttons(input, 2)
        press_buttons(input, 25)
