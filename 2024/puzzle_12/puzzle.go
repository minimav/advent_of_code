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

type location struct {
	row    int
	column int
}

func part_1(input string) {
	defer timeTrack(time.Now(), "part_1")
	lines := strings.Split(input, "\n")
	num_rows := len(lines)
	num_columns := len(lines[0])
	grid := make(map[location]rune)
	for row_index, row := range lines {
		for column_index, rune := range row {
			grid[location{row: row_index, column: column_index}] = rune
		}
	}

	answer := 0
	seen := make(map[location]struct{})
	for start_location, value := range grid {
		area := 0
		perimeter := 0
		_, done := seen[start_location]
		if done {
			continue
		}
		var pool = []location{start_location}
		for len(pool) > 0 {
			loc := pool[0]
			pool = pool[1:]
			_, done := seen[loc]
			if done {
				continue
			}
			seen[loc] = struct{}{}
			area += 1
			moves := []location{
				{row: loc.row - 1, column: loc.column},
				{row: loc.row + 1, column: loc.column},
				{row: loc.row, column: loc.column - 1},
				{row: loc.row, column: loc.column + 1},
			}
			for _, move := range moves {
				if move.row < 0 || move.column < 0 || move.row >= num_rows || move.column >= num_columns {
					perimeter += 1
					continue
				} else if grid[move] != value {
					// No perimeter adding in this case
					perimeter += 1
					continue
				} else if _, done := seen[move]; done {
					continue
				}
				pool = append(pool, move)
			}
		}
		answer += area * perimeter

	}
	fmt.Println(answer)
}

func part_2(input string) {
	defer timeTrack(time.Now(), "part_2")
	// Traverse via right hand rule keeping track of side?
	// Get from area and perimeter in a clever way?
	lines := strings.Split(input, "\n")
	num_rows := len(lines)
	num_columns := len(lines[0])
	grid := make(map[location]rune)
	for row_index, row := range lines {
		for column_index, rune := range row {
			grid[location{row: row_index, column: column_index}] = rune
		}
	}

	answer := 0
	seen := make(map[location]struct{})
	for start_location, value := range grid {
		area := 0
		perimeter := 0
		_, done := seen[start_location]
		if done {
			continue
		}
		var pool = []location{start_location}
		for len(pool) > 0 {
			loc := pool[0]
			pool = pool[1:]
			_, done := seen[loc]
			if done {
				continue
			}
			seen[loc] = struct{}{}
			area += 1
			moves := []location{
				{row: loc.row - 1, column: loc.column},
				{row: loc.row + 1, column: loc.column},
				{row: loc.row, column: loc.column - 1},
				{row: loc.row, column: loc.column + 1},
			}
			for _, move := range moves {
				if move.row < 0 || move.column < 0 || move.row >= num_rows || move.column >= num_columns {
					perimeter += 1
					continue
				} else if grid[move] != value {
					// No perimeter adding in this case
					perimeter += 1
					continue
				} else if _, done := seen[move]; done {
					continue
				}
				pool = append(pool, move)
			}
		}
		answer += area * perimeter

	}
	fmt.Println(answer)
}

func main() {

	for i := 3; i <= 3; i++ {
		example, err := os.ReadFile(fmt.Sprintf("puzzle_12/example_%d.txt", i))
		if err != nil {
			panic(err)
		}
		part_1(string(example))
		part_2(string(example))
	}

	input, err := os.ReadFile("puzzle_12/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(input))
	part_2(string(input))
}
