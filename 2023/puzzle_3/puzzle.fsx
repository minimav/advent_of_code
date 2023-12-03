open System.IO

let lines = File.ReadLines("puzzle_3/input.txt") |> Seq.toList

////////////////////// Part 1

// Create array of booleans that indicate if there is an adjacent symbol
let numRows = List.length lines
let numColumns = String.length lines.[0]
let (rawSymbolAdjacent : bool array2d) = Array2D.create numRows numColumns false

let linesWithRowIndex = List.mapi (fun index element -> (index, element)) lines
let linesWithRowIndexAndColumnIndex = List.map (fun (rowIndex, line) -> List.mapi (fun index element -> (rowIndex, index, element)) (Seq.toList line)) linesWithRowIndex

let symbolIndexes = List.filter (fun (rowIndex, columnIndex, char) -> not (System.Char.IsDigit char)) (List.concat linesWithRowIndexAndColumnIndex)
let symbolAdjacent = List.fold (fun (acc: bool array2d) (rowIndex, columnIndex, symbol) ->
                                   if (symbol <> '.') then acc.[rowIndex, columnIndex] <- true
                                   if (symbol <> '.') && (rowIndex > 0) then acc.[rowIndex - 1, columnIndex] <- true
                                   if (symbol <> '.') && (columnIndex > 0) then acc.[rowIndex, columnIndex - 1] <- true
                                   if (symbol <> '.') && (rowIndex < numRows - 1) then acc.[rowIndex + 1, columnIndex] <- true
                                   if (symbol <> '.') && (columnIndex < numColumns - 1) then acc.[rowIndex, columnIndex + 1] <- true
                                   if (symbol <> '.') && (rowIndex > 0) && (columnIndex > 0) then acc.[rowIndex - 1, columnIndex - 1] <- true
                                   if (symbol <> '.') && (rowIndex < numRows - 1) && (columnIndex > 0) then acc.[rowIndex + 1, columnIndex - 1] <- true
                                   if (symbol <> '.') && (rowIndex > 0) && (columnIndex < numColumns - 1) then acc.[rowIndex - 1, columnIndex + 1] <- true
                                   if (symbol <> '.') && (rowIndex < numRows - 1) && (columnIndex < numColumns - 1) then acc.[rowIndex + 1, columnIndex + 1] <- true
                                   acc
                               ) rawSymbolAdjacent symbolIndexes

let digitLocations = List.concat (linesWithRowIndexAndColumnIndex |> List.map (List.filter (fun (rowIndex, columnIndex, char) -> System.Char.IsDigit char)))

let groupDigits l =
  let concat otherDigit l =
    let (otherRow, otherColumn, otherChar) = otherDigit
    match l with
    | [] -> [[otherDigit]]
    | (x::xs)::tail ->
        let (row, col, digit) = x
        if (row = otherRow) && (col - otherColumn = 1) then (otherDigit::x::xs)::tail
        else [otherDigit]::(x::xs)::tail
    | _ -> failwith "Unreachable"

  List.foldBack concat l []

let digitGroups = groupDigits digitLocations
let validDigitGroups = List.filter (fun group ->
                                        List.exists (fun (rowIndex, columnIndex, char) ->
                                                        symbolAdjacent.[rowIndex, columnIndex]
                                                    ) group
                                    ) digitGroups

let getNumber (digits: (int * int * char) list) =
    let (row, startCol, _) = digits.[0]
    let (_, endCol, _) = digits.[List.length digits - 1]
    System.Int32.Parse lines.[row].[startCol..endCol]

let numbers = List.map getNumber validDigitGroups
printfn "%A" (List.sum numbers);;

////////////////////// Part 2
let starIndexes = List.filter (fun (rowIndex, columnIndex, char) -> char = '*') (List.concat linesWithRowIndexAndColumnIndex)

let findDigitGroupsNearStar (starIndex: (int * int * char)) =
    let (starRow, starColumn, _) = starIndex
    List.filter (fun group ->
                    List.exists (fun (rowIndex, columnIndex, char) ->
                                    (System.Math.Abs(rowIndex - starRow) <= 1) && (System.Math.Abs(columnIndex - starColumn) <= 1)
                                ) group
                ) digitGroups

let digitGroupsNearAStar = List.map findDigitGroupsNearStar starIndexes
let pairsNearAStar = List.filter (fun group -> List.length group = 2) digitGroupsNearAStar

let numberGroups = List.map (List.map getNumber) pairsNearAStar
let products = List.map (fun x -> List.fold (fun acc y -> acc * y) 1 x) numberGroups
printfn "%A" (List.sum products);;
