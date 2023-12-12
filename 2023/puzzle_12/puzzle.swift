import Foundation

func readFile(path: String) -> String {
    let path = URL(fileURLWithPath: path)
    do {
        return try! String(contentsOf: path)
    } catch {
        return ""
    }
}

func checkValidSoFar(puzzle: String, springs: [Int]) -> Bool {
    let numHashes = springs.reduce(0, +)
    let hashCount = puzzle.filter { $0 == "#" }.count
    let unknownCount = puzzle.filter { $0 == "?" }.count
    
    if (puzzle.count < numHashes + springs.count - 1) {
        // Must be at least as many characters as springs with gaps in between
        return false
    } else if (numHashes > hashCount + unknownCount) {
        // Must be enough unknowns to fill
        return false
    }
    
    // Zero or more . or ? to start
    var pattern = "[\\.\\?]*"
    for (index, spring) in springs.enumerated() {
        // # for at most this number of springs
        pattern += "[\\#\\?]{0,\(spring)}"
       
        if index != springs.count - 1 {
            // Followed by 1 or more . or ? if not the last spring
            pattern += "[\\.\\?]+"
        } else {
            // Else 0 or more . or ? to the end
            pattern += "[\\.\\?]*"
        }
    }
    pattern = "^" + pattern + "$"
    do {
        let regex = try NSRegularExpression(pattern: pattern, options: [])
        let range = NSRange(location: 0, length: puzzle.utf16.count)
        let matches = regex.matches(in: puzzle, options: [], range: range)
        return matches.count > 0
    } catch {
        return false
    }    
}

assert(checkValidSoFar(puzzle: "...#.#", springs: [1, 1]) == true)
assert(checkValidSoFar(puzzle: ".?.#.#", springs: [1, 1]) == true)
assert(checkValidSoFar(puzzle: ".##?#", springs: [1, 1]) == false)
assert(checkValidSoFar(puzzle: "???#", springs: [1, 1]) == true)
assert(checkValidSoFar(puzzle: "???#", springs: [1, 1, 1]) == false)
assert(checkValidSoFar(puzzle: "#???#", springs: [1, 1, 1]) == true)
assert(checkValidSoFar(puzzle: "???#", springs: [2, 1]) == true)
assert(checkValidSoFar(puzzle: "..?#", springs: [2, 1]) == false)
assert(checkValidSoFar(puzzle: "?#?#?#?#?#?#?#?", springs: [1, 3, 1, 6]) == true)


func checkComplete(puzzle: String, springs: [Int]) -> Int {
    // Zero or more . to start
    var pattern = "\\.*"
    for (index, spring) in springs.enumerated() {
        // # for this number of springs
        pattern += "\\#{\(spring)}"
       
        if index != springs.count - 1 {
            // Followed by 1 or more . if not the last spring
            pattern += "\\.+"
        } else {
            // Else 0 or more . to the end
            pattern += "\\.*"
        }
    }
    do {
        let regex = try NSRegularExpression(pattern: pattern, options: [])
        let range = NSRange(location: 0, length: puzzle.utf16.count)
        let matches = regex.matches(in: puzzle, options: [], range: range)
        return matches.count == 1 ? 1 : 0
    } catch {
        print("invalid regex: \(error.localizedDescription)")
        return 0
    }    
}

assert(checkComplete(puzzle: "...#.#", springs: [1, 1]) == 1)
assert(checkComplete(puzzle: "...#.#", springs: [1, 2]) == 0)
assert(checkComplete(puzzle: "...#.#", springs: [1, 1, 1]) == 0)
assert(checkComplete(puzzle: "###..##.", springs: [3, 2]) == 1)
assert(checkComplete(puzzle: ".###.", springs: [3]) == 1)
assert(checkComplete(puzzle: "###.", springs: [3]) == 1)
assert(checkComplete(puzzle: ".###", springs: [3]) == 1)

func solve(puzzle: String, springs: [Int]) -> Int {
    var numArrangements = 0
    let firstUnknown = puzzle.firstIndex(of: "?")
    if firstUnknown == nil {
        return checkComplete(puzzle: puzzle, springs: springs)
    } else {
        let replaceRange = firstUnknown!..<puzzle.index(after: firstUnknown!)
        let dotReplaced = puzzle.replacingCharacters(in: replaceRange, with: ".")
        if checkValidSoFar(puzzle: dotReplaced, springs: springs) {
            numArrangements += solve(puzzle: dotReplaced, springs: springs)
        }
        
        let hashReplaced = puzzle.replacingCharacters(in: replaceRange, with: "#")
        if checkValidSoFar(puzzle: hashReplaced, springs: springs) {
            numArrangements += solve(puzzle: hashReplaced, springs: springs)
        }
    }
    return numArrangements
}

func part1(input: String) {
    let lines = input.split(separator: "\n")
    var answer = 0
    for line in lines {
        let components = line.split(separator: " ")
        let puzzle = String(components[0])
        let springs = components[1].split(separator: ",").map { Int($0)!  } as [Int]
        let numArrangements = solve(puzzle: puzzle, springs: springs)
        answer += numArrangements
    }
    print(answer)
}

var cache = [String: Int]()

func clean(puzzle: String) -> String {
    var cleaned = puzzle
    while cleaned.first == "." {
        cleaned.removeFirst()
    }
    while cleaned.last == "." {
        cleaned.removeLast()
    }
    // Remove consecutive .s
    var noDots = ""
    var previous = ""
    for char in cleaned {
        if (char == ".") && (previous == ".") {
            continue
        }
        noDots += String(char)
        previous = String(char)
    }
    return noDots
}

assert(clean(puzzle: "...#.#") == "#.#")
assert(clean(puzzle: "#.#...") == "#.#")
assert(clean(puzzle: "...#...###..#") == "#.###.#")

func smartSolve(puzzle: String, springs: [Int]) -> Int {
    let key = puzzle + springs.map { String($0) }.joined(separator: ",")
    if let cached = cache[key] {
        return cached
    }
    if springs.count == 0 {
        if puzzle.contains("#") {
            cache[key] = 0
            return 0
        } else {
            cache[key] = 1
            return 1
        }
    } else if (puzzle.count == 0) {
        cache[key] = 0
        return 0
    }
    let firstSpring = springs[0]
    let remainingSprings = springs[1..<springs.count]
    if puzzle.count < firstSpring {
        cache[key] = 0
        return 0
    }

    let numHashes = springs.reduce(0, +)
    let hashCount = puzzle.filter { $0 == "#" }.count
    let unknownCount = puzzle.filter { $0 == "?" }.count
    if (numHashes > hashCount + unknownCount) {
        cache[key] = 0
        return 0
    }

    if let firstChar = puzzle.first, firstChar == "#" {
        let endIndex = puzzle.index(puzzle.startIndex, offsetBy: firstSpring)
        let puzzleFirstSpring = puzzle[puzzle.startIndex..<endIndex]
        if puzzleFirstSpring.contains(".") {
            cache[key] = 0
            return 0
        } else if puzzle.count == firstSpring {
            if remainingSprings.count == 0 {
                cache[key] = 1
                return 1
            } else {
                cache[key] = 0
                return 0
            }
        } else {
            let afterIndexChar = puzzle[puzzle.index(puzzle.startIndex, offsetBy: firstSpring)]
            if afterIndexChar == "#" {
                cache[key] = 0
                return 0
            }
            let fillFirstSpring = smartSolve(
                puzzle: clean(puzzle: String(puzzle.dropFirst(firstSpring + 1))),
                springs: Array(remainingSprings)
            )
            cache[key] = fillFirstSpring
            return fillFirstSpring
        }
    } else {
        let puzzleEnd = String(puzzle.dropFirst(1))
        let hashStart = smartSolve(
            puzzle: clean(puzzle: "#" + puzzleEnd),
            springs: springs
        )
        let dotStart = smartSolve(
            puzzle: clean(puzzle: puzzleEnd),
            springs: springs
        )
        cache[key] = hashStart + dotStart
        return hashStart + dotStart
    }
}

func part2(input: String, repeats: Int) {
    let lines = input.split(separator: "\n")
    var answer = 0
    for line in lines {
        let components = line.split(separator: " ")
        let puzzle = String(components[0])
        let springs = components[1].split(separator: ",").map { Int($0)!  } as [Int]
        
        // Unfold the puzzle n times prior to solving
        var newPuzzle = puzzle
        for _ in 0..<repeats - 1 {
            newPuzzle += "?" + puzzle
        }
        let numArrangements = smartSolve(
            puzzle: clean(puzzle: newPuzzle),
            springs: Array(repeating: springs, count: repeats).flatMap { $0 }
        )
        answer += numArrangements
    }
    print(answer)
}

// Original part 1 regexp-based solution
//part1(input: readFile(path: "puzzle_12/example.txt"), repeats: 1)
//part1(input: readFile(path: "puzzle_12/input.txt"), repeats: 1)

part2(input: readFile(path: "puzzle_12/example.txt"), repeats: 1)
part2(input: readFile(path: "puzzle_12/input.txt"), repeats: 1)
part2(input: readFile(path: "puzzle_12/example.txt"), repeats: 5)
part2(input: readFile(path: "puzzle_12/input.txt"), repeats: 5)