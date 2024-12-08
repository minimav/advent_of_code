// go run puzzle_?/puzzle.go
package main

import (
	"fmt"
	"os"
	"strings"
	"time"
)

type location struct {
	row    int
	column int
}

func (loc location) add(other_loc location) location {
	return location{
		row:    loc.row + other_loc.row,
		column: loc.column + other_loc.column,
	}
}

func (loc location) diff(other_loc location) location {
	return location{
		row:    loc.row - other_loc.row,
		column: loc.column - other_loc.column,
	}
}

func (loc location) minus() location {
	return location{
		row:    -loc.row,
		column: -loc.column,
	}
}

type antenna_grid struct {
	grid        map[string][]location
	num_rows    int
	num_columns int
}

func (a antenna_grid) in_bounds(loc location) bool {
	if loc.row < 0 || loc.row >= a.num_rows {
		return false
	}
	if loc.column < 0 || loc.column >= a.num_columns {
		return false
	}
	return true
}

func timeTrack(start time.Time, name string) {
	elapsed := time.Since(start)
	fmt.Printf("%s took %s\n", name, elapsed)
}

func build_antenna_grid(input string) antenna_grid {
	lines := strings.Split(input, "\n")
	num_columns := len(lines[0])
	num_rows := len(lines)

	grid := make(map[string][]location)
	for row, line := range lines {
		for column, char := range line {
			freq := string(char)
			if freq == "." {
				continue
			}
			loc := location{row, column}
			_, exists := grid[freq]
			if !exists {
				grid[freq] = []location{}
			}
			grid[freq] = append(grid[freq], loc)
		}
	}
	return antenna_grid{
		grid,
		num_rows,
		num_columns,
	}
}

func part_1(input string) {
	defer timeTrack(time.Now(), "part_1")
	antenna_grid := build_antenna_grid(input)

	unique_res_locs := make(map[location]struct{})
	for _, freq_locs := range antenna_grid.grid {
		for i, first_loc := range freq_locs {
			for j := i + 1; j < len(freq_locs); j++ {
				second_loc := freq_locs[j]
				diff := first_loc.diff(second_loc)

				possible_res_locs := []location{
					first_loc.add(diff),
					second_loc.add(diff.minus()),
				}
				for _, loc := range possible_res_locs {
					if !antenna_grid.in_bounds(loc) {
						continue
					}
					unique_res_locs[loc] = struct{}{}
				}
			}
		}
	}

	fmt.Println(len(unique_res_locs))
}

func printResonance(antenna_grid antenna_grid, unique_res_locs map[location]struct{}) {
	for row := 0; row < antenna_grid.num_rows; row++ {
		for column := 0; column < antenna_grid.num_columns; column++ {
			_, exists := unique_res_locs[location{row, column}]
			if exists {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Print("\n")
	}
}

func part_2(input string) {
	defer timeTrack(time.Now(), "part_2")
	antenna_grid := build_antenna_grid(input)

	unique_res_locs := make(map[location]struct{})
	for _, freq_locs := range antenna_grid.grid {
		for i, first_loc := range freq_locs {
			for j := i + 1; j < len(freq_locs); j++ {
				second_loc := freq_locs[j]
				diff := first_loc.diff(second_loc)

				var one_dir = first_loc
				for antenna_grid.in_bounds(one_dir) {
					unique_res_locs[one_dir] = struct{}{}
					one_dir = one_dir.add(diff)
				}

				var other_dir = second_loc
				for antenna_grid.in_bounds(other_dir) {
					unique_res_locs[other_dir] = struct{}{}
					other_dir = other_dir.add(diff.minus())
				}
			}
		}
	}
	//printResonance(antenna_grid, unique_res_locs)
	fmt.Println(len(unique_res_locs))
}

func main() {
	example, err := os.ReadFile("puzzle_8/example.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example))
	part_2(string(example))

	input, err := os.ReadFile("puzzle_8/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(input))
	part_2(string(input))
}
