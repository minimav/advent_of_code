object PuzzleScala {

    def puzzle(input: String) = {
        println(input)
    }

    def main(args: Array[String]) = {
        val example = scala.io.Source.fromFile("example.txt").mkString
        puzzle(example)
        val input = scala.io.Source.fromFile("input.txt").mkString
        puzzle(input)
    }
}