from collections import defaultdict


class Path:
    def __init__(self, path, counts=None) -> None:
        self.path = path
        self.counts = defaultdict(int)
        if counts is not None:
            for k, v in counts.items():
                self.counts[k] = v

    def __repr__(self):
        return "-".join(self.path)

    def copy(self):
        return Path(self.path.copy(), counts=self.counts)

    def extend(self, node):
        self.path.append(node)
        if node.islower():
            self.counts[node] += 1

    def finished(self):
        return self.path[-1] == "end"

    def valid(self):
        not_too_much = not self.counts or (max(self.counts.values()) < 3)
        at_most_one_twice = len([v for v in self.counts.values() if v == 2]) <= 1
        return self.counts["start"] == 1 and not_too_much and at_most_one_twice


def main():
    with open("data/input_12.txt", "r") as f:
        graph = defaultdict(set)
        for line in f.readlines():
            start, end = line.rstrip("\n").split("-")
            graph[start].add(end)
            graph[end].add(start)

    finished_paths = 0
    initial_path = Path(["start"], counts={"start": 1})
    current_paths = [initial_path]
    iteration = 0
    while current_paths:
        iteration += 1
        if iteration % 1 == 0:
            print(
                f"Iteration {iteration}, {finished_paths} finished paths, evaluating {len(current_paths)} candidates"
            )

        next_current_paths = []
        for path in current_paths:
            end = path.path[-1]
            for node in graph[end]:
                new_path = path.copy()
                new_path.extend(node)

                if new_path.finished():
                    finished_paths += 1
                elif not new_path.valid():
                    continue
                else:
                    next_current_paths.append(new_path)

        current_paths = next_current_paths

    print(finished_paths)


if __name__ == "__main__":
    main()
