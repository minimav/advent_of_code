from collections import defaultdict


def get_neighbours(x, y):
    yield (x - 1, y - 1)
    yield (x, y - 1)
    yield (x + 1, y - 1)
    yield (x - 1, y)
    yield (x, y)
    yield (x + 1, y)
    yield (x - 1, y + 1)
    yield (x, y + 1)
    yield (x + 1, y + 1)


mapping = {".": "0", "#": "1"}


def coords_to_iterate_over(image):
    # https://stackoverflow.com/questions/8762819/runtimeerror-dictionary-changed-size-during-iteration-during-iteration-with-i
    coords = list(image.keys())
    # remember 1-border around current image, since neighbourhoods can contain #s
    x_coords = [x for x, _ in coords]
    y_coords = [y for _, y in coords]
    x_min, x_max = min(x_coords), max(x_coords)
    y_min, y_max = min(y_coords), max(y_coords)
    # top edge
    coords += [(x, y_min - 1) for x in range(x_min - 1, x_max + 2)]
    # right edge
    coords += [(x_max + 1, y) for y in range(y_min - 1, y_max + 2)]
    # bottom edge
    coords += [(x, y_max + 1) for x in range(x_min - 1, x_max + 2)]
    # left edge
    coords += [(x_min - 1, y) for y in range(y_min - 1, y_max + 2)]
    return coords


# 1-border around single pixel should be its neighbours
assert set(coords_to_iterate_over({(0, 0): "."})) == set(get_neighbours(0, 0))


def enhance(image, image_enhancement_algorithm, last_default):
    # this works for test input, but should really use first/last chars of the algorithm
    # to know what to do in all . and all # cases
    default = "#" if last_default == "." else "."
    new_image = defaultdict(lambda: default)

    for x, y in coords_to_iterate_over(image):
        pixels = "".join([mapping[image[nx, ny]] for nx, ny in get_neighbours(x, y)])
        index = int(pixels, base=2)
        # print(x, y, image[x, y], pixels, index)
        new_image[x, y] = image_enhancement_algorithm[index]
    return new_image, default


def num_lit(image):
    return sum(1 for pixel in image.values() if pixel == "#")


def reconstruct(image):
    coords = list(image.keys())
    x_coords = [x for x, _ in coords]
    y_coords = [y for _, y in coords]
    x_min, x_max = min(x_coords), max(x_coords)
    y_min, y_max = min(y_coords), max(y_coords)

    output = ""
    for y in range(y_min, y_max + 1):
        output += "".join([image[x, y] for x in range(x_min, x_max + 1)])
        output += "\n"

    return output


def main():
    path = "data/input_20.txt"
    # path = "data/example_20.txt"
    with open(path, "r") as f:
        image_enhancement_algorithm, _, *raw_image = [
            l.rstrip("\n") for l in f.readlines()
        ]

    default = "."
    image = defaultdict(lambda: default)
    for y, row in enumerate(raw_image):
        for x, value in enumerate(row):
            image[x, y] = value

    for iteration in range(1, 51):
        image, default = enhance(image, image_enhancement_algorithm, default)
        print(f"{num_lit(image)} lit after {iteration=}")
        # print(reconstruct(image))


if __name__ == "__main__":
    main()
