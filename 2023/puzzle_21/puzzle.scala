object PuzzleScala {

    def puzzle(input: String) = {
        println(input)
    }

    def main(args: Array[String]) = {
        val example = scala.io.Source.fromFile("puzzle_21/example.txt").mkString
        puzzle(example)
        val input = scala.io.Source.fromFile("puzzle_21/input.txt").mkString
        puzzle(input)
    }
}