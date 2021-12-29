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
        if node.islower() and node not in {"start", "end"}:
            self.counts[node] += 1

    def finished(self):
        return self.path[-1] == "end"

    def valid(self):
        return all(v < 2 for v in self.counts.values())


def main():
    with open("data/input_12.txt", "r") as f:
        raw_graph = defaultdict(set)
        for line in f.readlines():
            start, end = line.rstrip("\n").split("-")
            raw_graph[start].add(end)
            raw_graph[end].add(start)

    # remove dead end lower case nodes + prevent going back to the start
    intermediate_graph = {}
    to_remove = {"start"}
    for node, neighbours in raw_graph.items():
        if not node.islower() or len(neighbours) > 1:
            intermediate_graph[node] = neighbours
            continue

        if all(n.islower() for n in neighbours):
            to_remove.add(node)
        else:
            intermediate_graph[node] = neighbours

    graph = {
        node: [n for n in neighbours if n not in to_remove]
        for node, neighbours in intermediate_graph.items()
    }

    finished_paths = 0
    initial_path = Path(["start"])
    current_paths = [initial_path]
    iteration = 0
    while current_paths:
        iteration += 1
        if iteration % 10 == 0:
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
