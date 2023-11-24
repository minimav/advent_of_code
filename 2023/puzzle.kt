import java.io.*

fun readFile(fileName: String): String 
  = File(fileName).readText(Charsets.UTF_8)

fun puzzle(input: String) {
    println(input)
}

fun main(args : Array<String>) {
    puzzle(readFile("example.txt"))
    puzzle(readFile("input.txt"))
}