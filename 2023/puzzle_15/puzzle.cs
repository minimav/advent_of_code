
using System.IO;

class Puzzle {   

    private static int getHash(string line)
    {
        int hash = 0;
        for (int i = 0; i < line.Length; i++)
        {
            int ansiCode = (int) line[i];
            hash += ansiCode;
            hash *= 17;
            hash = hash % 256;
        }
        return hash;
    }

    private static void part1(string input)
    {
        int answer = 0;
        string[] lines = input.Split(",");
        foreach (string line in lines)
        {
            answer += getHash(line);
        }
        System.Console.WriteLine(answer);
    }

    public static void Main(string[] args)
    {
        string example = File.ReadAllText(@"puzzle_15/example.txt");
        part1(example);

        string input = File.ReadAllText(@"puzzle_15/input.txt");
        part1(input);
    }
}

Puzzle.Main([])
