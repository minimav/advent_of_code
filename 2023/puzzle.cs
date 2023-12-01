
using System.IO;

class Puzzle {   
    private static void puzzle(string input)
    {
        System.Console.WriteLine(input);
    }

    public static void Main(string[] args)
    {
        string example = File.ReadAllText(@"example.txt");
        puzzle(example);

        string input = File.ReadAllText(@"input.txt");
        puzzle(input);
    }
}

Puzzle.Main([])
