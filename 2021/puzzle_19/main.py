from dataclasses import dataclass
import itertools


@dataclass(frozen=True)
class Point:
    x: int
    y: int
    z: int

    def __repr__(self):
        return str((self.x, self.y, self.z))

    def __add__(self, other):
        return Point(self.x + other.x, self.y + other.y, self.z + other.z)

    def __sub__(self, other):
        return Point(self.x - other.x, self.y - other.y, self.z - other.z)

    def __mul__(self, other):
        return Point(self.x * other.x, self.y * other.y, self.z * other.z)

    def apply_transformation(self, transformation: "Transform") -> "Point":
        permuted = Point(
            x=getattr(self, transformation.permutation["x"]),
            y=getattr(self, transformation.permutation["y"]),
            z=getattr(self, transformation.permutation["z"]),
        )
        return permuted * transformation.multiplier


ORIGIN = Point(0, 0, 0)


def manhattan_distance(p_1: Point, p_2: Point) -> int:
    return abs(p_1.x - p_2.x) + abs(p_1.y - p_2.y) + abs(p_1.z - p_2.z)


@dataclass
class Transformation:
    permutation: str
    multiplier: Point

    def __post_init__(self):
        self.permutation = dict(zip("xyz", self.permutation))

    def __mul__(self, other):
        # apply self first, then other
        permutation = "".join([other.permutation[self.permutation[c]] for c in "xyz"])
        return Transformation(permutation, self.multiplier * other.multiplier)

    def inverse(self):
        inverse_permutation = {v: k for k, v in self.permutation.items()}
        return Transformation(
            "".join([inverse_permutation[c] for c in "xyz"]), self.multiplier
        )

    @staticmethod
    def transformations():
        even_permutations = ["xyz", "yzx", "zxy"]
        even_multipliers = [
            Point(1, 1, 1),
            Point(1, -1, -1),
            Point(-1, 1, -1),
            Point(-1, -1, 1),
        ]
        yield from [
            Transformation(*args)
            for args in itertools.product(even_permutations, even_multipliers)
        ]

        odd_permutations = ["xzy", "yxz", "zyx"]
        odd_multipliers = [
            Point(-1, 1, 1),
            Point(1, -1, 1),
            Point(1, 1, -1),
            Point(-1, -1, -1),
        ]
        yield from [
            Transformation(*args)
            for args in itertools.product(odd_permutations, odd_multipliers)
        ]


assert len(list(Transformation.transformations())) == 24
assert Point(1, 2, 3).apply_transformation(
    Transformation("xyz", Point(1, 1, 1))
) == Point(1, 2, 3)
assert Point(1, 1, 1).apply_transformation(
    Transformation("xyz", Point(1, -1, 1))
) == Point(1, -1, 1)
assert Point(1, 2, 3).apply_transformation(
    Transformation("xzy", Point(1, -1, 1))
) == Point(1, -3, 2)
assert Point(1, 2, 3).apply_transformation(
    Transformation("yzx", Point(-1, -1, 1))
) == Point(-2, -3, 1)


def compare_scanners(scanner_1_coords, scanner_2_coords) -> dict:
    correct_shift = None
    correct_transformation = None
    found_transformation = False
    for transformation in Transformation.transformations():
        # get scanner 2 (potentially) into scanner 1 space
        transformed_2_coords = [
            p.apply_transformation(transformation) for p in scanner_2_coords
        ]
        for point in scanner_1_coords:
            for other_point in transformed_2_coords:
                # other + shift = point
                shift = point - other_point
                shifted_2_coords = [p + shift for p in transformed_2_coords]
                in_common = scanner_1_coords.intersection(shifted_2_coords)
                if len(in_common) >= 12:
                    print(
                        f"Found {len(in_common)} beacons in common using shift {shift}"
                    )
                    # print(in_common)
                    correct_transformation = transformation
                    correct_shift = shift
                    found_transformation = True
                    break

            if found_transformation:
                break

        if found_transformation:
            break

    return {"transformation": correct_transformation, "shift": correct_shift}


def main():
    path = "data/input_19.txt"
    # path = "data/example_19_1.txt"
    with open(path, "r") as f:
        scanners = {}
        scanner_coords = []
        scanner_id = -1
        for line in f.readlines():
            line = line.rstrip("\n")
            if not line:
                continue
            elif line.startswith("--- scanner "):
                if scanner_coords:
                    scanners[scanner_id] = set(scanner_coords)
                scanner_coords = []
                scanner_id += 1
            else:
                scanner_coords.append(Point(*map(int, line.split(","))))

        # final scanner
        scanners[scanner_id] = set(scanner_coords)

    # store transformations back to 0
    resolved_scanners = {0}
    transformations = {}
    compared_scanners = set()
    while resolved_scanners != set(scanners):
        for scanner_index in resolved_scanners:
            for other_index in set(scanners).difference(resolved_scanners):
                key = (min(scanner_index, other_index), max(scanner_index, other_index))
                if key in compared_scanners:
                    continue

                # don't look at this pair of scanners again after this time
                compared_scanners.add(key)
                output = compare_scanners(
                    scanners[scanner_index], scanners[other_index]
                )
                if output["transformation"] is not None:
                    print(
                        f"Success comparing scanners {scanner_index} and {other_index}"
                    )

                    resolved_scanners.add(other_index)
                    transformations[other_index, scanner_index] = output
                    break

            if other_index in resolved_scanners:
                break

    def path_to_0_coords(scanner_index):
        path = []
        current = scanner_index
        while current != 0:
            for from_index, to_index in transformations:
                if current == from_index:
                    path.append((current, to_index))
                    current = to_index
                    break
        return path

    beacons = set()
    scanner_locations = {0: ORIGIN}
    for scanner_index, relative_beacons in scanners.items():
        if scanner_index == 0:
            beacons = beacons.union(relative_beacons)
            continue

        # work back to scanner 0 coordinate system through transformations
        location = ORIGIN
        for key in path_to_0_coords(scanner_index):
            t = transformations[key]
            location = location.apply_transformation(t["transformation"]) + t["shift"]
            relative_beacons = {
                p.apply_transformation(t["transformation"]) + t["shift"]
                for p in relative_beacons
            }
        beacons = beacons.union(relative_beacons)
        scanner_locations[scanner_index] = location

    print(f"Part 1 solution: {len(beacons)}")

    manhattan_distances = []
    for loc_1, loc_2 in itertools.combinations(scanner_locations.values(), 2):
        manhattan_distances.append(manhattan_distance(loc_1, loc_2))
    print(f"Part 2 solution: {max(manhattan_distances)}")


if __name__ == "__main__":
    main()
