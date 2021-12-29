from dataclasses import dataclass


@dataclass
class State:
    x: int
    y: int
    x_v: int
    y_v: int

    def step(self):
        self.x += self.x_v
        self.y += self.y_v
        self.y_v -= 1
        if self.x_v > 0:
            self.x_v -= 1
        elif self.x_v < 1:
            self.x_v += 1


def get_neighbours(x_v, y_v):
    return [
        (x_v + 1, y_v),
        (x_v + 1, y_v + 1),
        (x_v, y_v + 1),
    ]


def make_candidates(bounds):
    done = set()
    # 0 < x < bounds["x_max"]
    candidates = {(1, 0)}
    while True:
        done = done.union(candidates)
        next_candidates = set()
        for x_v, y_v in candidates:
            next_candidates = next_candidates.union(
                [
                    coord
                    for coord in get_neighbours(x_v, y_v)
                    if coord not in done and coord[0] <= bounds["x_max"]
                ]
            )
            yield x_v, y_v
        candidates = next_candidates


def main():
    with open("data/input_17.txt", "r") as f:
        x_condition, y_condition = (
            f.read().rstrip("\n").lstrip("target area: ").split(", ")
        )

    x_min, x_max = map(int, x_condition.lstrip("x=").split(".."))
    y_min, y_max = map(int, y_condition.lstrip("y=").split(".."))
    bounds = {"x_min": x_min, "x_max": x_max, "y_min": y_min, "y_max": y_max}

    def in_target(state: State):
        return (x_min <= state.x <= x_max) and (y_min <= state.y <= y_max)

    def should_terminate(state: State):
        return state.x > x_max or (state.y < y_min and state.y_v < 0)

    best_y = 0
    for i, (x_v, y_v) in enumerate(make_candidates(bounds)):
        best_y_this_run = 0
        ever_in_target = False
        state = State(0, 0, x_v, y_v)
        while not should_terminate(state):
            state.step()
            best_y_this_run = max(best_y_this_run, state.y)
            ever_in_target = ever_in_target or in_target(state)

        if best_y_this_run > best_y and ever_in_target:
            print(
                f"Iteration {i} new best: velocities {x_v=}, {y_v=}, {best_y_this_run=}"
            )
            best_y = best_y_this_run


if __name__ == "__main__":
    main()
