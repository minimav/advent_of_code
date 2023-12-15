import "io" for File

class StringHelper {
    static replaceAt(str, index, replacement) {
        var result = ""
        for (i in 0...str.count) {
            if (i == index) {
                result = result + replacement
            } else {
                result = result + str[i]
            }
        }
        return result
    }
}

class Grid {
    grid { _grid }

    grid=(value) { _grid = value }
    
    construct new(grid) {
        this.grid = grid
    }
  
    [index] {
        return this.grid[index]
    }
    [x, y] {
        return this.grid[x][y]
    }

    numRows() {
        return this.grid.count
    }

    numColumns() {
        return this.grid[0].count
    }

    toString() {
        var output = ""
        for (row in 0..this.numRows() - 1) {
            output = output + this[row] + "\n"
        }
        return output
    }

    getScore() {
        var score = 0
        for (row in 0..this.numRows() - 1) {
            for (column in 0..this.numColumns() - 1) {
                if (this[row][column] == "O") {
                    score = score + (this.numRows() - row)
                }
            }
        }
        return score
    }

    rotate() {
        // Clockwise rotation
        var rotatedGrid = []
        for (column in 0..this.numColumns() - 1) {
            var nextRow = ""
            for (row in 0..this.numRows() - 1) {
                nextRow = nextRow + this[this.numRows() - row - 1][column]
            }
            rotatedGrid.add(nextRow)
        }
        return Grid.new(rotatedGrid)
    }

    tilt() {
        // First create a copy of the grid to mutate
        var tiltedGrid = []
        for (row in this.grid) {
            tiltedGrid.add(row)
        }
        for (column in 0..this.numColumns() - 1) {
            for (row in 0..this.numRows() - 1) {
                if (tiltedGrid[row][column] != "O") {
                    continue
                } else {
                    var newRow = row
                    while ((newRow >= 1) && (tiltedGrid[newRow - 1][column] == ".")) {
                        newRow = newRow - 1
                    }
                    if (newRow != row) {
                        tiltedGrid[newRow] = StringHelper.replaceAt(tiltedGrid[newRow], column, "O")
                        tiltedGrid[row] = StringHelper.replaceAt(tiltedGrid[row], column, ".")
                    }
                } 
            }
        }
        return Grid.new(tiltedGrid)

    }

    cycle() {
        var grid = this
        for (index in 0..3) {
            grid = grid.tilt().rotate()
        }
        return grid
    }

    spinCycle(n) {
        var cache = Map.new()
        var grid = this
        for (index in 1..n + 1) {
            // Cycle so that index reflects number of completed cycles
            grid = grid.cycle()
            
            var key = grid.toString()
            if (cache[key] != null) {
                var cycleLength = index - cache[key]
                var remainingCycles = (n - cache[key]) % cycleLength
                for (remainingIndex in 0..remainingCycles - 1) {
                    grid = grid.cycle()
                }
                return grid
            } else {
                cache[key] = index
            } 
        }
    }
}

var example = Grid.new(File.read("puzzle_14/example.txt").split("\n"))
var input = Grid.new(File.read("puzzle_14/input.txt").split("\n"))

System.print(example.tilt().getScore())
System.print(input.tilt().getScore())

System.print(example.spinCycle(1000000000).getScore())
// Very slow...
System.print(input.spinCycle(1000000000).getScore())

