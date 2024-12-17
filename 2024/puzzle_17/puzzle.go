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

type registers struct {
	A int
	B int
	C int
}

func join(nums []int) string {
	output := ""
	for i, n := range nums {
		if i < len(nums)-1 {
			output += fmt.Sprintf("%d,", n)
		} else {
			output += fmt.Sprintf("%d", n)
		}
	}
	return output
}

func get_combo_operand(operand int, registers registers) int {
	if operand <= 3 {
		return operand
	} else if operand == 4 {
		return registers.A
	} else if operand == 5 {
		return registers.B
	} else if operand == 6 {
		return registers.C
	} else {
		return 0
	}
}

func int_pow(n int, m int) int {
	if m == 0 {
		return 1
	} else if m == 1 {
		return n
	}

	result := n
	for i := 2; i <= m; i++ {
		result *= n
	}
	return result
}

func bitwise_xor(a int, b int) int {
	return a ^ b
}

func run(registers registers, instructions []int) []int {
	instruction_pointer := 0
	output := []int{}
	for {
		if instruction_pointer >= len(instructions) {
			break
		}
		opcode := instructions[instruction_pointer]
		operand := instructions[instruction_pointer+1]

		if opcode == 0 {
			registers.A /= int_pow(2, get_combo_operand(operand, registers))
		} else if opcode == 1 {
			registers.B = bitwise_xor(registers.B, operand)
		} else if opcode == 2 {
			registers.B = get_combo_operand(operand, registers) % 8
		} else if opcode == 3 && registers.A != 0 {
			instruction_pointer = operand
			continue
		} else if opcode == 4 {
			registers.B = bitwise_xor(registers.B, registers.C)
		} else if opcode == 5 {
			output = append(output, get_combo_operand(operand, registers)%8)
		} else if opcode == 6 {
			registers.B = registers.A / int_pow(2, get_combo_operand(operand, registers))
		} else if opcode == 7 {
			registers.C = registers.A / int_pow(2, get_combo_operand(operand, registers))
		}
		instruction_pointer += 2
	}
	return output
}

func parse(input string) (registers, []int) {
	lines := strings.Split(input, "\n")
	areg, _ := regexp.Compile(`Register A: ([0-9]+)`)
	a_matches := areg.FindStringSubmatch(lines[0])
	a, _ := strconv.Atoi(a_matches[1])

	breg, _ := regexp.Compile(`Register B: ([0-9]+)`)
	b_matches := breg.FindStringSubmatch(lines[1])
	b, _ := strconv.Atoi(b_matches[1])

	creg, _ := regexp.Compile(`Register C: ([0-9]+)`)
	c_matches := creg.FindStringSubmatch(lines[2])
	c, _ := strconv.Atoi(c_matches[1])

	registers := registers{
		A: a,
		B: b,
		C: c,
	}

	instructions := []int{}
	for _, raw_code := range strings.Split(lines[4][9:], ",") {
		code, _ := strconv.Atoi(raw_code)
		instructions = append(instructions, code)
	}
	return registers, instructions
}

func part_1(input string) {
	defer timeTrack(time.Now(), "part_1")
	registers, instructions := parse(input)
	output := run(registers, instructions)
	fmt.Println(join(output))
}

func arr_eq(a []int, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	for i, v_a := range a {
		v_b := b[i]
		if v_a != v_b {
			return false
		}
	}

	return true
}

func part_2(input string) {
	defer timeTrack(time.Now(), "part_2")

	registers, instructions := parse(input)
	for i := 1; i <= 1_000; i++ {
		registers.A = i
		output := run(registers, instructions)
		fmt.Println(i, strconv.FormatInt(int64(i), 2), output)
		if arr_eq(output, instructions) {
			fmt.Println(i)
			break
		}
	}
}

func main() {
	example_1, err := os.ReadFile("puzzle_17/example_1.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example_1))

	example_2, err := os.ReadFile("puzzle_17/example_2.txt")
	if err != nil {
		panic(err)
	}
	part_2(string(example_2))

	input, err := os.ReadFile("puzzle_17/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(input))
	part_2(string(input))
}
