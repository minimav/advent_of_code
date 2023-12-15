
using System.IO;
using System.Collections;
using System.Collections.Specialized;

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

    private static void part2(string input)
    {
        OrderedDictionary[] boxes = new OrderedDictionary[256];
        for (int i = 0; i < boxes.Length; i++)
        {
            boxes[i] = new OrderedDictionary();
        }
        
        string[] lines = input.Split(",");
        foreach (string line in lines)
        {
            string label = "";
            if (line.Contains("-"))
            {
                label = line.Substring(0, line.Length - 1);
                int boxIndex = getHash(label);
                OrderedDictionary box = boxes[boxIndex];

                // Remove label from box
                if (box.Contains(label)) {
                    box.Remove(label);
                }
            } else
            {
                label = line.Substring(0, line.Length - 2);
                int boxIndex = getHash(label);
                OrderedDictionary box = boxes[boxIndex];

                // Remove '0' ASCII code to parse char into integer!?
                int focalLength = line[line.Length - 1] - '0';
                
                // If already in box, replace with new focal length, otherwise
                // add at the end
                if (box.Contains(label))
                {
                    box[label] = focalLength;
                }
                else
                {
                    box.Add(label, focalLength);
                }
            }            
        }
        
        int answer = 0;
        for (int boxIndex = 0; boxIndex < boxes.Count(); boxIndex++)
        {
            var box = boxes[boxIndex];
            int lensIndex = 0;
            foreach (DictionaryEntry entry in box)
            {
                var focalLength = (int)entry.Value;
                answer += (boxIndex + 1) * (lensIndex + 1) * focalLength;
                lensIndex++;
            }
        }

        System.Console.WriteLine(answer);
    }

    public static void Main(string[] args)
    {
        string example = File.ReadAllText(@"puzzle_15/example.txt");
        part1(example);
        part2(example);

        string input = File.ReadAllText(@"puzzle_15/input.txt");
        part1(input);
        part2(input);
    }
}

Puzzle.Main([])
