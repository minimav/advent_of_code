// go run puzzle_?/puzzle.go
package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

func timeTrack(start time.Time, name string) {
	elapsed := time.Since(start)
	fmt.Printf("%s took %s\n", name, elapsed)
}

type equation struct {
	target  int
	numbers []int
}

func parse_line(line string) equation {
	parts := strings.Split(line, ": ")
	target, _ := strconv.Atoi(parts[0])
	numbers := []int{}
	for _, part := range strings.Split(parts[1], " ") {
		number, _ := strconv.Atoi(part)
		numbers = append(numbers, number)
	}
	return equation{target, numbers}
}

type partial_equation struct {
	current   int
	remaining []int
}

func solve_equation(equation equation, allow_concatenation bool) int {
	partials := []partial_equation{}
	partials = append(
		partials,
		partial_equation{
			current:   equation.numbers[0],
			remaining: equation.numbers[1:],
		},
	)
	for len(partials) > 0 {
		partial := partials[0]
		partials = partials[1:]

		if (partial.current == equation.target) && len(partial.remaining) == 0 {
			return equation.target
		} else if len(partial.remaining) == 0 {
			continue
		}
		sum := partial.current + partial.remaining[0]
		product := partial.current * partial.remaining[0]
		if sum <= equation.target {
			partials = append(
				partials,
				partial_equation{
					current:   sum,
					remaining: partial.remaining[1:],
				},
			)
		}
		if product <= equation.target {
			partials = append(
				partials,
				partial_equation{
					current:   product,
					remaining: partial.remaining[1:],
				},
			)
		}
		if !allow_concatenation {
			continue
		}
		concat, _ := strconv.Atoi(fmt.Sprintf("%d%d", partial.current, partial.remaining[0]))
		if concat <= equation.target {
			partials = append(
				partials,
				partial_equation{
					current:   concat,
					remaining: partial.remaining[1:],
				},
			)
		}
	}
	return 0
}

func part_1(input string) {
	defer timeTrack(time.Now(), "part_1")
	lines := strings.Split(input, "\n")
	answer := 0
	for _, line := range lines {
		answer += solve_equation(parse_line(line), false)
	}
	fmt.Println(answer)
}

func part_2(input string) {
	defer timeTrack(time.Now(), "part_2")
	lines := strings.Split(input, "\n")
	answer := 0
	for _, line := range lines {
		answer += solve_equation(parse_line(line), true)
	}
	fmt.Println(answer)
}

func main() {
	example, err := os.ReadFile("puzzle_7/example.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example))
	part_2(string(example))

	input, err := os.ReadFile("puzzle_7/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(input))
	part_2(string(input))
}
