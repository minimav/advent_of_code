findSymmetry = function (strings, ignore)
    local bestCandidate = 0
    local bestIndex = 0
    for i = 1, #strings - 1 do
        local string = strings[i]
        -- Find consecutive equal strings, then expand from there
        if string == strings[i + 1] then
            local candidate = 1
            while i - candidate > 0 and i + candidate + 1 <= #strings do
                if strings[i - candidate] == strings[i + candidate + 1] then
                    candidate = candidate + 1
                else
                    -- Must reach the edge of the grid to be valid
                    candidate = 0
                    break
                end
            end

            if candidate > bestCandidate and i ~= ignore then
                bestCandidate = candidate
                bestIndex = i
            end
        end
    end
    return bestIndex
end

buildColumns = function (rows) 
    local columns = {}
    local numColumns = rows[1]:len()
    for index = 1, numColumns do
        local column = ""
        for _, row in ipairs(rows) do
            column = column .. string.sub(row, index, index)
        end
        table.insert(columns, column)
    end
    return columns
end

solveGrid = function (rows)
    local columns = buildColumns(rows)
    local rowsAboveReflection = findSymmetry(rows, -1)
    local columnsLeftOfReflection = findSymmetry(columns, -1)
    return {rowsAboveReflection, columnsLeftOfReflection}
end

copy = function (rows) 
    local newRows = {}
    for _, v in ipairs(rows) do
        table.insert(newRows, v)
    end
    return newRows
end

solveSmudgedGrid = function (rows)
    local originalRowsAboveReflection, originalColumnsLeftOfReflection = table.unpack(
        solveGrid(rows)
    )

    -- Find row/columns indexes to ignore previous solution when searching smudged grids
    local rowIgnore = -1
    if originalRowsAboveReflection > 0 then
        rowIgnore = originalRowsAboveReflection
    end

    local columnIgnore = -1
    if originalColumnsLeftOfReflection > 0 then
        columnIgnore = originalColumnsLeftOfReflection
    end

    -- Smudge the grid location by location
    for i = 1, #rows do
        for j = 1, rows[i]:len() do
            local newRows = copy(rows)
            local char = string.sub(newRows[i], j, j)
            local preChar = string.sub(newRows[i], 1, j - 1)
            local postChar = string.sub(newRows[i], j + 1)
            if char == "#" then
                newRows[i] = preChar .. "." .. postChar
            else
                newRows[i] = preChar .. "#" .. postChar
            end
            local columns = buildColumns(newRows)

            -- Need to ignore the previous symmetry explicitly so that we don't
            -- terminate the search too early
            local rowsAboveReflection = findSymmetry(newRows, rowIgnore)
            local columnsLeftOfReflection = findSymmetry(columns, columnIgnore)

            if rowsAboveReflection > 0 and rowsAboveReflection ~= originalRowsAboveReflection then
                return {rowsAboveReflection, 0}
            elseif columnsLeftOfReflection > 0 and columnsLeftOfReflection ~= originalColumnsLeftOfReflection then
                return {0, columnsLeftOfReflection}
            end
        end
    end
end

read_file = function (path)
    local file = io.open(path, "rb") 
    if not file then return nil end

    local grids = {}
    local rows = {}

    for line in io.lines(path) do
        if line == "" then
            table.insert(grids, rows)
            rows = {}
        else
            table.insert(rows, line)
        end
    end
    -- Insert the final grid
    table.insert(grids, rows)
    
    local answerPart1 = 0
    local answerPart2 = 0
    for _, rows in ipairs(grids) do
        local numRows, numColumns = table.unpack(solveGrid(rows))
        local numRowsSmudged, numColumnsSmudged = table.unpack(solveSmudgedGrid(rows))
        answerPart1 = answerPart1 + numColumns + 100 * numRows
        answerPart2 = answerPart2 + numColumnsSmudged + 100 * numRowsSmudged
    end

    print(answerPart1)
    print(answerPart2)
    file:close()
end

read_file("puzzle_13/example.txt")
read_file("puzzle_13/input.txt")