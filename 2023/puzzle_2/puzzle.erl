-module(puzzle).
-export([solvePuzzles/0]).

readlines(FileName) ->
    {ok, Data} = file:read_file(FileName),
    BinaryInput = binary:split(Data, [<<"\n">>], [global]),
    lists:map(fun(X) -> binary_to_list(X) end, BinaryInput).

extractGameNumber(Line) ->
    [FirstPart | _] = string:split(Line, ":", all),
    [_ | GameNumberString] = string:split(FirstPart, " ", all),
    list_to_integer(hd(GameNumberString)).

checkAgainstCubeCounts(CubeInfo) ->
    CubeCounts = #{"blue" => 14, "green" => 13, "red" => 12},
    [RawCount | Remainder] = string:split(CubeInfo, " ", all),
    Colour = hd(Remainder),
    Count = list_to_integer(RawCount),
    ValidCube = maps:get(Colour, CubeCounts) >= Count,
    if
        ValidCube ->
            true;
        true ->
            false
    end.

isValidCubeSet(CubeSet) ->
    % Assume only 1 occurrence of each colour per cube set
    Cubes = string:split(CubeSet, ", ", all),
    lists:all(fun checkAgainstCubeCounts/1, Cubes).

isValidGameNumber(Line) ->
    GameNumber = extractGameNumber(Line),
    % Get sets of cubes removed at the same time
    [_, CubeInformation] = string:split(Line, ": ", all),
    CubeExtractionSets = string:split(CubeInformation, "; ", all),
    % Determine which of those sets are valid
    ValidCubeSets = lists:map(fun isValidCubeSet/1, CubeExtractionSets),
    % Return the game number if all of them were valid
    AllValid = lists:all(fun(X) -> X end, ValidCubeSets),
    if
        AllValid ->
            GameNumber;
        true ->
            0
    end.

solvePart1(Path) ->
    Lines = readlines(Path),
    GameNumbers = lists:map(fun isValidGameNumber/1, Lines),
    Answer = lists:sum(GameNumbers),
    io:fwrite("~p~n", [Answer]).

getCubeCount(CubeInfo) ->
    [RawCount | Remainder] = string:split(CubeInfo, " ", all),
    Colour = hd(Remainder),
    Count = list_to_integer(RawCount),
    {Colour, Count}.

getCubeCounts(CubeSet) ->
    Cubes = string:split(CubeSet, ", ", all),
    lists:map(fun getCubeCount/1, Cubes).

getMaxCubeCountPerColour(CubeCount, CubeCounts) ->
    {Colour, Count} = CubeCount,
    maps:update_with(Colour, fun(V) -> max(V, Count) end, 1, CubeCounts).

getGamePower(Line) ->
    [_, CubeInformation] = string:split(Line, ": ", all),
    CubeExtractionSets = string:split(CubeInformation, "; ", all),
    CubeCountsPerSet = lists:map(fun getCubeCounts/1, CubeExtractionSets),

    % Calculate the max cube count per colour
    InitialCounts = #{"blue" => 0, "green" => 0, "red" => 0},
    MaxCountsPerColour = lists:foldl(
        fun getMaxCubeCountPerColour/2,
        InitialCounts,
        lists:flatten(CubeCountsPerSet)
    ),
    % Calculate the game power once max cube counts per colour is known
    lists:foldl(fun(X, Y) -> X * Y end, 1, maps:values(MaxCountsPerColour)).

solvePart2(Path) ->
    Lines = readlines(Path),
    GamePowers = lists:map(fun getGamePower/1, Lines),
    Answer = lists:sum(GamePowers),
    io:fwrite("~p~n", [Answer]).

solvePuzzles() ->
    io:fwrite("Part 1 Example: "),
    solvePart1("../puzzle_2/example.txt"),
    io:fwrite("Part 1 Input: "),
    solvePart1("../puzzle_2/input.txt"),
    io:fwrite("Part 2 Example: "),
    solvePart2("../puzzle_2/example.txt"),
    io:fwrite("Part 2 Input: "),
    solvePart2("../puzzle_2/input.txt").
