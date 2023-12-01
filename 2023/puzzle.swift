import Foundation

func readFile(path: String) -> String {
    let path=URL(fileURLWithPath: path)
    do {
        return try! String(contentsOf: path)
    } catch {
        return ""
    }
}

func puzzle(input: String) {
    print(input)
}

puzzle(input: readFile(path: "example.txt"))
puzzle(input: readFile(path: "input.txt"))