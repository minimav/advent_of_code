from dataclasses import dataclass
from typing import Optional

from sympy.solvers import solve_poly_system
from sympy import Symbol


@dataclass
class Position:
    x: int
    y: int
    z: int

    def __str__(self):
        return f"({self.x},{self.y},{self.z})"


@dataclass
class Velocity:
    x: int
    y: int
    z: int

    def __str__(self):
        return f"({self.x},{self.y},{self.z})"


@dataclass
class Hailstone:
    position: Position
    velocity: Velocity

    def __str__(self):
        return f"{self.position} @ {self.velocity}"

    def intersect_2d(self, other: "Hailstone") -> Optional[tuple[int, int]]:
        # Compare the x and y coordinates of the two hailstones:
        # other.position + mu * other.velocity
        # self.position + psi * self.velocity
        if self.velocity.x == other.velocity.x and self.velocity.y == other.velocity.y:
            return None
        if self.velocity.x * other.velocity.y - other.velocity.x * self.velocity.y == 0:
            return None

        mu_numerator = self.velocity.x * (
            self.position.y - other.position.y
        ) + self.velocity.y * (other.position.x - self.position.x)
        denominator = (
            self.velocity.x * other.velocity.y - other.velocity.x * self.velocity.y
        )
        mu = mu_numerator / denominator

        psi_numerator = other.velocity.x * (
            other.position.y - self.position.y
        ) + other.velocity.y * (self.position.x - other.position.x)
        psi = psi_numerator / -denominator
        if mu < 0 or psi < 0:
            return None
        x = other.position.x + mu * other.velocity.x
        y = other.position.y + mu * other.velocity.y
        return x, y


def parse_input(input: str):
    lines = input.split("\n")
    hailstones = []
    for line in lines:
        position_raw, velocity_raw = line.split(" @ ")
        position = Position(*map(int, position_raw.split(", ")))
        velocity = Velocity(*map(int, velocity_raw.split(", ")))
        hailstones.append(Hailstone(position, velocity))
    return hailstones


def part_1(
    input: str,
    low: int,
    high: int,
):
    hailstones = parse_input(input)

    intersections = 0
    for i, hail in enumerate(hailstones):
        for j, other in enumerate(hailstones):
            if i >= j:
                continue

            intersection = hail.intersect_2d(other)
            if intersection is not None:
                x, y = intersection
                if low <= x <= high and low <= y <= high:
                    intersections += 1
    print(intersections)


def part_2(input: str):
    hailstones = sorted(parse_input(input), key=lambda h: h.velocity.x)

    x = Symbol("x")
    y = Symbol("y")
    z = Symbol("z")
    v_x = Symbol("v_x")
    v_y = Symbol("v_y")
    v_z = Symbol("v_z")
    times = [Symbol(f"t_{i}") for i in range(3)]
    equations = []
    for i, hailstone in enumerate(hailstones[:3]):
        equations.append(
            x + times[i] * v_x - hailstone.position.x - times[i] * hailstone.velocity.x
        )
        equations.append(
            y + times[i] * v_y - hailstone.position.y - times[i] * hailstone.velocity.y
        )
        equations.append(
            z + times[i] * v_z - hailstone.position.z - times[i] * hailstone.velocity.z
        )

    solution, *_ = solve_poly_system(equations, x, y, z, v_x, v_y, v_z, *times)
    x, y, z, *_ = solution
    print(x + y + z)


if __name__ == "__main__":
    with open("puzzle_24/example.txt", "r") as f:
        example = f.read()

    with open("puzzle_24/input.txt", "r") as f:
        input = f.read()

    part_1(example, low=7, high=27)
    part_1(input, low=200000000000000, high=400000000000000)
    part_2(example)
    part_2(input)
