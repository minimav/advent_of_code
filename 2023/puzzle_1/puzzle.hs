{-# LANGUAGE OverloadedStrings #-}
import System.IO
import Data.Char (isDigit, digitToInt, isLetter)
import Data.Text(pack, unpack, replace)
import Data.List(findIndex, tails, isPrefixOf)
import Data.Maybe (catMaybes)
import Data.List (isInfixOf, find)


notLetter = not . isLetter
filterDigitChars xs = filter notLetter xs
toDigits xs = map digitToInt xs
fromDigits :: [Int] -> Int = foldl addDigit 0
   where addDigit num d = 10 * num + d
topAndTail xs = [head xs, last xs]

solvePart1 path = do  
        content <- readFile path
        let linesOfFile = lines $ content
        let digitChars = map filterDigitChars linesOfFile
        let allDigits = map toDigits digitChars
        let digits = map topAndTail allDigits
        let rowNumbers = map fromDigits digits
        let answer = sum rowNumbers
        print answer


numberStringToDigit = [("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")]
numberStrings = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

findString :: (Eq a) => [a] -> [a] -> Maybe Int
findString search str = findIndex (isPrefixOf search) (tails str)
finders = map findString numberStrings

--for a list of tuples, find the first element of the tuple whose second element is the minimum
-- minTuple :: (Ord b) => [(a, b)] -> a
-- minTuple xs = fst $ foldl1 (\acc x -> if snd x < snd acc then x else acc) xs

--applyFuns should take a list of tuples, where the first element of each tuple is a key, and the second element is a list of functions
--it should then apply each function to the input, and return a list of tuples where the first element is the key, and the second element is the result of applying the function
-- applyFuns :: [(c, a->b)] -> a -> [(c, b)]
-- applyFuns xs input = map (\(x, y) -> (x, y input)) xs

-- applyToSecond :: (a -> b) -> [(c, a)] -> [(c, b)]
-- applyToSecond xs func = map (\(x, y) -> (x, func y)) xs


--replace a substring at a particular index
replaceSubstringAtIndex :: Int -> String -> String -> String
replaceSubstringAtIndex start replacement str = (take start str) ++ replacement ++ (drop (start + (length replacement)) str)

--for a string s, find the first substring in s that matches an element of numberStrings
--replace the whole substring with the corresponding digit then return the result
--if no substring matches, return the original string
--replaceNumberStrings :: String -> String
--test cases
--replaceNumberStrings "eightwothree" == "8twothree"
--replaceNumberStrings "ash" == "ash"
--replaceNumberStrings "onetwothree" == "1twothree"
--replaceNumberStrings "1two3" == "123"
--replaceNumberStrings should compile and the above test cases should all hold


solvePart2 path = do  
        content <- readFile path
        let linesOfFile = lines $ content
        print linesOfFile
        let cleanedLinesOfFile = map replaceFirstNumberStrings linesOfFile
        print cleanedLinesOfFile
        let digitChars = map filterDigitChars cleanedLinesOfFile
        let allDigits = map toDigits digitChars
        print allDigits
        let digits = map topAndTail allDigits
        let rowNumbers = map fromDigits digits
        let answer = sum rowNumbers
        print answer

main = do
    let s = replaceFirstNumberStrings "none1eight"
    print s
    --solvePart1 "./example_1_1.txt"
    --solvePart1 "./input_1.txt"
    solvePart2 "./example_1_2.txt"
    --solvePart2 "./input_1.txt"