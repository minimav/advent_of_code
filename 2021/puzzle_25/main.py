def main():
    with open("data/input_25.txt") as f:
        state = {}
        for y, row in enumerate(f.readlines()):
            for x, value in enumerate(row.rstrip("\n")):
                if value != ".":
                    state[x, y] = value

    max_x = max(x for x, _ in state) + 1
    max_y = max(y for _, y in state) + 1

    step = 1
    while True:
        post_east_state = {}
        move = False
        for (x, y), value in state.items():
            if value != ">":
                post_east_state[x, y] = value
                continue
            new_x = (x + 1) % max_x
            if (new_x, y) not in state:
                move = True
                post_east_state[new_x, y] = value
            else:
                post_east_state[x, y] = value

        post_south_state = {}
        for (x, y), value in post_east_state.items():
            if value != "v":
                post_south_state[x, y] = value
                continue
            new_y = (y + 1) % max_y
            if (x, new_y) not in post_east_state:
                move = True
                post_south_state[x, new_y] = value
            else:
                post_south_state[x, y] = value

        state = post_south_state
        if not move:
            print(f"No move after {step=}")
            break

        step += 1


if __name__ == "__main__":

    main()
