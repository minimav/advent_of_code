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

func parse_lock(lines []string) []int {
	lock := []int{-1, -1, -1, -1, -1}
	for _, line := range lines {
		for i, char := range line {
			if string(char) == "#" {
				lock[i] += 1
			}
		}
	}
	return lock
}

func parse_key(lines []string) []int {
	key := []int{-1, -1, -1, -1, -1}
	num_lines := len(lines)
	for j := num_lines - 1; j >= 0; j-- {
		line := lines[j]
		for i, char := range line {
			if string(char) == "#" {
				key[i] += 1
			}
		}
	}
	return key
}

func part_1(input string) {
	defer timeTrack(time.Now(), "part_1")
	lines := strings.Split(input, "\n")

	locks := [][]int{}
	keys := [][]int{}
	i := 0
	for i < len(lines) {
		line := lines[i]
		if len(line) == 0 {
			i += 1
			continue
		} else if line == "#####" {
			locks = append(locks, parse_lock(lines[i:i+7]))
			i += 7

		} else if line == "....." {
			keys = append(keys, parse_key(lines[i:i+7]))
			i += 7
		}
	}
	answer := 0
	for _, key := range keys {
		for _, lock := range locks {
			fits := true
			for i, pin := range lock {
				if pin+key[i] > 5 {
					fits = false
					break
				}
			}
			if fits {
				answer += 1
			}
		}
	}
	fmt.Println(answer)
}

func main() {
	example, err := os.ReadFile("puzzle_25/example.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example))

	input, err := os.ReadFile("puzzle_25/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(input))
}
