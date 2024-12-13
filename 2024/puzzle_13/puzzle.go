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

func part_1(input string) {
	defer timeTrack(time.Now(), "part_1")
	lines := strings.Split(input, "\n")

	num_claws := len(lines)/4 + 1
	x, _ := regexp.Compile(`X\+([0-9]+)`)
	y, _ := regexp.Compile(`Y\+([0-9]+)`)
	aim_x, _ := regexp.Compile(`X=([0-9]+)`)
	aim_y, _ := regexp.Compile(`Y=([0-9]+)`)

	total := 0
	for claw_index := 0; claw_index < num_claws; claw_index++ {
		claw_lines := lines[claw_index*4 : claw_index*4+3]
		x1_matches := x.FindStringSubmatch(claw_lines[0])
		y1_matches := y.FindStringSubmatch(claw_lines[0])
		x2_matches := x.FindStringSubmatch(claw_lines[1])
		y2_matches := y.FindStringSubmatch(claw_lines[1])
		aim_x_matches := aim_x.FindStringSubmatch(claw_lines[2])
		aim_y_matches := aim_y.FindStringSubmatch(claw_lines[2])

		a, _ := strconv.Atoi(x1_matches[1])
		b, _ := strconv.Atoi(x2_matches[1])
		c, _ := strconv.Atoi(y1_matches[1])
		d, _ := strconv.Atoi(y2_matches[1])

		ax, _ := strconv.Atoi(aim_x_matches[1])
		ay, _ := strconv.Atoi(aim_y_matches[1])

		denom := a*d - b*c
		solution_a := (d*ax - b*ay) / denom
		solution_b := (-c*ax + a*ay) / denom

		if solution_a*a+solution_b*b != ax || solution_a*c+solution_b*d != ay {
			continue
		} else if solution_a > 100 || solution_b > 100 {
			continue
		} else if solution_a < 0 || solution_b < 0 {
			continue
		}
		total += solution_a*3 + solution_b
	}
	fmt.Println(total)
}

func part_2(input string) {
	defer timeTrack(time.Now(), "part_2")
	lines := strings.Split(input, "\n")

	num_claws := len(lines)/4 + 1
	x, _ := regexp.Compile(`X\+([0-9]+)`)
	y, _ := regexp.Compile(`Y\+([0-9]+)`)
	aim_x, _ := regexp.Compile(`X=([0-9]+)`)
	aim_y, _ := regexp.Compile(`Y=([0-9]+)`)

	total := 0
	for claw_index := 0; claw_index < num_claws; claw_index++ {
		claw_lines := lines[claw_index*4 : claw_index*4+3]
		x1_matches := x.FindStringSubmatch(claw_lines[0])
		y1_matches := y.FindStringSubmatch(claw_lines[0])
		x2_matches := x.FindStringSubmatch(claw_lines[1])
		y2_matches := y.FindStringSubmatch(claw_lines[1])
		aim_x_matches := aim_x.FindStringSubmatch(claw_lines[2])
		aim_y_matches := aim_y.FindStringSubmatch(claw_lines[2])

		a, _ := strconv.Atoi(x1_matches[1])
		b, _ := strconv.Atoi(x2_matches[1])
		c, _ := strconv.Atoi(y1_matches[1])
		d, _ := strconv.Atoi(y2_matches[1])

		ax, _ := strconv.Atoi(aim_x_matches[1])
		ay, _ := strconv.Atoi(aim_y_matches[1])
		ax += 10000000000000
		ay += 10000000000000

		denom := a*d - b*c
		solution_a := (d*ax - b*ay) / denom
		solution_b := (-c*ax + a*ay) / denom

		if solution_a*a+solution_b*b != ax || solution_a*c+solution_b*d != ay {
			continue
		} else if solution_a < 0 || solution_b < 0 {
			continue
		}
		total += solution_a*3 + solution_b
	}
	fmt.Println(total)
}

func main() {
	example, err := os.ReadFile("puzzle_13/example.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example))
	part_2(string(example))

	input, err := os.ReadFile("puzzle_13/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(input))
	part_2(string(input))
}
