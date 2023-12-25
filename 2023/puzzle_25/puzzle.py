import networkx as nx
import numpy as np


def puzzle(input):
    graph = nx.Graph()
    for line in input.splitlines():
        node, tos = line.split(": ")
        for to in tos.split(" "):
            if node not in graph:
                graph.add_node(node)
            if to not in graph:
                graph.add_node(to)
            graph.add_edge(node, to)

    nodes = list(graph.nodes())
    L = nx.normalized_laplacian_matrix(graph)
    e, v = np.linalg.eigh(L.toarray())

    min_e = min(e)
    second_min_e_index = min(
        range(len(e)), key=lambda i: e[i] if e[i] != min_e else float("inf")
    )
    second_min_e_v = v[:, second_min_e_index]

    sort_index = np.argsort(second_min_e_v)
    sorted_nodes = np.array(nodes)[sort_index]
    for i in range(len(sorted_nodes) - 1):
        s = sorted_nodes[: i + 1]
        t = sorted_nodes[i + 1 :]
        conductance = nx.conductance(graph, s, t)
        cut_size = nx.cut_size(graph, s, t)
        if cut_size == 3:
            print("######### Potential solution")
            print(conductance)
            print(cut_size)
            print(len(s), len(t), len(s) * len(t))


if __name__ == "__main__":
    with open("example.txt", "r") as f:
        example = f.read()

    with open("input.txt", "r") as f:
        input = f.read()

    puzzle(example)
    puzzle(input)
