import scala.collection.mutable.HashSet

object PuzzleScala {

    def notOutOfBounds(row: Int, col: Int, numRows: Int, numCols: Int): Boolean = {
        if (row < 0 || row >= numRows || col < 0 || col >= numCols) {
            return false
        }
        return true
    }

    def positiveDivisor(a: Int, b: Int): Int = {
        return (a % b + b) % b
    }

    def part1(input: String, stepsBound: Int) = {
        val lines = input.split("\n")
        val numRows = lines.length
        val numCols = lines(0).length
        var start = (0, 0)
        val grid = Array.ofDim[Char](numRows, numCols)
        lines.zipWithIndex.foreach((line, rowIndex) => {
            line.zipWithIndex.foreach { case (c, columnIndex) => 
                grid(rowIndex)(columnIndex) = c
                if (c == 'S') {
                    start = (rowIndex, columnIndex)
                }
            }
        })
        
        var answer = 0
        var queue = List((start, 0))
        val visited = Array.ofDim[Boolean](numRows, numCols)
        visited(start._1)(start._2) = true

        while (queue.nonEmpty) {
            val ((row, col), steps) = queue.head
            queue = queue.tail
            if (steps <= stepsBound) {
                if (steps % 2 == 0) {
                    // Any shorter path from S to another node can be extended
                    // to a 16 length path with redundant back and forths
                    answer += 1
                }
                
                // Up
                if (
                    notOutOfBounds(row - 1, col, numRows, numCols) &&
                    (grid(row - 1)(col) != '#') &&
                    !visited(row - 1)(col)
                ) {
                    queue = queue :+ ((row - 1, col), steps + 1)
                    visited(row - 1)(col) = true
                }

                // Down
                if (
                    notOutOfBounds(row + 1, col, numRows, numCols) &&
                    (grid(row + 1)(col) != '#') &&
                    !visited(row + 1)(col)
                ) {
                    queue = queue :+ ((row + 1, col), steps + 1)
                    visited(row + 1)(col) = true
                }

                // Left
                if (
                    notOutOfBounds(row, col - 1, numRows, numCols) &&
                    (grid(row)(col - 1) != '#') &&
                    !visited(row)(col - 1)
                ) {
                    queue = queue :+ ((row, col - 1), steps + 1)
                    visited(row)(col - 1) = true
                }

                // Right
                if (
                    notOutOfBounds(row, col + 1, numRows, numCols) &&
                    (grid(row)(col + 1) != '#') &&
                    !visited(row)(col + 1)
                ) {
                    queue = queue :+ ((row, col + 1), steps + 1)
                    visited(row)(col + 1) = true
                }
            }
        }
        println(answer)
    }

    def part2(input: String, stepsBound: Int) = {
        val lines = input.split("\n")
        val numRows = lines.length
        val numCols = lines(0).length
        var start = (0, 0)
        val grid = Array.ofDim[Char](numRows, numCols)
        lines.zipWithIndex.foreach((line, rowIndex) => {
            line.zipWithIndex.foreach { case (c, columnIndex) => 
                grid(rowIndex)(columnIndex) = c
                if (c == 'S') {
                    start = (rowIndex, columnIndex)
                }
            }
        })

        var visited = HashSet[(Int, Int)]()
        visited.add(start)
        var steps = 0
        while (steps < stepsBound) {
            var nextVisited = HashSet[(Int, Int)]()
            visited.foreach { case (row, col) =>
                // Up
                if (
                    (grid(positiveDivisor(row - 1, numRows))(positiveDivisor(col, numCols)) != '#')
                ) {
                    nextVisited.add((row - 1, col))
                }

                // Down
                if (
                    (grid(positiveDivisor(row + 1, numRows))(positiveDivisor(col, numCols)) != '#')
                ) {
                    nextVisited.add((row + 1, col))
                }

                // Left
                if (
                    (grid(positiveDivisor(row, numRows))(positiveDivisor(col - 1, numCols)) != '#')
                ) {
                    nextVisited.add((row, col - 1))
                }

                // Right
                if (
                    (grid(positiveDivisor(row, numRows))(positiveDivisor(col + 1, numCols)) != '#')
                ) {
                    nextVisited.add((row, col + 1))
                }
            }
            visited = nextVisited
            steps += 1
        }
        println(visited.size)
    }

    def main(args: Array[String]) = {
        val example = scala.io.Source.fromFile("puzzle_21/example.txt").mkString
        part1(example, 6)
        // part2(example, 6)
        // part2(example, 10)
        // part2(example, 50)
        // part2(example, 100)
        // part2(example, 500)
        // part2(example, 1000)
        val input = scala.io.Source.fromFile("puzzle_21/input.txt").mkString
        part1(input, 65)
        part2(input, 65)
        part2(input, 65 + 131)
        part2(input, 65 + 2 * 131)
        part2(input, 65 + 3 * 131)
        part2(input, 65 + 4 * 131)
    }
}