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

type location struct {
	row    int
	column int
}

type route struct {
	location location
	visited  map[location]struct{}
}

func parse_grid(lines []string) (map[location]int, []location) {
	grid := make(map[location]int)
	zeros := []location{}
	for row, line := range lines {
		for column, char := range line {
			loc := location{row, column}
			height, _ := strconv.Atoi(string(char))
			grid[loc] = height
			if height == 0 {
				zeros = append(zeros, loc)
			}
		}
	}
	return grid, zeros
}

func part_1(input string) {
	defer timeTrack(time.Now(), "part_1")
	lines := strings.Split(input, "\n")
	num_rows := len(lines)
	num_columns := len(lines[0])
	grid, zeros := parse_grid(lines)

	trailhead_sum := 0
	for _, zero := range zeros {
		summits := make(map[location]struct{})
		start := route{
			location: zero,
			visited:  make(map[location]struct{}),
		}
		queue := []route{start}
		for len(queue) > 0 {
			current_route := queue[0]
			queue = queue[1:]
			_, seen_before := current_route.visited[current_route.location]
			current_height := grid[current_route.location]
			if seen_before {
				continue
			} else if current_height == 9 {
				summits[current_route.location] = struct{}{}
				continue
			}
			current_route.visited[current_route.location] = struct{}{}

			moves := []location{
				{row: current_route.location.row, column: current_route.location.column - 1},
				{row: current_route.location.row, column: current_route.location.column + 1},
				{row: current_route.location.row - 1, column: current_route.location.column},
				{row: current_route.location.row + 1, column: current_route.location.column},
			}
			for _, move := range moves {
				if move.row < 0 || move.column < 0 || move.row >= num_rows || move.column >= num_columns {
					continue
				}
				next_height := grid[move]
				if next_height != current_height+1 {
					continue
				}
				new_route := route{
					location: move,
					visited:  current_route.visited,
				}
				queue = append(queue, new_route)
			}
		}
		trailhead_sum += len(summits)
	}
	fmt.Println(trailhead_sum)
}

func part_2(input string) {
	defer timeTrack(time.Now(), "part_2")
	lines := strings.Split(input, "\n")
	num_rows := len(lines)
	num_columns := len(lines[0])
	grid, zeros := parse_grid(lines)

	trailhead_rating := 0
	for _, zero := range zeros {
		num_summits := 0
		start := route{
			location: zero,
			visited:  make(map[location]struct{}),
		}
		queue := []route{start}
		for len(queue) > 0 {
			current_route := queue[0]
			queue = queue[1:]
			_, seen_before := current_route.visited[current_route.location]
			current_height := grid[current_route.location]
			if seen_before {
				continue
			} else if current_height == 9 {
				num_summits += 1
				continue
			}
			current_route.visited[current_route.location] = struct{}{}

			moves := []location{
				{row: current_route.location.row, column: current_route.location.column - 1},
				{row: current_route.location.row, column: current_route.location.column + 1},
				{row: current_route.location.row - 1, column: current_route.location.column},
				{row: current_route.location.row + 1, column: current_route.location.column},
			}
			for _, move := range moves {
				if move.row < 0 || move.column < 0 || move.row >= num_rows || move.column >= num_columns {
					continue
				}
				next_height := grid[move]
				if next_height != current_height+1 {
					continue
				}

				visited_copy := make(map[location]struct{})
				for k, v := range current_route.visited {
					visited_copy[k] = v
				}
				new_route := route{
					location: move,
					visited:  visited_copy,
				}
				queue = append(queue, new_route)
			}
		}
		trailhead_rating += num_summits
	}
	fmt.Println(trailhead_rating)
}

func main() {
	example, err := os.ReadFile("puzzle_10/example.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example))
	part_2(string(example))

	input, err := os.ReadFile("puzzle_10/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(input))
	part_2(string(input))
}
