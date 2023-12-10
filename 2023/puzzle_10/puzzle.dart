import 'dart:io';

class Location {
  final int row;
  final int column;

  Location(this.row, this.column);

  @override
  bool operator ==(Object other) {
    return other is Location &&
        other.row == row &&
        other.column == column;
  }

  @override
  int get hashCode => row.hashCode ^ column.hashCode;

  @override
  String toString() {
    return '($row, $column)';
  }

  List<Location> neighbours(int numRows, int numCols) {
    List<Location> neighbours = [];
    if (row > 0) {
      neighbours.add(Location(row - 1, column));
    }
    if (row < numRows - 1) {
      neighbours.add(Location(row + 1, column));
    }
    if (column > 0) {
      neighbours.add(Location(row, column - 1));
    }
    if (column < numCols - 1) {
      neighbours.add(Location(row, column + 1));
    }
    return neighbours;
  }
}

Location findStart(List<String> pipes) {
  int numRows = pipes.length;
  int numCols = pipes[0].length;

  for (int row = 0; row < numRows; row++) {
    for (int column = 0; column < numCols; column++) {
      if (pipes[row][column] == 'S') {
        return Location(row, column);
      }
    }
  }
  throw Exception('Start location not found');
}

Location move(Location incoming, Location current, List<String> pipes) {
  String incomingPipe = pipes[incoming.row][incoming.column];
  String currentPipe = pipes[current.row][current.column];
  if (currentPipe == '|') {
    if (incoming == Location(current.row + 1, current.column)) {
      // In from below, out above
      return Location(current.row - 1, current.column);
    } else {
      // In from above, out below
      return Location(current.row + 1, current.column);
    }
  } else if (currentPipe == "-") {
    if (incoming == Location(current.row, current.column + 1)) {
      // In from right, out left
      return Location(current.row, current.column - 1);
    } else {
      // In from left, out right
      return Location(current.row, current.column + 1);
    }
  } else if (currentPipe == "L") {
    if (incoming == Location(current.row - 1, current.column)) {
      // In from above, out right
      return Location(current.row, current.column + 1);
    } else {
      // In from right, out above
      return Location(current.row - 1, current.column);
    }
  } else if (currentPipe == "J") {
    if (incoming == Location(current.row - 1, current.column)) {
      // In from above, out left
      return Location(current.row, current.column - 1);
    } else {
      // In from left, out above
      return Location(current.row - 1, current.column);
    }
  } else if (currentPipe == "7") {
    if (incoming == Location(current.row, current.column - 1)) {
      // In from left, out below
      return Location(current.row + 1, current.column);
    } else {
      // In from below, out left
      return Location(current.row, current.column - 1);
    }
  } else if (currentPipe == "F") {
    if (incoming == Location(current.row + 1, current.column)) {
      // In from below, out right
      return Location(current.row, current.column + 1);
    } else {
      // In from right, out below
      return Location(current.row + 1, current.column);
    }
  }
  throw Exception('Invalid pipe found');
}

Location getMovement(Location start, List<String> pipes) {
  
  int numRows = pipes.length;
  int numCols = pipes[0].length;
  // Don't care about which of the two valid movements we choose to start moving

  // Vertical pipe below or above
  if ((start.row < numRows - 1) && (pipes[start.row + 1][start.column] == '|')) {
    return Location(start.row + 1, start.column);
  }
  if ((start.row > 0) && (pipes[start.row - 1][start.column] == '|')) {
    return Location(start.row - 1, start.column);
  }
  
  // Horizontal pipe to the right or left
  if ((start.column < numCols - 1) && (pipes[start.row][start.column + 1] == '-')) {
    return Location(start.row, start.column + 1);
  }
  if ((start.column > 0) && (pipes[start.row][start.column - 1] == '-')) {
    return Location(start.row, start.column - 1);
  }

  // Corner pipe below or left
  if ((start.row < numRows - 1) && (pipes[start.row + 1][start.column] == 'L')) {
    return Location(start.row + 1, start.column);
  }
  if ((start.column > 0) && (pipes[start.row][start.column - 1] == 'L')) {
    return Location(start.row, start.column - 1);
  }

  // Corner pipe below or right
  if ((start.row < numRows - 1) && (pipes[start.row + 1][start.column] == 'J')) {
    return Location(start.row + 1, start.column);
  }
  if ((start.column < numCols - 1) && (pipes[start.row][start.column + 1] == 'J')) {
    return Location(start.row, start.column + 1);
  }

  // Corner pipe above or right
  if ((start.row > 0) && (pipes[start.row - 1][start.column] == '7')) {
    return Location(start.row - 1, start.column);
  }
  if ((start.column < numCols - 1) && (pipes[start.row][start.column + 1] == '7')) {
    return Location(start.row, start.column + 1);
  }

  // Corner pipe above or left
  if ((start.row > 0) && (pipes[start.row - 1][start.column] == 'F')) {
    return Location(start.row - 1, start.column);
  }
  if ((start.column > 0) && (pipes[start.row][start.column - 1] == 'F')) {
    return Location(start.row, start.column - 1);
  }

  throw Exception('No valid movement found from start location');
}

void part1(var input) {
  List<String> pipes = input.split('\n');
  Location start = findStart(pipes);

  // Moving from the start is a special case of general movement
  Location previous = start;
  Location current = getMovement(start, pipes);

  int numMoves = 1;
  while (current != start) {
    Location next = move(previous, current, pipes);
    previous = current;
    current = next;
    numMoves++;
  }  
  print(numMoves / 2);
}

Set<Location> buildPath(Location start, List<String> pipes) {
  // Moving from the start is a special case of general movement
  Location previous = start;
  Location current = getMovement(start, pipes);
  Set<Location> path = {start, current};
  
  while (current != start) {
    Location next = move(previous, current, pipes);
    previous = current;
    current = next;
    path.add(current);
  }
  return path;
}

Set<Location> flood(List<String> pipes, Set<Location> path, Set<Location> contained, Location start) {
  Set<Location> toCheck = {start};
  Set<Location> checked = Set();
  while (toCheck.isNotEmpty) {
    Location current = toCheck.first;
    toCheck.remove(current);
    checked.add(current);
    if (path.contains(current)) {
      continue;
    }
    contained.add(current);
    toCheck.addAll(current.neighbours(pipes.length, pipes[0].length).where((neighbour) => !checked.contains(neighbour)));
  }
  return contained;
}

// Maintain a tracking location that keeps on the same 'side' of the path at all times
Location getNextTracker(Location current, Location next) {
  if (current.row == next.row) {
    if (current.column < next.column) {
      // Moved right, so tracker is one above next
      return Location(next.row - 1, next.column);
    } else {
      // Moved left, so tracker is one below next
      return Location(next.row + 1, next.column);
    }
  } else {
    if (current.row < next.row) {
      // Moved down, so tracker is one to the left of next
      return Location(next.row , next.column + 1);
    } else {
      // Moved up so tracker is one to the right of next
      return Location(next.row, next.column - 1);
    }
  }
}

void part2(var input) {
  List<String> pipes = input.split('\n');
  Location start = findStart(pipes);

  Set<Location> path = buildPath(start, pipes);
  Set<Location> contained = Set();
  Location previous = start;
  
  // Left perspective works for first this location in first examples
  //Location current = Location(3, 0);
  //Location innerTracker = Location(3, 1);

  // example_2
  //Location current = Location(0, 3);
  //Location innerTracker = Location(1, 3);

  // example_3 start at (4, 12)
  //Location current = Location(4, 13);
  //Location innerTracker = Location(3, 13);

  // example_4
  //Location current = Location(4, 12);
  //Location innerTracker = Location(2, 2);
  
  // Real input
  // Start at (41, 111), can go left to (41, 110) or up to (40, 111)
  Location current = Location(40, 111);
  Location innerTracker = Location(40, 110);

  while (current != start) {
    contained = flood(pipes, path, contained, innerTracker);
    Location next = move(previous, current, pipes);
    innerTracker = getNextTracker(current, next);
    previous = current;
    current = next;
  }

  print('Path length: ${path.length}');
  print('Contained length: ${contained.length}');
}

void main() {
  File('puzzle_10/example.txt').readAsString().then((String contents) {
    // Should be 8
    //part1(contents);
    // Should be 1
    //part2(contents);
  });

  File('puzzle_10/example_2.txt').readAsString().then((String contents) {
    // Should be 4
    //part2(contents);
  });

  File('puzzle_10/example_3.txt').readAsString().then((String contents) {
    // Should be 8
    //part2(contents);
  });

  File('puzzle_10/example_4.txt').readAsString().then((String contents) {
    // Should be 10
    //part2(contents);
  });

  File('puzzle_10/input.txt').readAsString().then((String contents) {
    //part1(contents);
    part2(contents);
  });
}