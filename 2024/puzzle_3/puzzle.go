// go run puzzle_3/puzzle.go
package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func part_1(input string) {
	r, _ := regexp.Compile(`mul\((?P<left>\d{1,3}),(?P<right>\d{1,3})\)`)

	total := 0
	matches := r.FindAllStringSubmatch(input, -1)
	for _, v := range matches {
		left, _ := strconv.Atoi(v[1])
		right, _ := strconv.Atoi(v[2])
		total += left * right
	}
	fmt.Println((total))
}

func part_2(input string) {
	r, _ := regexp.Compile(`mul\((?P<left>\d{1,3}),(?P<right>\d{1,3})\)|do\(\)|don't\(\)`)

	total := 0
	enabled := true
	matches := r.FindAllStringSubmatch(input, -1)
	for _, v := range matches {
		if v[0] == "do()" {
			enabled = true
		} else if v[0] == "don't()" {
			enabled = false
		} else if enabled {
			left, _ := strconv.Atoi(v[1])
			right, _ := strconv.Atoi(v[2])
			total += left * right
		}
	}
	fmt.Println((total))
}

func main() {
	example, err := os.ReadFile("puzzle_3/example.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example))

	example_2, err := os.ReadFile("puzzle_3/example_2.txt")
	if err != nil {
		panic(err)
	}
	part_2(string(example_2))

	part_1_input, err := os.ReadFile("puzzle_3/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(part_1_input))
	part_2(string(part_1_input))
}
