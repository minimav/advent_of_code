// go run puzzle_?/puzzle.go
package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func part_1(input string) {
	lines := strings.Split(input, "\n")
	var orders = map[int][]int{}
	var break_index = 0
	for index, line := range lines {
		if line == "" {
			break_index = index + 1
			break
		}
		pair_ordering := strings.Split(line, "|")
		left, _ := strconv.Atoi(pair_ordering[0])
		right, _ := strconv.Atoi(pair_ordering[1])

		if _, ok := orders[left]; ok {
			orders[left] = append(orders[left], right)
		} else {
			orders[left] = []int{right}
		}
	}

	total := 0
	for _, line := range lines[break_index:] {
		var values = []int{}
		for _, value := range strings.Split(line, ",") {
			parsed, _ := strconv.Atoi(value)
			values = append(values, parsed)
		}

		passes := true
	checks:
		for i, value := range values {
			comparisons := orders[value]
			for j := i - 1; j >= 0; j-- {
				if slices.Contains(comparisons, values[j]) {
					passes = false
					break checks
				}

			}
		}
		if passes {
			middle := values[(len(values)-1)/2]
			total += middle
		}
	}
	fmt.Println(total)
}

func reorder(values []int, orders map[int][]int) []int {
	// Should this algorithm always work?! I'm not convinced even though it works
	// Maybe only if there is a guaranteed total ordering?
	var correct_order = []int{}
	for i, value := range values {
		if i == 0 {
			correct_order = append(correct_order, value)
			continue
		}
		inserted := false
		comparison := orders[value]
		for j, other_value := range correct_order {
			if slices.Contains(comparison, other_value) {
				// value should be before this other one
				correct_order = slices.Insert(correct_order, j, value)
				inserted = true
				break
			}
		}
		if !inserted {
			correct_order = append(correct_order, value)
		}
	}
	return correct_order
}

func part_2(input string) {
	lines := strings.Split(input, "\n")
	var orders = map[int][]int{}
	var break_index = 0
	for index, line := range lines {
		if line == "" {
			break_index = index + 1
			break
		}
		pair_ordering := strings.Split(line, "|")
		left, _ := strconv.Atoi(pair_ordering[0])
		right, _ := strconv.Atoi(pair_ordering[1])

		if _, ok := orders[left]; ok {
			orders[left] = append(orders[left], right)
		} else {
			orders[left] = []int{right}
		}
	}

	total := 0
	for _, line := range lines[break_index:] {
		var values = []int{}
		for _, value := range strings.Split(line, ",") {
			parsed, _ := strconv.Atoi(value)
			values = append(values, parsed)
		}

		passes := true
	checks:
		for i, value := range values {
			comparisons := orders[value]
			for j := i - 1; j >= 0; j-- {
				if slices.Contains(comparisons, values[j]) {
					passes = false
					break checks
				}

			}
		}
		if !passes {
			reordered_values := reorder(values, orders)
			middle := reordered_values[(len(reordered_values)-1)/2]
			total += middle
		}
	}
	fmt.Println(total)
}

func main() {
	example, err := os.ReadFile("puzzle_5/example.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example))
	part_2(string(example))

	input, err := os.ReadFile("puzzle_5/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(input))
	part_2(string(input))
}
