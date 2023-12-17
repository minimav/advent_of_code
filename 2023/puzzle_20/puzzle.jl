
function puzzle(input)
    println(input)
end

open("puzzle_20/example.txt") do example
    puzzle(read(example, String))
end

open("puzzle_20/input.txt") do input
    puzzle(read(input, String))
end