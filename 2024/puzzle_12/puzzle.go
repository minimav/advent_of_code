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

type location_border struct {
	loc    location
	border string
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
	for row := 0; row < num_rows; row++ {
		for column := 0; column < num_columns; column++ {
			start_location := location{row, column}
			_, done := seen[start_location]
			if done {
				continue
			}

			area := 0
			value := grid[start_location]
			polygon := make(map[location]struct{})
			perimeter := make(map[location_border]struct{})
			var pool = []location{start_location}
			for len(pool) > 0 {
				loc := pool[0]
				pool = pool[1:]
				_, done := seen[loc]
				if done {
					continue
				}
				seen[loc] = struct{}{}
				polygon[loc] = struct{}{}
				area += 1
				moves := map[string]location{
					"up":    {row: loc.row - 1, column: loc.column},
					"down":  {row: loc.row + 1, column: loc.column},
					"left":  {row: loc.row, column: loc.column - 1},
					"right": {row: loc.row, column: loc.column + 1},
				}
				for direction, move := range moves {
					if move.row < 0 || move.column < 0 || move.row >= num_rows || move.column >= num_columns {
						perimeter[location_border{loc: loc, border: direction}] = struct{}{}
						continue
					} else if grid[move] != value {
						perimeter[location_border{loc: loc, border: direction}] = struct{}{}
						continue
					} else if _, done := seen[move]; done {
						continue
					}
					pool = append(pool, move)
				}
			}

			//fmt.Println(perimeter)
			num_lines := 0
			border_seen := make(map[location_border]struct{})
			for start_border, _ := range perimeter {
				if _, done := border_seen[start_border]; done {
					// Already seen this connected component of the perimeter
					continue
				}

				var border = start_border
				//fmt.Println("Start perimeter traversal from", border)
				for {
					//fmt.Println("Moving from", border)
					if _, done := border_seen[border]; done {
						break
					}
					border_seen[border] = struct{}{}
					if border.border == "up" {
						up_right := location_border{
							loc:    location{row: border.loc.row - 1, column: border.loc.column + 1},
							border: "left",
						}
						right := location_border{
							loc:    location{row: border.loc.row, column: border.loc.column + 1},
							border: "up",
						}
						turn_right := location_border{
							loc:    border.loc,
							border: "right",
						}
						if _, ok := perimeter[up_right]; ok {
							border = up_right
							num_lines += 1
						} else if _, ok := perimeter[right]; ok {
							border = right
						} else {
							border = turn_right
							num_lines += 1
						}
					} else if border.border == "left" {
						up_left := location_border{
							loc:    location{row: border.loc.row - 1, column: border.loc.column - 1},
							border: "down",
						}
						up := location_border{
							loc:    location{row: border.loc.row - 1, column: border.loc.column},
							border: "left",
						}
						turn_right := location_border{
							loc:    border.loc,
							border: "up",
						}
						if _, ok := perimeter[up_left]; ok {
							border = up_left
							num_lines += 1
						} else if _, ok := perimeter[up]; ok {
							border = up
						} else {
							border = turn_right
							num_lines += 1
						}
					} else if border.border == "right" {
						down_right := location_border{
							loc:    location{row: border.loc.row + 1, column: border.loc.column + 1},
							border: "up",
						}
						down := location_border{
							loc:    location{row: border.loc.row + 1, column: border.loc.column},
							border: "right",
						}
						turn_right := location_border{
							loc:    border.loc,
							border: "down",
						}
						if _, ok := perimeter[down_right]; ok {
							border = down_right
							num_lines += 1
						} else if _, ok := perimeter[down]; ok {
							border = down
						} else {
							border = turn_right
							num_lines += 1
						}
					} else if border.border == "down" {
						down_left := location_border{
							loc:    location{row: border.loc.row + 1, column: border.loc.column - 1},
							border: "right",
						}
						left := location_border{
							loc:    location{row: border.loc.row, column: border.loc.column - 1},
							border: "down",
						}
						turn_right := location_border{
							loc:    border.loc,
							border: "left",
						}
						if _, ok := perimeter[down_left]; ok {
							border = down_left
							num_lines += 1
						} else if _, ok := perimeter[left]; ok {
							border = left
						} else {
							border = turn_right
							num_lines += 1
						}
					}
					//fmt.Println("Moved to", border)
				}
			}
			//fmt.Println(string(value), polygon)
			//fmt.Println(area, num_lines)
			answer += area * num_lines
		}
	}
	fmt.Println(answer)
}

func main() {

	for i := 1; i <= 5; i++ {
		fmt.Printf("Example %d\n", i)
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
