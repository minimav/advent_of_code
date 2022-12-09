from matplotlib import pyplot as plt

if __name__ == "__main__":
    with open("output.txt", "r") as f:
        positions = f.readlines()

    bound = 20
    for raw_positions in positions:
        plt.xlim(-bound, bound)
        plt.ylim(-bound, bound)
        plt.axis("off")

        x = []
        y = []
        for coords in raw_positions.lstrip(" ").rstrip(" \n").split(" "):
            knot_x, knot_y = map(int, coords.split(","))
            if max(knot_x, knot_y) > bound:
                break
            x.append(knot_x)
            y.append(knot_y)

        plt.plot(x, y, color="green", lw=3)
        plt.pause(0.00005)
        plt.clf()

    plt.show()
