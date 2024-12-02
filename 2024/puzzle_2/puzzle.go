// go run puzzle_2/puzzle.go
package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func check_levels(line string) bool {
	safe := true
	values := strings.Split(line, " ")
	increasing := false

	for i := 0; i < len(values)-1; i += 1 {
		var pair []string = values[i : i+2]
		left, _ := strconv.Atoi(pair[0])
		right, _ := strconv.Atoi(pair[1])

		if right == left {
			return false

		}
		if (right > left) && (i == 0) {
			increasing = true
		} else if i == 0 {
			increasing = false
		}

		if (increasing && (right-left > 3)) ||
			(!increasing && (left-right > 3)) ||
			(increasing && right <= left) || (!increasing && right >= left) {
			return false

		}
	}
	return safe
}

func part_1(input string) {
	num_safe := 0
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		safe := check_levels(line)
		if safe {
			num_safe += 1
		}
	}
	fmt.Println(num_safe)
}

func check_levels_with_skip(line string, skip int) bool {
	safe := true
	values := strings.Split(line, " ")
	increasing := false

	var non_skipped_values []int
	for i, value := range values {
		if i == skip {
			continue
		}
		num, _ := strconv.Atoi(value)
		non_skipped_values = append(non_skipped_values, num)
	}

	for i := 0; i < len(non_skipped_values)-1; i += 1 {
		var pair []int = non_skipped_values[i : i+2]
		left := pair[0]
		right := pair[1]

		if right == left {
			return false

		}
		if (right > left) && (i == 0) {
			increasing = true
		} else if i == 0 {
			increasing = false
		}

		if (increasing && (right-left > 3)) ||
			(!increasing && (left-right > 3)) ||
			(increasing && right <= left) || (!increasing && right >= left) {
			return false

		}
	}
	return safe
}

func part_2(input string) {
	num_safe := 0
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		if check_levels(line) {
			num_safe += 1
			continue
		}

		safe_with_skip := false
		values := strings.Split(line, " ")
		for skip_index := 0; skip_index < len(values); skip_index += 1 {
			safe := check_levels_with_skip(line, skip_index)
			if safe {
				safe_with_skip = true
				break
			}
		}
		if safe_with_skip {
			num_safe += 1
		}
	}
	fmt.Println(num_safe)
}

func main() {
	example, err := os.ReadFile("puzzle_2/example.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example))
	part_2(string(example))

	part_1_input, err := os.ReadFile("puzzle_2/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(part_1_input))
	part_2(string(part_1_input))
}
