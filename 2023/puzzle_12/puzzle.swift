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

func puzzle(input: String) {
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

puzzle(input: readFile(path: "puzzle_12/example.txt"))
puzzle(input: readFile(path: "puzzle_12/input.txt"))