defmodule Puzzle do
  def traverse(node, step, mappings, instructions) do
    num_instructions = String.length(instructions)
    instruction_index = rem(step, num_instructions)
    instruction = instructions |> String.at(instruction_index)

    next_node =
      case instruction do
        "L" ->
          Enum.at(mappings[node], 0)

        _ ->
          Enum.at(mappings[node], 1)
      end

    if next_node == "ZZZ" do
      step + 1
    else
      traverse(next_node, step + 1, mappings, instructions)
    end
  end

  def traverse_part_2(node, step, mappings, instructions) do
    num_instructions = String.length(instructions)
    instruction_index = rem(step, num_instructions)
    instruction = instructions |> String.at(instruction_index)

    next_node =
      case instruction do
        "L" ->
          Enum.at(mappings[node], 0)

        _ ->
          Enum.at(mappings[node], 1)
      end

    if String.ends_with?(next_node, "Z") do
      step + 1
    else
      traverse_part_2(next_node, step + 1, mappings, instructions)
    end
  end

  def parse(input) do
    # Make parsing a little easier
    input = String.replace(input, " = ", " ")
    input = String.replace(input, "(", "")
    input = String.replace(input, ")", "")
    input = String.replace(input, ",", "")

    lines = String.split(input, "\n")
    [instructions | mapping_strings_with_junk] = lines
    # Remove blank line
    [_ | mapping_strings] = mapping_strings_with_junk

    mappings =
      Enum.map(mapping_strings, fn mapping ->
        [key, left, right] = String.split(mapping, " ")
        {key, [left, right]}
      end)
      |> Enum.into(%{})

    {instructions, mappings}
  end

  def part_1(input) do
    {instructions, mappings} = parse(input)
    IO.puts(traverse("AAA", 0, mappings, instructions))
  end

  defp gcd(0, b), do: b
  defp gcd(a, b), do: gcd(rem(b, a), a)
  defp calculate_lcm([h | t]), do: Enum.reduce(t, h, &lcm/2)
  defp lcm(a, b), do: round(abs(a * b) / gcd(a, b))

  def part_2(input) do
    {instructions, mappings} = parse(input)

    cycles =
      Map.keys(mappings)
      |> Enum.filter(fn key -> String.ends_with?(key, "A") end)
      |> Enum.map(fn key -> traverse_part_2(key, 0, mappings, instructions) end)

    lcm = calculate_lcm(cycles)
    IO.puts(lcm)
  end
end

{:ok, example} = File.read("puzzle_8/example.txt")
{:ok, example_part_2} = File.read("puzzle_8/example_2.txt")
{:ok, input} = File.read("puzzle_8/input.txt")

Puzzle.part_1(example)
Puzzle.part_2(example_part_2)

Puzzle.part_1(input)
Puzzle.part_2(input)
