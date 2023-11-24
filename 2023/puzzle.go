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
    example, err := os.ReadFile("example.txt")
    check(err)
    puzzle(string(example))

    input, err := os.ReadFile("input.txt")
    check(err)
    puzzle(string(input))
}
