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

func get_secret_number(number int) (int, int) {

	secret := ((number * 64) ^ number) % 16777216
	secret = ((secret / 32) ^ secret) % 16777216
	secret = ((secret * 2048) ^ secret) % 16777216

	return secret, secret % 10
}

func part_1(input string) {
	defer timeTrack(time.Now(), "part_1")
	lines := strings.Split(input, "\n")

	answer := 0
	for _, line := range lines {
		secret_number, _ := strconv.Atoi(line)
		for i := 0; i < 2000; i++ {
			secret_number, _ = get_secret_number(secret_number)
		}
		answer += secret_number
	}
	fmt.Println(answer)
}

type change struct {
	a int
	b int
	c int
	d int
}

func get_part_2_answer(all_changes map[change]struct{}, best_changes_by_line map[int]map[change]int) int {
	defer timeTrack(time.Now(), "part_2_final_answer")
	answer := 0
	for change, _ := range all_changes {
		score := 0
		for _, line_best := range best_changes_by_line {
			if price, seen := line_best[change]; seen {
				score += price
			}
		}
		if score > answer {
			//fmt.Println(change, score)
			answer = score
		}
	}
	return answer
}

func part_2(input string) {
	defer timeTrack(time.Now(), "part_2")
	lines := strings.Split(input, "\n")

	best_changes_by_line := make(map[int]map[change]int)
	all_changes := make(map[change]struct{})
	for line_index, line := range lines {
		line_best := make(map[change]int)
		changes := []int{}
		secret_number, _ := strconv.Atoi(line)
		price := secret_number % 10
		for i := 0; i < 2000; i++ {
			new_secret_number, new_price := get_secret_number(secret_number)

			changes = append(changes, new_price-price)

			secret_number = new_secret_number
			price = new_price

			// Not enough changes yet
			if i < 3 {
				continue
			}
			change := change{
				a: changes[i],
				b: changes[i-1],
				c: changes[i-2],
				d: changes[i-3],
			}
			// Only record first occurrence of this sequence of changes
			if _, seen := line_best[change]; seen {
				continue
			}

			line_best[change] = new_price
			all_changes[change] = struct{}{}
		}

		best_changes_by_line[line_index] = line_best
	}

	answer := get_part_2_answer(all_changes, best_changes_by_line)
	fmt.Println(answer)
}

func main() {
	example_1, err := os.ReadFile("puzzle_22/example_1.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example_1))

	example_2, err := os.ReadFile("puzzle_22/example_2.txt")
	if err != nil {
		panic(err)
	}
	part_2(string(example_2))

	input, err := os.ReadFile("puzzle_22/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(input))
	part_2(string(input))
}
