object PuzzleScala {

    def notOutOfBounds(row: Int, col: Int, numRows: Int, numCols: Int): Boolean = {
        if (row < 0 || row >= numRows || col < 0 || col >= numCols) {
            return false
        }
        return true
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
        
        //println(grid.map(_.mkString("")).mkString("\n"))
        //println(start)
        
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

    def main(args: Array[String]) = {
        val example = scala.io.Source.fromFile("puzzle_21/example.txt").mkString
        part1(example, 6)
        val input = scala.io.Source.fromFile("puzzle_21/input.txt").mkString
        part1(input, 64)
    }
}