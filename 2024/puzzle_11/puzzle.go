// go run puzzle_?/puzzle.go
package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
	"time"
	"unicode/utf8"
)

func timeTrack(start time.Time, name string) {
	elapsed := time.Since(start)
	fmt.Printf("%s took %s\n", name, elapsed)
}

type pair struct {
	left  int
	right int
}

func blinky(input string, num_blinks int) {
	line := strings.Split(input, "\n")[0]
	numbers := strings.Split(line, " ")
	blink := []int{}
	for _, n := range numbers {
		parse_n, _ := strconv.Atoi(n)
		blink = append(blink, parse_n)
	}

	split_cache := make(map[int]pair)

	for i := 0; i < num_blinks; i++ {
		new_blink := []int{}
		for _, n := range blink {
			if n == 0 {
				new_blink = append(new_blink, 1)
			} else if splits, done := split_cache[n]; done {
				new_blink = append(new_blink, splits.left)
				new_blink = append(new_blink, splits.right)
			} else if num_chars := utf8.RuneCountInString(strconv.Itoa(n)); num_chars%2 == 0 {
				half_chars := num_chars / 2
				factor := int(math.Pow(10, float64(half_chars)))
				left := (n - (n % factor)) / factor
				right := n % factor
				new_blink = append(new_blink, left)
				new_blink = append(new_blink, right)
				split_cache[n] = pair{left, right}
			} else {
				new_blink = append(new_blink, n*2024)
			}
		}

		blink = new_blink
	}
	fmt.Println(len(blink))
}

func part_1(input string) {
	defer timeTrack(time.Now(), "part_1")
	blinky(input, 25)
}

type rec_blinker struct {
	n      int
	blinks int
}

var cache = make(map[rec_blinker]int)

func blink(blinker rec_blinker) int {
	answer, done := cache[blinker]
	if done {
		return answer
	} else if blinker.blinks == 0 {
		cache[blinker] = 1
		return 1
	}

	if blinker.n == 0 {
		cache[blinker] = blink(rec_blinker{n: 1, blinks: blinker.blinks - 1})
		return cache[blinker]
	} else if num_chars := utf8.RuneCountInString(strconv.Itoa(blinker.n)); num_chars%2 == 0 {
		half_chars := num_chars / 2
		factor := int(math.Pow(10, float64(half_chars)))
		left := (blinker.n - (blinker.n % factor)) / factor
		right := blinker.n % factor
		cache[blinker] = blink(rec_blinker{n: left, blinks: blinker.blinks - 1}) + blink(rec_blinker{n: right, blinks: blinker.blinks - 1})
		return cache[blinker]
	} else {
		cache[blinker] = blink(rec_blinker{n: blinker.n * 2024, blinks: blinker.blinks - 1})
		return cache[blinker]
	}
}

func part_2(input string) {
	defer timeTrack(time.Now(), "part_2")
	line := strings.Split(input, "\n")[0]
	numbers := strings.Split(line, " ")
	answer := 0
	for _, raw_n := range numbers {
		n, _ := strconv.Atoi(raw_n)
		answer += blink(rec_blinker{n: n, blinks: 75})
	}
	fmt.Println(answer)
}

func main() {
	example, err := os.ReadFile("puzzle_11/example.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example))
	part_2(string(example))

	input, err := os.ReadFile("puzzle_11/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(input))
	part_2(string(input))
}
