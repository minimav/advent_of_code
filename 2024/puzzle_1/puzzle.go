// go run puzzle_1/puzzle.go
package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func part_1(input string) {
	var left []int
	var right []int
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		values := strings.Split(line, " ")
		left_value, _ := strconv.Atoi(values[0])
		right_value, _ := strconv.Atoi(values[len(values)-1])
		left = append(left, left_value)
		right = append(right, right_value)
	}

	diff := 0
	slices.Sort(left)
	slices.Sort(right)
	for index, left_value := range left {
		right_value := right[index]
		if right_value > left_value {
			diff += right_value - left_value
		} else {
			diff += left_value - right_value
		}
	}
	fmt.Println(diff)
}

func part_2(input string) {
	var left []int
	right_counts := make(map[int]int)
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		values := strings.Split(line, " ")
		left_value, _ := strconv.Atoi(values[0])
		right_value, _ := strconv.Atoi(values[len(values)-1])
		left = append(left, left_value)

		_, seen_before := right_counts[right_value]
		if seen_before {
			right_counts[right_value] += 1
		} else {
			right_counts[right_value] = 1
		}
	}

	code := 0
	for _, left_value := range left {
		count, in_right := right_counts[left_value]
		if in_right {
			code += left_value * count
		}
	}
	fmt.Println(code)
}

func main() {
	example, err := os.ReadFile("puzzle_1/example.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example))
	part_2(string(example))

	part_1_input, err := os.ReadFile("puzzle_1/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(part_1_input))
	part_2(string(part_1_input))
}
