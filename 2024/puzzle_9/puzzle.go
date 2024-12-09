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

type map_key struct {
	value      int
	repeats    int
	index      int
	free_space bool
}

type disk_map struct {
	keys []map_key
}

func (d disk_map) next_gap() (int, map_key, bool) {
	for i, key := range d.keys {
		if key.free_space {
			return i, key, true
		}
	}
	return 0, d.keys[0], false
}

func (d disk_map) print_disk_map() {
	for _, key := range d.keys {
		if key.free_space {
			fmt.Print(strings.Repeat(".", key.repeats))
		} else {
			fmt.Print(strings.Repeat(fmt.Sprintf("%d", key.value), key.repeats))
		}
	}
	fmt.Print("\n")
}

func (d disk_map) check_sum() int {
	output := 0
	for _, key := range d.keys {
		if key.free_space {
			continue
		}
		for i := key.index; i < key.index+key.repeats; i++ {
			output += i * key.value
		}
	}
	return output
}

func parse_disk_map(input string) disk_map {
	line := strings.Split(input, "\n")[0]
	keys := []map_key{}
	disk_index := 0
	for char_index, char := range line {
		repeats, _ := strconv.Atoi(string(char))
		free_space := char_index%2 != 0
		value := char_index / 2
		keys = append(
			keys,
			map_key{
				value,
				repeats,
				disk_index,
				free_space,
			},
		)
		disk_index += repeats
	}
	return disk_map{keys}
}

func process_disk_map(disk_map disk_map) disk_map {
	disk_map.print_disk_map()

	var gap_exists bool
	var current_gap map_key
	var current_index int
	current_index, current_gap, gap_exists = disk_map.next_gap()
	for gap_exists {
		last_folder := &disk_map.keys[len(disk_map.keys)-1]
		if last_folder.free_space {
			disk_map.keys = disk_map.keys[:len(disk_map.keys)-1]
			current_index, current_gap, gap_exists = disk_map.next_gap()
			continue
		}
		fmt.Println(current_gap, last_folder)

		if last_folder.repeats < current_gap.repeats {
			new_folder := map_key{
				value:      last_folder.value,
				index:      current_gap.index,
				repeats:    last_folder.repeats,
				free_space: false,
			}
			current_gap.index += last_folder.repeats
			current_gap.repeats -= last_folder.repeats
			// Add new folder, keep smaller gap and remove last folder
			new_first_keys := append(
				disk_map.keys[:current_index],
				new_folder,
				current_gap,
			)
			fmt.Println(new_first_keys)
			fmt.Println(disk_map.keys[current_index+1])
			disk_map.keys = append(
				new_first_keys,
				disk_map.keys[current_index+1:len(disk_map.keys)-1]...,
			)
		} else if last_folder.repeats == current_gap.repeats {
			new_folder := map_key{
				value:      last_folder.value,
				index:      current_gap.index,
				repeats:    last_folder.repeats,
				free_space: false,
			}
			// Remove folder itself from the end and insert, plus remove gap
			new_first_keys := append(
				disk_map.keys[:current_index],
				new_folder,
			)
			disk_map.keys = append(
				new_first_keys,
				disk_map.keys[current_index+1:len(disk_map.keys)-1]...,
			)
		} else {
			new_folder := map_key{
				value:      last_folder.value,
				index:      current_gap.index,
				repeats:    current_gap.repeats,
				free_space: false,
			}
			last_folder.repeats -= current_gap.repeats
			// Remove gap only
			new_first_keys := append(
				disk_map.keys[:current_index],
				new_folder,
			)
			disk_map.keys = append(
				new_first_keys,
				disk_map.keys[current_index+1:]...,
			)
		}
		disk_map.print_disk_map()
		current_index, current_gap, gap_exists = disk_map.next_gap()
	}
	disk_map.print_disk_map()
	return disk_map
}

func part_1(input string) {
	defer timeTrack(time.Now(), "part_1")
	disk_map := parse_disk_map(input)
	disk_map = process_disk_map(disk_map)
	fmt.Println(disk_map.check_sum())
}

func part_2(input string) {
	defer timeTrack(time.Now(), "part_2")
}

func main() {
	test_disk_map := parse_disk_map("12345")
	test_disk_map = process_disk_map(test_disk_map)
	fmt.Println(test_disk_map.check_sum())

	example, err := os.ReadFile("puzzle_9/example.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example))
	part_2(string(example))

	input, err := os.ReadFile("puzzle_9/input.txt")
	if err != nil {
		panic(err)
	}
	//part_1(string(input))
	part_2(string(input))
}
