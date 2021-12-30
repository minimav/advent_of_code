from dataclasses import dataclass
import itertools
import typing as tp


@dataclass(frozen=True)
class Volume:
    # inclusive?
    x: tp.Tuple[int, int]
    y: tp.Tuple[int, int]
    z: tp.Tuple[int, int]

    def volume(self) -> int:
        return (
            (self.x[-1] - self.x[0])
            * (self.y[-1] - self.y[0])
            * (self.z[-1] - self.z[0])
        )

    def intersection(
        self, other: "Volume"
    ) -> tp.Tuple[tp.List["Volume"], tp.Optional["Volume"], tp.List["Volume"]]:
        def intersect(dim):
            min_1, max_1 = getattr(self, dim)
            min_2, max_2 = getattr(other, dim)
            if min_1 >= max_2 or min_2 >= max_1:
                return None
            else:
                return (max(min_1, min_2), min(max_1, max_2))

        x_intersection = intersect("x")
        y_intersection = intersect("y")
        z_intersection = intersect("z")
        if any(i is None for i in (x_intersection, y_intersection, z_intersection)):
            return None

        intersection = Volume(x=x_intersection, y=y_intersection, z=z_intersection)
        self_diff = self.subtract(intersection)
        other_diff = other.subtract(intersection)

        # include the intersection twice to match adding the input volumes
        diff_volume = sum(v.volume() for v in self_diff) 
        diff_volume += sum(v.volume() for v in other_diff)
        diff_volume += 2 * intersection.volume()
        assert diff_volume == self.volume() + other.volume(), (self, other)

        return self_diff, intersection, other_diff

    def subtract(self, other: tp.Optional["Volume"]) -> tp.Optional[tp.List["Volume"]]:
        # assume here that other is a subset of self (usually an intersection)
        # need to check in 26 'directions', each dimension can be <, = or >
        def find_remainder(dim, cmp):
            min_1, max_1 = getattr(self, dim)
            min_2, max_2 = getattr(other, dim)
            if cmp == "<":
                if min_1 == min_2:
                    return None
                else:
                    return (min_1, min_2)
            elif cmp == "=":
                return (min_2, max_2)
            else:
                if max_1 == max_2:
                    return None
                else:
                    return (max_2, max_1)

        remainders = []
        for x_cmp, y_cmp, z_cmp in itertools.product(("<=>"), repeat=3):
            if x_cmp == y_cmp == z_cmp == "=":
                continue
            x_remainder = find_remainder("x", x_cmp)
            y_remainder = find_remainder("y", y_cmp)
            z_remainder = find_remainder("z", z_cmp)

            if any(i is None for i in (x_remainder, y_remainder, z_remainder)):
                continue

            remainder = Volume(x=x_remainder, y=y_remainder, z=z_remainder)
            remainders.append(remainder)

        return remainders


def test_subset_intersection_1():
    # 2 x 2 x 2 area, 8 cubes - intersect with one of those cubes
    v_1 = Volume((-1, 1), (-1, 1), (-1, 1))
    v_2 = Volume((0, 1), (0, 1), (0, 1))
    expected_v_1_remainder = [
        Volume((0, 1), (0, 1), (-1, 0)),
        Volume((0, 1), (-1, 0), (0, 1)),
        Volume((-1, 0), (0, 1), (0, 1)),
        Volume((-1, 0), (0, 1), (-1, 0)),
        Volume((0, 1), (-1, 0), (-1, 0)),
        Volume((-1, 0), (-1, 0), (0, 1)),
        Volume((-1, 0), (-1, 0), (-1, 0)),
    ]
    v_1_remainder, intersection, v_2_remainder = v_1.intersection(v_2)
    assert intersection == v_2
    assert set(v_1_remainder) == set(expected_v_1_remainder)
    assert not v_2_remainder


def test_subset_intersection_2():
    # 3 x 3 x 3 area, 27 cubes - intersect with the middle of those cubes
    v_1 = Volume((0, 3), (0, 3), (0, 3))
    v_2 = Volume((1, 2), (1, 2), (1, 2))
    expected_v_1_remainder = [
        Volume(x, y, z)
        for x, y, z in itertools.product(((0, 1), (1, 2), (2, 3)), repeat=3)
        if Volume(x, y, z) != v_2
    ]
    v_1_remainder, intersection, v_2_remainder = v_1.intersection(v_2)
    assert intersection == v_2
    assert set(v_1_remainder) == set(expected_v_1_remainder)
    assert not v_2_remainder


def test_subset_intersection_3():
    # 2 x 2 x 2 area, 8 cubes - intersect with half of them
    v_1 = Volume((-1, 1), (-1, 1), (-1, 1))
    v_2 = Volume((-1, 1), (-1, 1), (0, 1))
    expected_v_1_remainder = [
        Volume((-1, 1), (-1, 1), (-1, 0)),
    ]
    v_1_remainder, intersection, v_2_remainder = v_1.intersection(v_2)
    assert intersection == v_2
    assert set(v_1_remainder) == set(expected_v_1_remainder)
    assert not v_2_remainder


def test_subset_intersection_4():
    # 2 x 2 x 2 area, 8 cubes - intersect with 2 them
    v_1 = Volume((-1, 1), (-1, 1), (-1, 1))
    v_2 = Volume((-1, 1), (0, 1), (0, 1))
    expected_v_1_remainder = [
        Volume((-1, 1), (0, 1), (-1, 0)),
        Volume((-1, 1), (-1, 0), (0, 1)),
        Volume((-1, 1), (-1, 0), (-1, 0)),
    ]
    v_1_remainder, intersection, v_2_remainder = v_1.intersection(v_2)
    assert intersection == v_2
    assert set(v_1_remainder) == set(expected_v_1_remainder)
    assert not v_2_remainder


def test_general_intersection_1():
    # 2  2 x 1 x 1 areas, 1 cube overlap so two remainders
    v_1 = Volume((-1, 1), (0, 1), (0, 1))
    v_2 = Volume((0, 2), (0, 1), (0, 1))
    expected_v_1_remainder = [Volume((-1, 0), (0, 1), (0, 1))]
    expected_v_2_remainder = [Volume((1, 2), (0, 1), (0, 1))]
    expected_intersection = Volume((0, 1), (0, 1), (0, 1))
    v_1_remainder, intersection, v_2_remainder = v_1.intersection(v_2)
    assert intersection == expected_intersection
    assert set(v_1_remainder) == set(expected_v_1_remainder)
    assert set(v_2_remainder) == set(expected_v_2_remainder)


def test_volume():
    for v in range(10):
        assert Volume((0, v), (0, v), (0, v)).volume() == v ** 3


test_subset_intersection_1()
test_subset_intersection_2()
test_subset_intersection_3()
test_subset_intersection_4()
test_general_intersection_1()
test_volume()


def num_cubes_on(volumes):
    # assumes disjoint
    return sum(v.volume() for v in volumes)


def main():
    #path = "data/input_22.txt"
    path = "data/example_22_a.txt"

    instructions = []
    with open(path, "r") as f:
        for line in f.readlines():
            change, coords = line.rstrip("\n").split()
            x, y, z = [c[2:] for c in coords.split(",")]
            volume = Volume(
                x=tuple(map(int, x.split(".."))),
                y=tuple(map(int, y.split(".."))),
                z=tuple(map(int, z.split(".."))),
            )
            instructions.append((change, volume))

    currently_on = set()
    bounding_volume = Volume((-50, 50), (-50, 50), (-50, 50))
    for change, new_volume in instructions:

        if bounding_volume.intersection(new_volume) is None:
            print(f"Skipping {change} for {new_volume} as no intersection with {bounding_volume}")
            continue

        print(f"Applying {change} for {new_volume}, currently {len(currently_on)} volumes are on")
        if not currently_on and change == "on":
            currently_on = {new_volume}
            continue

        next_currently_on = set()
        if change == "on":
            next_currently_on.add(new_volume)
    
        for on_volume in currently_on:
            try:
                on_volume_diff, *_ = on_volume.intersection(new_volume)
            except TypeError:
                # no intersection case - remains on regardless of this change
                next_currently_on.add(on_volume)
                continue

            next_currently_on = next_currently_on.union(on_volume_diff)

        currently_on = next_currently_on
        print(num_cubes_on(currently_on))

    #print(currently_on)
    print(num_cubes_on(currently_on))


if __name__ == "__main__":
    main()
