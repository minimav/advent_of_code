package main

import (
    "errors"
    "fmt"
    "math"
    "os"
    "strings"
    "strconv"
)

func check(e error) {
    if e != nil {
        panic(e)
    }
}

func getBounds(outline map[complex128]bool) (float64, float64, float64, float64) {
    minX, minY, maxX, maxY := 0.0, 0.0, 0.0, 0.0
    for point := range outline {
        if real(point) < minX {
            minX = real(point)
        } 
        if real(point) > maxX {
            maxX = real(point)
        }
        if imag(point) < minY {
            minY = imag(point)
        }
        if imag(point) > maxY {
            maxY = imag(point)
        }
    }
    return minX, minY, maxX, maxY
}

func flood(outline map[complex128]bool, initialFlooded map[complex128]bool) (map[complex128]bool, error) {
    flooded := make(map[complex128]bool)
    queue := []complex128{}
    
    // Initialise flooded grid and queue of points to flood from
    for point := range outline {
        flooded[point] = true
    }
    for point := range initialFlooded {
        queue = append(queue, point)
    }

    minX, minY, maxX, maxY := getBounds(outline)

    for len(queue) > 0 {
        point := queue[0]
        queue = queue[1:]
        if flooded[point] || outline[point] {
            //fmt.Println("Skipping", point)
            continue
        }
        //fmt.Println("Adding", point)
        flooded[point] = true

        neighbours := []complex128{
            point + complex(1, 0),
            point + complex(-1, 0),
            point + complex(0, 1),
            point + complex(0, -1),
        }
        should_error := false
        for _, neighbour := range neighbours {
            if real(neighbour) > maxX || real(neighbour) < minX || imag(neighbour) > maxY || imag(neighbour) < minY {
                should_error = true
                continue
            }
            //fmt.Println("Checking", neighbour)
            if _, ok := flooded[neighbour]; !ok {
                //fmt.Println("Queueing", neighbour)
                queue = append(queue, neighbour)
            }
        }
        if should_error {
            return nil, errors.New("Outside boundary detected!")
        }
    }
    return flooded, nil
}

func debugPrint(flooded map[complex128]bool) {
    minX, minY, maxX, maxY := getBounds(flooded)
    rows := int(maxY - minY + 1)
    cols := int(maxX - minX + 1)
    output := make([][]string, rows)
    for i := range output {
        output[i] = make([]string, cols)
        for j := range output[i] {
            output[i][j] = " "
        }
    }
    for point := range flooded {
        x := int(real(point) - minX)
        y := int(imag(point) - minY)
        output[y][x] = "#"
    }
    
    // Flip the output vertically so that it's the right way up
    var lines []string
    for i := len(output) - 1; i >= 0; i-- {
        row := output[i]
        line := strings.Join(row, "")
        lines = append(lines, line)
    }
    result := strings.Join(lines, "\n")
    fmt.Println(result)
}

func floodFill(outline []complex128) map[complex128]bool {
    leftFlood := make(map[complex128]bool)
    rightFlood := make(map[complex128]bool)

    // Initialise both left and right hand 'areas'
    for i := range outline[:len(outline)-1] {
        point := outline[i]
        nextPoint := outline[i+1]
        vector := nextPoint - point
        
        var leftNormal complex128
        if vector == complex(1, 0) {
            leftNormal = complex(0, 1)
        } else if vector == complex(0, 1) {
            leftNormal = complex(-1, 0)
        } else if vector == complex(-1, 0) {
            leftNormal = complex(0, -1)
        } else {
            leftNormal = complex(1, 0)
        }
        leftFlood[point + leftNormal] = true
        leftFlood[nextPoint + leftNormal] = true
        rightFlood[point - leftNormal] = true
        rightFlood[nextPoint - leftNormal] = true
    }

    outlineSet := make(map[complex128]bool)
    for _, point := range outline {
        outlineSet[point] = true
    }
    floodedLeft, leftErr := flood(outlineSet, leftFlood)
    floodedRight, rightErr := flood(outlineSet, rightFlood)
    if leftErr != nil {
        return floodedRight
    } else if rightErr != nil {
        return floodedLeft
    } else {
        panic("Both sides failed to flood!")
    }
}

var vectors = map[string]complex128{
    "R": complex(1, 0),
    "U": complex(0, 1),
    "D": complex(0, -1),
    "L": complex(-1, 0),
}

func part1(input string) {
    lines := strings.Split(input, "\n")
    outline := []complex128{}
    current := complex(0, 0)
    for _, line := range lines {
        parts := strings.Split(line, " ")
        direction := parts[0]
        distance, _ := strconv.Atoi(parts[1])
        vector := vectors[direction]
        for _ = range make([]struct{}, distance) {
            current += vector
            outline = append(outline, current)
        }
    }
    flooded := floodFill(outline)
    //debugPrint(flooded)
    fmt.Println(len(flooded))
}

var hexToDirection = map[string]string{
    "0": "R",
    "1": "D",
    "2": "L",
    "3": "U",
}

func calculateArea(corners []complex128) float64 {
    // Via shoelace formula
    area := 0.0
    for i := 0; i < len(corners) - 1; i++ {
        point := corners[i]
        nextPoint := corners[i+1]
        area += real(point) * imag(nextPoint) - real(nextPoint) * imag(point)
    }
    // Add the final point to the first point
    point := corners[len(corners) - 1]
    nextPoint := corners[0]
    area += real(point) * imag(nextPoint) - real(nextPoint) * imag(point)
    return math.Abs(area / 2)
}

func part2(input string) {
    lines := strings.Split(input, "\n")
    corners := []complex128{}
    current := complex(0, 0)
    corners = append(corners, current)
    boundaryPoints := 0.0
    for _, line := range lines {
        parts := strings.Split(line, " ")
        
        // Get number from (#aaaaaa) format
        hex := strings.Replace(parts[2], "(", "", -1)
        hex = strings.Replace(hex, ")", "", -1)
        hex = hex[1:]
                
        direction := hexToDirection[hex[len(hex) - 1:]]
        distance, _ := strconv.ParseInt(hex[:len(hex) - 1], 16, 64)
        vector := vectors[direction]

        current += vector * complex(float64(distance), 0.0)
        boundaryPoints += float64(distance)
        corners = append(corners, current)
    }

    // Pick's theorem
    area := calculateArea(corners)
    interiorPoints := area + 1 - boundaryPoints / 2
    fmt.Println(interiorPoints + boundaryPoints)
}

func main() {
    example, err := os.ReadFile("puzzle_18/example.txt")
    check(err)
    part1(string(example))
    part2(string(example))

    input, err := os.ReadFile("puzzle_18/input.txt")
    check(err)
    part1(string(input))
    part2(string(input))
}
