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

func part_2(input string) {
	defer timeTrack(time.Now(), "part_2")
	lines := strings.Split(input, "\n")

	change_counts := make(map[change]int)
	for _, line := range lines {
		seen_change := make(map[change]struct{})
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
			if _, seen := seen_change[change]; seen {
				continue
			}

			// Otherwise add to the change's running total
			if _, seen := change_counts[change]; seen {
				change_counts[change] += price
			} else {
				change_counts[change] = price
			}
			seen_change[change] = struct{}{}
		}
	}
	answer := 0
	for _, v := range change_counts {
		if v > answer {
			answer = v
		}
	}

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
