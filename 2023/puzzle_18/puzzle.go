package main

import (
    "fmt"
    "os"
)

func check(e error) {
    if e != nil {
        panic(e)
    }
}

func puzzle(input string) {
    fmt.Println(input)
}

func main() {
    example, err := os.ReadFile("puzzle_18/example.txt")
    check(err)
    puzzle(string(example))

    input, err := os.ReadFile("puzzle_18/input.txt")
    check(err)
    puzzle(string(input))
}
