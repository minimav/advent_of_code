
function puzzle(input)
    println(input)
end

open("example.txt") do example
    puzzle(read(example, String))
end

open("input.txt") do input
    puzzle(read(input, String))
end