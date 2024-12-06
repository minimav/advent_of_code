// go run puzzle_?/puzzle.go
package main

import (
	"fmt"
	"os"
	"strings"
)

type location struct {
	row    int
	column int
}

type location_with_direction struct {
	location  location
	direction string
}

type lab struct {
	positions [][]string
	location  location
	direction string
}

func make_array(input string) lab {
	lines := strings.Split(input, "\n")
	num_columns := len(lines[0])
	num_rows := len(lines)

	var array [][]string
	var start_row int
	var start_column int
	var direction string

	for row_index := 0; row_index < num_rows; row_index++ {
		var row []string
		for column_index := 0; column_index < num_columns; column_index++ {
			char := string(lines[row_index][column_index])
			if char != "." && char != "#" {
				start_column = column_index
				start_row = row_index
				row = append(row, ".")
				if char == "^" {
					direction = "up"
				} else if char == ">" {
					direction = "right"
				} else if char == "<" {
					direction = "left"
				} else if char == "v" {
					direction = "down"
				}
			} else {
				row = append(row, char)
			}
		}
		array = append(array, row)
	}
	return lab{
		positions: array,
		location: location{
			row:    start_row,
			column: start_column,
		},
		direction: direction,
	}
}

var offsets = map[string]location{
	"up": location{
		row:    -1,
		column: 0,
	},
	"down": location{
		row:    1,
		column: 0,
	},
	"left": location{
		row:    0,
		column: -1,
	},
	"right": location{
		row:    0,
		column: 1,
	},
}

var turn = map[string]string{
	"left":  "up",
	"up":    "right",
	"right": "down",
	"down":  "left",
}

func part_1(input string) {
	lab := make_array(input)
	num_columns := len(lab.positions[0])
	num_rows := len(lab.positions)
	visited := make(map[location]struct{})
	visited[lab.location] = struct{}{}
	for true {
		offset := offsets[lab.direction]
		next_row := lab.location.row + offset.row
		next_column := lab.location.column + offset.column
		if next_row >= num_rows || next_row < 0 || next_column < 0 || next_column >= num_columns {
			break
		}
		next_position := lab.positions[next_row][next_column]
		if next_position == "#" {
			// Turn right, remain at same place
			lab.direction = turn[lab.direction]
		} else {
			// Move to new location and mark it as seen
			lab.location = location{
				row:    next_row,
				column: next_column,
			}
			visited[lab.location] = struct{}{}
		}
	}
	fmt.Println(len(visited))
}

func part_2(input string) {
	lab := make_array(input)
	num_columns := len(lab.positions[0])
	num_rows := len(lab.positions)

	var possible_obstructions = []location{}
	for row_index := 0; row_index < num_rows; row_index++ {
		for column_index := 0; column_index < num_columns; column_index++ {
			if lab.positions[row_index][column_index] != "." || (lab.location.row == row_index && lab.location.column == column_index) {
				continue
			}
			possible_obstructions = append(
				possible_obstructions,
				location{
					row:    row_index,
					column: column_index,
				},
			)
		}
	}

	num_obstruction_loops := 0
	for _, obs := range possible_obstructions {
		lab := make_array(input)
		lab.positions[obs.row][obs.column] = "#"
		turned_at := make(map[location_with_direction]struct{})

		for true {
			offset := offsets[lab.direction]
			next_row := lab.location.row + offset.row
			next_column := lab.location.column + offset.column
			if next_row >= num_rows || next_row < 0 || next_column < 0 || next_column >= num_columns {
				break
			}
			next_position := lab.positions[next_row][next_column]
			if next_position == "#" {
				loc_dir := location_with_direction{
					location:  lab.location,
					direction: lab.direction,
				}
				_, seen_before := turned_at[loc_dir]
				if seen_before {
					num_obstruction_loops += 1
					break
				} else {
					turned_at[loc_dir] = struct{}{}
				}

				// Turn right, remain at same place
				lab.direction = turn[lab.direction]
			} else {
				// Move to new location
				lab.location = location{
					row:    next_row,
					column: next_column,
				}
			}
		}
	}
	fmt.Println(num_obstruction_loops)
}

func main() {
	example, err := os.ReadFile("puzzle_6/example.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example))
	part_2(string(example))

	input, err := os.ReadFile("puzzle_6/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(input))
	part_2(string(input))
}
