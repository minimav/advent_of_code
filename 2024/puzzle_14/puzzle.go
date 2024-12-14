// go run puzzle_?/puzzle.go
package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
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

type robot struct {
	loc    location
	vector location
}

func (r *robot) move(num_rows int, num_columns int, num_moves int) {
	new_row := (r.loc.row + r.vector.row*num_moves) % num_rows
	new_column := (r.loc.column + r.vector.column*num_moves) % num_columns
	if new_row < 0 {
		new_row += num_rows
	}
	if new_column < 0 {
		new_column += num_columns
	}
	r.loc = location{
		row:    new_row,
		column: new_column,
	}
}

func printRobots(locations map[location]int, num_rows int, num_columns int) {
	for row := 0; row < num_rows; row++ {
		fmt.Printf("|%3d|", row)
		for column := 0; column < num_columns; column++ {
			if count, has_count := locations[location{row, column}]; has_count {
				fmt.Printf("%d", count)
			} else {
				fmt.Print(".")
			}
		}
		fmt.Print("\n")
	}
}

type rle struct {
	num_robots int
	count      int
}

func rleRobots(locations map[location]int, num_rows int, num_columns int) []rle {
	encoding := []rle{}
	current := -1
	current_count := 0
	for row := 0; row < num_rows; row++ {
		for column := 0; column < num_columns; column++ {
			num_robots, has_count := locations[location{row, column}]
			next := 0
			if has_count {
				next = num_robots
			}
			if next == current {
				current_count += 1
			} else {
				if current_count > 0 {
					encoding = append(encoding, rle{
						num_robots: current,
						count:      current_count,
					})
				}
				current_count = 1
				current = next
			}
		}
	}
	if current_count > 0 {
		encoding = append(encoding, rle{
			num_robots: current,
			count:      current_count,
		})
	}
	return encoding
}

func reverse_entropy(encoding []rle) float64 {
	num := 0
	denom := 0
	for _, r := range encoding {
		if r.num_robots == 0 {
			continue
		}
		denom += 1
		num += r.count
	}
	// Count average run length of non-zero counts
	return float64(num) / float64(denom)
}

func parse_robots(input string) []robot {
	lines := strings.Split(input, "\n")

	robots := []robot{}
	for _, line := range lines {
		p, _ := regexp.Compile(`p=(?P<left>\d{1,3}),(?P<right>\d{1,3})`)
		v, _ := regexp.Compile(`v=(?P<left>[-]?\d{1,3}),(?P<right>[-]?\d{1,3})`)

		p_matches := p.FindStringSubmatch(line)
		v_matches := v.FindStringSubmatch(line)

		p_column, _ := strconv.Atoi(p_matches[1])
		p_row, _ := strconv.Atoi(p_matches[2])
		v_column, _ := strconv.Atoi(v_matches[1])
		v_row, _ := strconv.Atoi(v_matches[2])
		robots = append(
			robots,
			robot{
				loc:    location{row: p_row, column: p_column},
				vector: location{row: v_row, column: v_column},
			},
		)
	}
	return robots
}

func part_1(input string, num_rows int, num_columns int) {
	defer timeTrack(time.Now(), "part_1")
	robots := parse_robots(input)

	// Perform moves all in one go
	quadrants := []int{0, 0, 0, 0}
	row_factor := ((num_rows - 1) / 2)
	column_factor := ((num_columns - 1) / 2)
	num_moves := 100
	locations := make(map[location]int)
	for _, robot := range robots {
		robot.move(num_rows, num_columns, num_moves)

		if _, seen := locations[robot.loc]; seen {
			locations[robot.loc] += 1
		} else {
			locations[robot.loc] = 1
		}

		if robot.loc.row == row_factor || robot.loc.column == column_factor {
			// In middle case
			continue
		}

		q_row := robot.loc.row / (row_factor + 1)
		q_column := robot.loc.column / (column_factor + 1)
		quadrants[2*q_row+q_column] += 1
	}

	answer := 1
	for _, quad_count := range quadrants {
		answer *= quad_count
	}
	fmt.Println(answer)
}

func part_2(input string) {
	defer timeTrack(time.Now(), "part_2")
	num_rows := 103
	num_columns := 101

	robots := parse_robots(input)

	for move := 1; move > 0; move++ {
		// Simpler check for first few rows being sparse
		by_row := make(map[int]int)
		for _, robot := range robots {
			robot.move(num_rows, num_columns, move)
			if _, seen := by_row[robot.loc.row]; !seen {
				by_row[robot.loc.row] = 1
			} else {
				by_row[robot.loc.row] += 1
			}
		}

		// 500 trees across ~100 rows, expect 50 in first 10
		count := 0
		for row := 0; row < 10; row++ {
			num_values, row_exists := by_row[row-1]
			if row_exists {
				count += num_values
			}
		}
		is_christmas_tree := count < 20
		if is_christmas_tree {
			locations := make(map[location]int)
			for _, robot := range robots {
				robot.move(num_rows, num_columns, move)
				if _, seen := locations[robot.loc]; seen {
					locations[robot.loc] += 1
				} else {
					locations[robot.loc] = 1
				}
			}
			printRobots(locations, num_rows, num_columns)
			fmt.Println(move)
			fmt.Scanln()
		}
	}
}

func part_2_alt(input string) {
	defer timeTrack(time.Now(), "part_2")
	num_rows := 103
	num_columns := 101

	robots := parse_robots(input)

	for move := 1; move > 0; move++ {
		locations := make(map[location]int)
		for _, robot := range robots {
			robot.move(num_rows, num_columns, move)
			if _, seen := locations[robot.loc]; seen {
				locations[robot.loc] += 1
			} else {
				locations[robot.loc] = 1
			}
		}
		rle := rleRobots(locations, num_rows, num_columns)
		rev_ent := reverse_entropy(rle)

		if rev_ent > 1.3 {
			printRobots(locations, num_rows, num_columns)
			fmt.Println(move, rev_ent)
			fmt.Scanln()
		}

	}
}

func main() {
	example, err := os.ReadFile("puzzle_14/example.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example), 7, 11)

	input, err := os.ReadFile("puzzle_14/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(input), 103, 101)
	//part_2(string(input))
	part_2_alt(string(input))
}
