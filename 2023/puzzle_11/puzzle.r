
getDistance <- function(galaxy, otherGalaxy, dotRows, dotColumns, expansionFactor) {
    startRow = galaxy[1]
    endRow = otherGalaxy[1]
    minRow = min(startRow, endRow)
    maxRow = max(startRow, endRow)

    rowsToTraverse = dotRows[minRow:maxRow]
    distance = maxRow - minRow + sum(rowsToTraverse) * (expansionFactor - 1)

    startColumn = galaxy[2]
    endColumn = otherGalaxy[2]
    minColumn = min(startColumn, endColumn)
    maxColumn = max(startColumn, endColumn)
    columnsToTraverse = dotColumns[minColumn:maxColumn]
    distance = distance + maxColumn - minColumn + sum(columnsToTraverse) * (expansionFactor - 1)

    distance
}

puzzle <- function(lines, expansionFactor) {
    numRows = length(lines)
    numColumns = nchar(lines[[1]])
    
    galaxies = list()
    dotRows = rep(TRUE, numRows)
    dotColumns = rep(TRUE, numColumns)
    for (i in 1:numRows) {
        line <- lines[[i]]
        for (j in 1:numColumns) {
            char <- substr(line, start = j, stop = j)
            if (char == "#") {
                galaxies <- append(galaxies, list(c(i, j)))
                dotRows[i] = FALSE
                dotColumns[j] = FALSE
            }
        }
    }

    answer = 0
    numGalaxies = length(galaxies)
    for (i in 1:numGalaxies) {
        galaxy <- galaxies[[i]]
        for (j in 1:numGalaxies) {
            if (i >= j) {
                next
            }
            otherGalaxy <- galaxies[[j]]
            distance = getDistance(
                galaxy, otherGalaxy, dotRows, dotColumns, expansionFactor
            )
            answer = answer + distance
        }
        
    }
    print(answer)
}

puzzle(readLines("puzzle_11/example.txt"), 2)
puzzle(readLines("puzzle_11/input.txt"), 2)
puzzle(readLines("puzzle_11/example.txt"), 10)
puzzle(readLines("puzzle_11/example.txt"), 100)
puzzle(readLines("puzzle_11/input.txt"), 1000000)
