// go run puzzle_?/puzzle.go
package main

import (
	"fmt"
	"os"
	"strings"
	"time"
)

func timeTrack(start time.Time, name string) {
	elapsed := time.Since(start)
	fmt.Printf("%s took %s\n", name, elapsed)
}

func part_1(input string) {
	defer timeTrack(time.Now(), "part_1")
	lines := strings.Split(input, "\n")

	atoms := make(map[string]struct{})
	for _, atom := range strings.Split(lines[0], ", ") {
		atoms[atom] = struct{}{}
	}

	solvable := 0
	for _, line := range lines[2:] {
		current_substrings := make(map[string]struct{})
		current_substrings[""] = struct{}{}
		for len(current_substrings) > 0 {
			var substring string
			for iter_substring := range current_substrings {
				substring = iter_substring
				break
			}
			delete(current_substrings, substring)

			if substring == line {
				solvable += 1
				break
			}

			// Check if atoms can be added
			for atom, _ := range atoms {
				new_substring := substring + atom
				if strings.HasPrefix(line, new_substring) {
					current_substrings[new_substring] = struct{}{}
				}
			}
		}
	}
	fmt.Println(solvable)
}

func part_2(input string) {
	defer timeTrack(time.Now(), "part_2")
	lines := strings.Split(input, "\n")

	atoms := make(map[string]struct{})
	for _, atom := range strings.Split(lines[0], ", ") {
		atoms[atom] = struct{}{}
	}

	ways_solvable := 0
	for _, line := range lines[2:] {
		current_substrings := make(map[string]int)
		current_substrings[""] = 1
		for len(current_substrings) > 0 {
			var substring string
			var count int
			min_length := len(line) + 1
			for iter_substring, iter_count := range current_substrings {
				// Inefficient, but hey ho...
				if new_len := len(iter_substring); new_len < min_length {
					substring = iter_substring
					count = iter_count
					min_length = new_len
				}
			}
			if substring == line {
				break
			}
			delete(current_substrings, substring)

			// Check if atoms can be added
			for atom, _ := range atoms {
				new_substring := substring + atom
				if strings.HasPrefix(line, new_substring) {
					prior_count, exists := current_substrings[new_substring]
					if exists {
						current_substrings[new_substring] = count + prior_count
					} else {
						current_substrings[new_substring] = count
					}
				}
			}
		}
		ways_solvable += current_substrings[line]

	}
	fmt.Println(ways_solvable)
}

func main() {
	example, err := os.ReadFile("puzzle_19/example.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example))
	part_2(string(example))

	input, err := os.ReadFile("puzzle_19/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(input))
	part_2(string(input))
}
