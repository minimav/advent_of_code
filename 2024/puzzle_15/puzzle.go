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

func (l location) left() location {
	return location{
		row: l.row, column: l.column - 1,
	}
}

func (l location) right() location {
	return location{
		row: l.row, column: l.column + 1,
	}
}

func (l location) up() location {
	return location{
		row: l.row - 1, column: l.column,
	}
}

func (l location) down() location {
	return location{
		row: l.row + 1, column: l.column,
	}
}

type grid struct {
	walls       map[location]struct{}
	boxes       map[location]struct{}
	robot       location
	num_rows    int
	num_columns int
}

func (g grid) print() {
	for row := 0; row < g.num_rows; row++ {
		for column := 0; column < g.num_columns; column++ {
			loc := location{row: row, column: column}
			if _, is_wall := g.walls[loc]; is_wall {
				fmt.Print("#")
			} else if _, is_box := g.boxes[loc]; is_box {
				fmt.Print("O")
			} else if loc == g.robot {
				fmt.Print("@")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Print("\n")
	}
}

func part_1(input string) {
	defer timeTrack(time.Now(), "part_1")
	lines := strings.Split(input, "\n")

	walls := make(map[location]struct{})
	boxes := make(map[location]struct{})
	max_row := 0
	var robot location
	commands := ""
	for row, line := range lines {
		if len(line) == 0 {
			continue
		} else if string(line[0]) != "#" {
			commands += string(line)
		}

		for column, rune := range line {
			char := string(rune)
			loc := location{row: row, column: column}
			if char == "#" {
				walls[loc] = struct{}{}
				max_row = row + 1
			} else if char == "@" {
				robot = loc
			} else if char == "O" {
				boxes[loc] = struct{}{}
			}
		}
	}

	grid := grid{
		walls:       walls,
		boxes:       boxes,
		robot:       robot,
		num_rows:    max_row,
		num_columns: len(lines[0]),
	}
	//grid.print()

	for _, command := range commands {
		c := string(command)
		var move location
		move = grid.robot.get_move(c)

		if _, into_wall := grid.walls[move]; into_wall {
			//fmt.Printf("After %s (into wall):\n", c)
			//grid.print()
			continue
		}
		_, into_box := grid.boxes[move]
		if !into_box {
			// Into space case
			grid.robot = move
			//fmt.Printf("After %s (into space):\n", c)
			//grid.print()
			continue
		}

		// Possible box moving case
		boxes_to_move := []location{move}
		space_to_move := false
		for {
			move = move.get_move(c)

			if _, into_wall := grid.walls[move]; into_wall {
				space_to_move = false
				break
			}
			_, into_box := grid.boxes[move]
			if !into_box {
				space_to_move = true
				break
			}
			boxes_to_move = append(boxes_to_move, move)
		}
		if !space_to_move {
			continue
		} else {
			// Only need to move the first (robot) and last in the chain of
			// boxes, the rest will still be in the boxes lookup
			delete(grid.boxes, boxes_to_move[0])
			grid.robot = boxes_to_move[0]
			//fmt.Println("Moving robot to ", boxes_to_move[0])
			last_box_to_move := boxes_to_move[len(boxes_to_move)-1]
			if c == "^" {
				grid.boxes[last_box_to_move.up()] = struct{}{}
			} else if c == ">" {
				grid.boxes[last_box_to_move.right()] = struct{}{}
			} else if c == "<" {
				grid.boxes[last_box_to_move.left()] = struct{}{}
			} else {
				grid.boxes[last_box_to_move.down()] = struct{}{}
			}

		}
		//fmt.Printf("After %s (box move=%t):\n", c, space_to_move)
		//grid.print()
	}

	answer := 0
	for box, _ := range boxes {
		answer += box.row*100 + box.column
	}
	fmt.Println(answer)
}

type grid_part_2 struct {
	walls       map[location]struct{}
	boxes       map[location]string
	robot       location
	num_rows    int
	num_columns int
}

func (g grid_part_2) print() {
	for row := 0; row < g.num_rows; row++ {
		for column := 0; column < g.num_columns; column++ {
			loc := location{row, column}
			if _, is_wall := g.walls[loc]; is_wall {
				fmt.Print("#")
			} else if side, is_box := g.boxes[loc]; is_box && side == "left" {
				fmt.Print("[")
			} else if side, is_box := g.boxes[loc]; is_box && side == "right" {
				fmt.Print("]")
			} else if loc == g.robot {
				fmt.Print("@")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Print("\n")
	}
	fmt.Print("\n")
}

func (l location) get_move(c string) location {
	if c == "^" {
		return l.up()
	} else if c == ">" {
		return l.right()
	} else if c == "<" {
		return l.left()
	} else {
		return l.down()
	}
}

func part_2(input string) {
	defer timeTrack(time.Now(), "part_2")
	lines := strings.Split(input, "\n")

	walls := make(map[location]struct{})
	boxes := make(map[location]string)
	max_row := 0
	var robot location
	commands := ""
	for row, line := range lines {
		if len(line) == 0 {
			continue
		} else if string(line[0]) != "#" {
			commands += string(line)
		}

		for column, rune := range line {
			char := string(rune)
			if char == "#" {
				walls[location{row: row, column: column * 2}] = struct{}{}
				walls[location{row: row, column: column*2 + 1}] = struct{}{}
				max_row = row + 1
			} else if char == "@" {
				robot = location{row: row, column: column * 2}
			} else if char == "O" {
				boxes[location{row: row, column: column * 2}] = "left"
				boxes[location{row: row, column: column*2 + 1}] = "right"
			}
		}
	}

	grid := grid_part_2{
		walls:       walls,
		boxes:       boxes,
		robot:       robot,
		num_rows:    max_row,
		num_columns: len(lines[0]) * 2,
	}
	//grid.print()

	for _, command := range commands {
		c := string(command)
		var move location
		move = grid.robot.get_move(c)

		if _, into_wall := grid.walls[move]; into_wall {
			//fmt.Printf("After %s (into wall):\n", c)
			//grid.print()
			continue
		}

		if _, into_box := grid.boxes[move]; !into_box {
			// Into space case
			grid.robot = move
			//fmt.Printf("After %s (into space):\n", c)
			//grid.print()
			continue
		}

		// Possible box moving case - need to deal with cases like the following
		// when moving up (or equivalent down case, cannot happen sideways due
		// to only doubling the map width!):
		//    [][]
		//     []
		//     @
		// Can still delete the first box, but everything needs updating as
		// no longer shifting in a line.
		if c == ">" || c == "<" {
			boxes_to_move := []location{}
			space_to_move := false
			for {
				if _, into_wall := grid.walls[move]; into_wall {
					space_to_move = false
					break
				}
				_, into_box := grid.boxes[move]
				if !into_box {
					space_to_move = true
					break
				}
				boxes_to_move = append(boxes_to_move, move)
				move = move.get_move(c)
			}
			if !space_to_move {
				continue
			} else {
				// Remove first box, move robot there
				delete(grid.boxes, boxes_to_move[0])
				grid.robot = boxes_to_move[0]

				// Change side of each box in the chain
				for _, box := range boxes_to_move[1:] {
					current_side, _ := grid.boxes[box]
					switched_side := "right"
					if current_side == "right" {
						switched_side = "left"
					}
					grid.boxes[box] = switched_side

				}

				// Last box case, assign side depending on movement
				// direction
				side := "right"
				if c == "<" {
					side = "left"
				}
				last_box := boxes_to_move[len(boxes_to_move)-1]
				grid.boxes[last_box.get_move(c)] = side
			}
			//fmt.Printf("After %s (box move=%t):\n", c, space_to_move)
			//grid.print()
		} else {
			boxes_to_move := []map[location]struct{}{}

			current_row := map[location]struct{}{
				move: struct{}{},
			}
			if side, _ := grid.boxes[move]; side == "left" {
				current_row[move.right()] = struct{}{}
			} else {
				current_row[move.left()] = struct{}{}
			}
			boxes_to_move = append(boxes_to_move, current_row)
			space_to_move := false
		move_checks:
			for {
				next_row := make(map[location]struct{})
				all_spaces := true
				for box, _ := range current_row {
					// Check directly above or below
					move = box.get_move(c)
					if _, into_wall := grid.walls[move]; into_wall {
						break move_checks
					}
					_, into_box := grid.boxes[move]
					if !into_box {
						continue
					}

					// Will need to check another row as there is at least one
					// box in the way (assuming something else won't be blocked)
					// by a wall
					all_spaces = false

					// Make sure both sides of the box above would be there
					next_row[move] = struct{}{}
					if side, _ := grid.boxes[move]; side == "left" {
						next_row[move.right()] = struct{}{}
					} else {
						next_row[move.left()] = struct{}{}
					}
				}
				if all_spaces {
					space_to_move = true
					break
				}
				boxes_to_move = append(boxes_to_move, next_row)
				current_row = next_row
			}

			if !space_to_move {
				//fmt.Printf("After %s (box move=%t):\n", c, space_to_move)
				//grid.print()
				continue
			}

			// Remove first box, move robot there
			grid.robot = grid.robot.get_move(c)

			// Move boxes, side will always stay the same
			// Do this in reverse to avoid overwriting side information we need
			for i := range boxes_to_move {
				box_row := boxes_to_move[len(boxes_to_move)-i-1]
				for box, _ := range box_row {
					side, _ := grid.boxes[box]
					delete(grid.boxes, box)
					grid.boxes[box.get_move(c)] = side
				}
			}
			//fmt.Printf("After %s (box move=%t):\n", c, space_to_move)
			//grid.print()
		}
	}

	answer := 0
	for box, side := range boxes {
		if side == "right" {
			continue
		}
		answer += box.row*100 + box.column
	}
	fmt.Println(answer)
}

func main() {
	for i := 2; i <= 2; i++ {
		example, err := os.ReadFile(fmt.Sprintf("puzzle_15/example_%d.txt", i))
		if err != nil {
			panic(err)
		}
		part_1(string(example))
		part_2(string(example))
	}

	input, err := os.ReadFile("puzzle_15/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(input))
	part_2(string(input))
}
