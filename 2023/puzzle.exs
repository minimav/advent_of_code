defmodule Puzzle do
  def puzzle(input) do
    IO.puts(input)
  end
end

{:ok, example} = File.read("example.txt")
Puzzle.puzzle(example)

{:ok, input} = File.read("input.txt")
Puzzle.puzzle(input)
