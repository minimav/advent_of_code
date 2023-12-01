
def puzzle(input)
    puts input
  end

example = File.open("example.txt").readlines.map(&:chomp)
puzzle(example)

input = File.open("input.txt").readlines.map(&:chomp)
puzzle(input)