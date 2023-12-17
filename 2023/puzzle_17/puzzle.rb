require 'set'
require 'rubygems'
require 'algorithms'

def part_1(input)
  grid = input.map(&:chars)
  num_rows = grid.length
  num_columns = grid[0].length
  puts "Rows: #{num_rows}"
  puts "Columns: #{num_columns}"
  
  queue = Containers::PriorityQueue.new
  start = [0, 0, 0, "-", 0]
  queue.push(start, 0)
  seen_before = Hash.new
  seen_before["0,0,0,-"] = 0
  solutions = []

  while queue.size() > 0
    current = queue.pop()
    row, column, num_straight_moves, direction, loss = current

    if row == num_rows - 1 && column == num_columns - 1
      solutions.push(loss)
      next
    end

    # Move down
    if row < num_rows - 1 && !(direction == "D" && num_straight_moves == 3) && !(direction == "U")
      if direction == "D"
        new_num_straight_moves = num_straight_moves + 1
      else
        new_num_straight_moves = 1
      end
      new_loss = loss + grid[row + 1][column].to_i
      new_path = [row + 1, column, new_num_straight_moves, "D", new_loss]
      new_key = "#{row + 1},#{column},#{new_num_straight_moves},D"

      if !seen_before.key?(new_key) | (seen_before[new_key] && seen_before[new_key] > new_loss)
        seen_before[new_key] = new_loss
        queue.push(new_path, -new_loss)
      end
    end

    # Move up
    if row > 0 && !(direction == "U" && num_straight_moves == 3) && !(direction == "D")
      if direction == "U"
        new_num_straight_moves = num_straight_moves + 1
      else
        new_num_straight_moves = 1
      end
      new_loss = loss + grid[row - 1][column].to_i
      new_path = [row - 1, column, new_num_straight_moves, "U", new_loss]
      new_key = "#{row - 1},#{column},#{new_num_straight_moves},U"

      if !seen_before.key?(new_key) | (seen_before[new_key] && seen_before[new_key] > new_loss)
        seen_before[new_key] = new_loss
        queue.push(new_path, -new_loss)
      end
    end

    # Move left
    if column > 0 && !(direction == "L" && num_straight_moves == 3) && !(direction == "R")
      if direction == "L"
        new_num_straight_moves = num_straight_moves + 1
      else
        new_num_straight_moves = 1
      end
      new_loss = loss + grid[row][column - 1].to_i
      new_path = [row, column - 1, new_num_straight_moves, "L", new_loss]
      new_key = "#{row},#{column - 1},#{new_num_straight_moves},L"

      if !seen_before.key?(new_key) | (seen_before[new_key] && seen_before[new_key] > new_loss)
        seen_before[new_key] = new_loss
        queue.push(new_path, -new_loss)
      end
    end

    # Move right
    if column < num_columns - 1 && !(direction == "R" && num_straight_moves == 3) && !(direction == "L")
      if direction == "R"
        new_num_straight_moves = num_straight_moves + 1
      else
        new_num_straight_moves = 1
      end
      new_loss = loss + grid[row][column + 1].to_i
      new_path = [row, column + 1, new_num_straight_moves, "R", new_loss]
      new_key = "#{row},#{column + 1},#{new_num_straight_moves},R"

      if !seen_before.key?(new_key) | (seen_before[new_key] && seen_before[new_key] > new_loss)
        seen_before[new_key] = new_loss
        queue.push(new_path, -new_loss)
      end
    end
    
  end
  puts solutions.min
end

def part_2(input)
  grid = input.map(&:chars)
  num_rows = grid.length
  num_columns = grid[0].length
  puts "Rows: #{num_rows}"
  puts "Columns: #{num_columns}"
  
  queue = Containers::PriorityQueue.new
  # New turning condition means need to initialize with 2 paths
  start_right = [0, 0, 0, "R", 0]
  start_down = [0, 0, 0, "D", 0]
  queue.push(start_right, 0)
  queue.push(start_down, 0)
  seen_before = Hash.new
  seen_before["0,0,0,R"] = 0
  seen_before["0,0,0,D"] = 0
  
  solutions = []

  while queue.size() > 0
    current = queue.pop()
    row, column, num_straight_moves, direction, loss = current

    if row == num_rows - 1 && column == num_columns - 1
      if num_straight_moves < 4
        next
      end
      solutions.push(loss)
      next
    end

    # Move down
    if (
      row < num_rows - 1 &&
      (direction != "U") &&
      (
        (direction != "D" && num_straight_moves >= 4) ||
        (direction == "D" && num_straight_moves < 10)
      )
    )
      if direction == "D"
        new_num_straight_moves = num_straight_moves + 1
      else
        new_num_straight_moves = 1
      end
      new_loss = loss + grid[row + 1][column].to_i
      new_path = [row + 1, column, new_num_straight_moves, "D", new_loss]
      new_key = "#{row + 1},#{column},#{new_num_straight_moves},D"

      if !seen_before.key?(new_key) | (seen_before[new_key] && seen_before[new_key] > new_loss)
        seen_before[new_key] = new_loss
        queue.push(new_path, -new_loss)
      end
    end

    # Move up
    if (
      row > 0 && 
      (direction != "D") &&
      (
        (direction != "U" && num_straight_moves >= 4) ||
        (direction == "U" && num_straight_moves < 10)
      )
    )
      if direction == "U"
        new_num_straight_moves = num_straight_moves + 1
      else
        new_num_straight_moves = 1
      end
      new_loss = loss + grid[row - 1][column].to_i
      new_path = [row - 1, column, new_num_straight_moves, "U", new_loss]
      new_key = "#{row - 1},#{column},#{new_num_straight_moves},U"

      if !seen_before.key?(new_key) | (seen_before[new_key] && seen_before[new_key] > new_loss)
        seen_before[new_key] = new_loss
        queue.push(new_path, -new_loss)
      end
    end

    # Move left
    if (
      column > 0 &&
      (direction != "R") &&
      (
        (direction != "L" && num_straight_moves >= 4) ||
        (direction == "L" && num_straight_moves < 10)
      )
    )
      if direction == "L"
        new_num_straight_moves = num_straight_moves + 1
      else
        new_num_straight_moves = 1
      end
      new_loss = loss + grid[row][column - 1].to_i
      new_path = [row, column - 1, new_num_straight_moves, "L", new_loss]
      new_key = "#{row},#{column - 1},#{new_num_straight_moves},L"

      if !seen_before.key?(new_key) | (seen_before[new_key] && seen_before[new_key] > new_loss)
        seen_before[new_key] = new_loss
        queue.push(new_path, -new_loss)
      end
    end

    # Move right
    if (
      column < num_columns - 1 &&
      (direction != "L") &&
      (
        (direction != "R" && num_straight_moves >= 4) ||
        (direction == "R" && num_straight_moves < 10)
      )
    )
      if direction == "R"
        new_num_straight_moves = num_straight_moves + 1
      else
        new_num_straight_moves = 1
      end
      new_loss = loss + grid[row][column + 1].to_i
      new_path = [row, column + 1, new_num_straight_moves, "R", new_loss]
      new_key = "#{row},#{column + 1},#{new_num_straight_moves},R"

      if !seen_before.key?(new_key) | (seen_before[new_key] && seen_before[new_key] > new_loss)
        seen_before[new_key] = new_loss
        queue.push(new_path, -new_loss)
      end
    end
    
  end
  puts solutions.min
end


example = File.open("puzzle_17/example.txt").readlines.map(&:chomp)
part_1(example)
part_2(example)

example_2 = File.open("puzzle_17/example_2.txt").readlines.map(&:chomp)
part_2(example_2)

input = File.open("puzzle_17/input.txt").readlines.map(&:chomp)
part_1(input)
part_2(input)