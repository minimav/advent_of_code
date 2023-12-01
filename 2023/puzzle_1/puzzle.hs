{-# LANGUAGE OverloadedStrings #-}
import System.IO
import Data.Char (digitToInt, isLetter)
import Data.List(tails, isPrefixOf)

notLetter :: Char -> Bool
notLetter = not . isLetter

filterDigitChars :: [Char] -> [Char]
filterDigitChars xs = filter notLetter xs

toDigits :: [Char] -> [Int]
toDigits xs = map digitToInt xs

fromDigits :: [Int] -> Int
fromDigits :: [Int] -> Int = foldl addDigit 0
   where addDigit num d = 10 * num + d

topAndTail :: [a] -> [a]
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

numberStringWithDigit :: [([Char], Int)]
numberStringWithDigit = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)]

-- Determine whether the character at each index in a string is not a letter
indexWithNotALetter :: [Char] -> [(Int, Bool)]
indexWithNotALetter xs = map (\(x, y) -> (x, notLetter y)) (zip [0..] xs)

-- Filter to indexes where the value is not a letter, i.e. a number
whereNotALetter :: [(Int, Bool)] -> [(Int, Bool)]
whereNotALetter xs = filter (\(x, y) -> y == True) xs

-- Get the int digit at a given index in a string
intAtIndex :: Int -> [Char] -> (Int, Int)
intAtIndex index xs = (index, digitToInt (xs !! index))

-- Get the int digit at the given index in a string for multiple indexes
getIntAtIndex :: ([(Int, Bool)], [Char]) -> [(Int, Int)]
getIntAtIndex (indexes, xs) = map (\(x, y) -> intAtIndex x xs) indexes

-- Get the int digit at the min index
intAtMinIndex :: [(Int, Int)] -> Int
intAtMinIndex xs = snd (head (filter (\(x, y) -> x == minIndex) xs))
    where minIndex = minimum (map fst xs)

-- Get the int digit at the max index
intAtMaxIndex :: [(Int, Int)] -> Int
intAtMaxIndex xs = snd (head (filter (\(x, y) -> x == maxIndex) xs))
    where maxIndex = maximum (map fst xs)

-- Concatenate two lists of tuples
concatTuple :: ([(Int, Int)], [(Int, Int)]) -> [(Int, Int)]
concatTuple (xs, ys) = xs ++ ys

-- Convert a pair to a list
pairToList :: (a, a) -> [a]
pairToList (x,y) = [x,y]

-- Cartesian product of two lists
cartProd :: [a] -> [b] -> [(a, b)]
cartProd xs ys = [(x,y) | x <- xs, y <- ys]

-- (("one","1"),(0,"eighthree")) check 
filterSubstrings :: [(([Char], Int), (Int, [Char]))] -> [(([Char], Int), (Int, [Char]))]
filterSubstrings xs = filter (\(x, y) -> isPrefixOf (fst x) (snd y)) xs

-- (("one","1"),(0,"eighthree")) extract index and digit corresponding to substring
extractDigitSubstring :: (([Char], Int), (Int, [Char])) -> (Int, Int)
extractDigitSubstring ((x, y), (z, w)) = (z, y)


solvePart2 path = do  
        content <- readFile path
        let linesOfFile = lines $ content
        
        let indexesNotALetter = map indexWithNotALetter linesOfFile
        -- Get the indexes of the digits in each line
        let indexesWithDigits = map getIntAtIndex (zip (map whereNotALetter indexesNotALetter) linesOfFile)

        -- Get the same format except the start index of number substrings
        let tailsWithIndex = map (zip [0..] . tails) linesOfFile
        let allTailsWithSubstrings = map (cartProd numberStringWithDigit) tailsWithIndex
        let substringMatches = map filterSubstrings allTailsWithSubstrings
        let indexesWithStringDigits = map (map extractDigitSubstring) substringMatches
        
        -- Combine both sets of indexes per line, then get the digit for the first and last indexes
        let finalIndexesWithDigits = map concatTuple (zip indexesWithDigits indexesWithStringDigits)
        let firstAndLast = zip (map intAtMinIndex finalIndexesWithDigits) (map intAtMaxIndex finalIndexesWithDigits)
        let answer = sum (map fromDigits (map pairToList firstAndLast))

        print answer

main = do
    solvePart1 "puzzle_1/example_1_1.txt"
    solvePart1 "puzzle_1/input_1.txt"
    solvePart2 "puzzle_1/example_1_2.txt"
    solvePart2 "puzzle_1/input_1.txt"