// go run puzzle_?/puzzle.go
package main

import (
	"fmt"
	"os"
	"sort"
	"strings"
	"time"
)

func timeTrack(start time.Time, name string) {
	elapsed := time.Since(start)
	fmt.Printf("%s took %s\n", name, elapsed)
}

type clique struct {
	a string
	b string
	c string
}

func parse_graph(input string) map[string]map[string]struct{} {
	lines := strings.Split(input, "\n")
	graph := make(map[string]map[string]struct{})
	for _, line := range lines {
		node_1 := line[:2]
		node_2 := line[3:]

		if _, seen_1 := graph[node_1]; !seen_1 {
			graph[node_1] = make(map[string]struct{})
		}
		graph[node_1][node_2] = struct{}{}
		if _, seen_2 := graph[node_2]; !seen_2 {
			graph[node_2] = make(map[string]struct{})
		}
		graph[node_2][node_1] = struct{}{}
	}
	return graph
}

func part_1(input string) {
	defer timeTrack(time.Now(), "part_1")
	graph := parse_graph(input)

	seen_cliques := make(map[clique]struct{})
	for node, onodes := range graph {
		for onode, _ := range onodes {
			oonodes, _ := graph[onode]
			for oonode, _ := range oonodes {
				_, complete := onodes[oonode]
				if complete {
					nodes := []string{node, onode, oonode}
					sort.Slice(nodes, func(i, j int) bool {
						return nodes[i] < nodes[j]
					})
					clique := clique{
						a: nodes[0],
						b: nodes[1],
						c: nodes[2],
					}
					seen_cliques[clique] = struct{}{}
				}
			}
		}
	}
	fmt.Println(len(seen_cliques))
	answer := 0
	for clique, _ := range seen_cliques {
		if strings.HasPrefix(clique.a, "t") || strings.HasPrefix(clique.b, "t") || strings.HasPrefix(clique.c, "t") {
			answer += 1
		}
	}
	fmt.Println(answer)
}

func make_key(nodes []string) string {
	return strings.Join(nodes, ",")
}

func intersect(a map[string]struct{}, b map[string]struct{}, c string, d string) map[string]struct{} {
	out := make(map[string]struct{})
	for v, _ := range a {
		if _, exists := b[v]; exists {
			out[v] = struct{}{}
		}
	}
	if c != "" {
		out[c] = struct{}{}
	}
	if d != "" {
		out[d] = struct{}{}
	}
	return out
}

type pclique struct {
	intersection map[string]struct{}
	remaining    []string
}

func part_2(input string) {
	defer timeTrack(time.Now(), "part_2")
	lines := strings.Split(input, "\n")
	graph := parse_graph(input)

	potential_cliques := []pclique{}
	for _, line := range lines {
		a := line[:2]
		b := line[3:]
		a_nodes, _ := graph[a]
		b_nodes, _ := graph[b]
		intersection := intersect(a_nodes, b_nodes, a, b)
		if len(intersection) == 0 {
			continue
		}
		remaining := []string{}
		for k, _ := range intersection {
			if k != a && k != b {
				remaining = append(remaining, k)
			}
		}
		if len(remaining) == 0 {
			continue
		}

		pclique := pclique{
			intersection, remaining,
		}
		potential_cliques = append(potential_cliques, pclique)
	}

	current_best_length := 0
	var current_best pclique
	for len(potential_cliques) > 0 {
		clique := potential_cliques[0]
		potential_cliques = potential_cliques[1:]

		if len(clique.remaining) == 0 {
			//fmt.Printf("Found clique of length %d", len(clique.intersection))
			//fmt.Println(clique.intersection)
			if len(clique.intersection) > current_best_length {
				current_best_length = len(clique.intersection)
				current_best = clique
			}
			continue
		}

		node := clique.remaining[0]
		next_remaining := clique.remaining[1:]
		new_nodes, _ := graph[node]
		next_intersection := intersect(clique.intersection, new_nodes, node, "")
		new_clique := pclique{
			intersection: next_intersection,
			remaining:    next_remaining,
		}
		potential_cliques = append(potential_cliques, new_clique)
	}
	fmt.Println(current_best, current_best_length)

	nodes := []string{}
	for k, _ := range current_best.intersection {
		nodes = append(nodes, k)
	}
	sort.Slice(nodes, func(i, j int) bool {
		return nodes[i] < nodes[j]
	})
	fmt.Println(make_key(nodes))
}

func main() {
	example, err := os.ReadFile("puzzle_23/example.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example))
	part_2(string(example))

	input, err := os.ReadFile("puzzle_23/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(input))
	part_2(string(input))
}
