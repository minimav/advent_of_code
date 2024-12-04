// go run puzzle_4/puzzle.go
package main

import (
	"fmt"
	"os"
	"strings"
)

func check_left(
	array [][]string,
	column_index int,
	row_index int,
	num_columns int,
	num_rows int,
) bool {
	if column_index < 3 {
		return false
	} else if array[row_index][column_index] == "X" && array[row_index][column_index-1] == "M" && array[row_index][column_index-2] == "A" && array[row_index][column_index-3] == "S" {
		return true
	}
	return false
}

func check_right(
	array [][]string,
	column_index int,
	row_index int,
	num_columns int,
	num_rows int,
) bool {
	if num_columns-column_index < 4 {
		return false
	} else if array[row_index][column_index] == "X" && array[row_index][column_index+1] == "M" && array[row_index][column_index+2] == "A" && array[row_index][column_index+3] == "S" {
		return true
	}
	return false
}

func check_up(
	array [][]string,
	column_index int,
	row_index int,
	num_columns int,
	num_rows int,
) bool {
	if row_index < 3 {
		return false
	} else if array[row_index][column_index] == "X" && array[row_index-1][column_index] == "M" && array[row_index-2][column_index] == "A" && array[row_index-3][column_index] == "S" {
		return true
	}
	return false
}

func check_down(
	array [][]string,
	column_index int,
	row_index int,
	num_columns int,
	num_rows int,
) bool {
	if num_rows-row_index < 4 {
		return false
	} else if array[row_index][column_index] == "X" && array[row_index+1][column_index] == "M" && array[row_index+2][column_index] == "A" && array[row_index+3][column_index] == "S" {
		return true
	}
	return false
}

func check_up_left(
	array [][]string,
	column_index int,
	row_index int,
	num_columns int,
	num_rows int,
) bool {
	if row_index < 3 || column_index < 3 {
		return false
	} else if array[row_index][column_index] == "X" && array[row_index-1][column_index-1] == "M" && array[row_index-2][column_index-2] == "A" && array[row_index-3][column_index-3] == "S" {
		return true
	}
	return false
}

func check_up_right(
	array [][]string,
	column_index int,
	row_index int,
	num_columns int,
	num_rows int,
) bool {
	if row_index < 3 || num_columns-column_index < 4 {
		return false
	} else if array[row_index][column_index] == "X" && array[row_index-1][column_index+1] == "M" && array[row_index-2][column_index+2] == "A" && array[row_index-3][column_index+3] == "S" {
		return true
	}
	return false
}

func check_down_left(
	array [][]string,
	column_index int,
	row_index int,
	num_columns int,
	num_rows int,
) bool {
	if num_rows-row_index < 4 || column_index < 3 {
		return false
	} else if array[row_index][column_index] == "X" && array[row_index+1][column_index-1] == "M" && array[row_index+2][column_index-2] == "A" && array[row_index+3][column_index-3] == "S" {
		return true
	}
	return false
}

func check_down_right(
	array [][]string,
	column_index int,
	row_index int,
	num_columns int,
	num_rows int,
) bool {
	if num_rows-row_index < 4 || num_columns-column_index < 4 {
		return false
	} else if array[row_index][column_index] == "X" && array[row_index+1][column_index+1] == "M" && array[row_index+2][column_index+2] == "A" && array[row_index+3][column_index+3] == "S" {
		return true
	}
	return false
}

func make_array(input string) [][]string {
	lines := strings.Split(input, "\n")
	num_columns := len(lines[0])
	num_rows := len(lines)
	var array [][]string
	for row_index := 0; row_index < num_rows; row_index++ {
		var row []string
		for column_index := 0; column_index < num_columns; column_index++ {
			row = append(row, string(lines[row_index][column_index]))
		}
		array = append(array, row)
	}
	return array
}

func part_1(input string) {
	array := make_array(input)
	num_columns := len(array[0])
	num_rows := len(array)

	num_xmas := 0
	for column_index := 0; column_index < num_columns; column_index++ {
		for row_index := 0; row_index < num_rows; row_index++ {
			if check_left(array, column_index, row_index, num_columns, num_rows) {
				num_xmas += 1
			}
			if check_right(array, column_index, row_index, num_columns, num_rows) {
				num_xmas += 1
			}
			if check_up(array, column_index, row_index, num_columns, num_rows) {
				num_xmas += 1
			}
			if check_down(array, column_index, row_index, num_columns, num_rows) {
				num_xmas += 1
			}
			if check_up_left(array, column_index, row_index, num_columns, num_rows) {
				num_xmas += 1
			}
			if check_up_right(array, column_index, row_index, num_columns, num_rows) {
				num_xmas += 1
			}
			if check_down_left(array, column_index, row_index, num_columns, num_rows) {
				num_xmas += 1
			}
			if check_down_right(array, column_index, row_index, num_columns, num_rows) {
				num_xmas += 1
			}
		}
	}
	fmt.Println(num_xmas)
}

func part_2(input string) {
	array := make_array(input)
	num_columns := len(array[0])
	num_rows := len(array)

	num_x_mas := 0
	// Centres identify X-MAS uniquely, these can only occur in the 'internal'
	// part of the grid
	for column_index := 1; column_index < num_columns-1; column_index++ {
		for row_index := 1; row_index < num_rows-1; row_index++ {
			if array[row_index][column_index] != "A" {
				continue
			}
			up_left_s := array[row_index-1][column_index-1] == "S"
			up_left_m := array[row_index-1][column_index-1] == "M"
			down_left_s := array[row_index+1][column_index-1] == "S"
			down_left_m := array[row_index+1][column_index-1] == "M"
			up_right_s := array[row_index-1][column_index+1] == "S"
			up_right_m := array[row_index-1][column_index+1] == "M"
			down_right_s := array[row_index+1][column_index+1] == "S"
			down_right_m := array[row_index+1][column_index+1] == "M"
			if ((up_left_m && down_right_s) || (up_left_s && down_right_m)) && ((up_right_m && down_left_s) || (up_right_s && down_left_m)) {
				num_x_mas += 1
			}
		}
	}
	fmt.Println(num_x_mas)
}

func main() {
	example, err := os.ReadFile("puzzle_4/example.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example))
	part_2(string(example))

	input, err := os.ReadFile("puzzle_4/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(input))
	part_2(string(input))
}
