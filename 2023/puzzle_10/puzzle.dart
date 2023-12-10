import 'dart:io';

class Location {
  final int row;
  final int column;

  Location(this.row, this.column);

  bool equals(Location other) {
    return ((other.row == this.row) && (other.column == this.column));
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
    if (incoming.equals(Location(current.row + 1, current.column))) {
      // In from below, out above
      return Location(current.row - 1, current.column);
    } else {
      // In from above, out below
      return Location(current.row + 1, current.column);
    }
  } else if (currentPipe == "-") {
    if (incoming.equals(Location(current.row, current.column + 1))) {
      // In from right, out left
      return Location(current.row, current.column - 1);
    } else {
      // In from left, out right
      return Location(current.row, current.column + 1);
    }
  } else if (currentPipe == "L") {
    if (incoming.equals(Location(current.row - 1, current.column))) {
      // In from above, out right
      return Location(current.row, current.column + 1);
    } else {
      // In from right, out above
      return Location(current.row - 1, current.column);
    }
  } else if (currentPipe == "J") {
    if (incoming.equals(Location(current.row - 1, current.column))) {
      // In from above, out left
      return Location(current.row, current.column - 1);
    } else {
      // In from left, out above
      return Location(current.row - 1, current.column);
    }
  } else if (currentPipe == "7") {
    if (incoming.equals(Location(current.row, current.column - 1))) {
      // In from left, out below
      return Location(current.row + 1, current.column);
    } else {
      // In from below, out left
      return Location(current.row, current.column - 1);
    }
  } else if (currentPipe == "F") {
    if (incoming.equals(Location(current.row + 1, current.column))) {
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

void puzzle(var input) {
  
  List<String> pipes = input.split('\n');
  Location start = findStart(pipes);
  // Moving from the start is a special case of general movement

  Location previous = start;
  Location current = getMovement(start, pipes);

  int numMoves = 1;
  while (!current.equals(start)) {
    Location next = move(previous, current, pipes);
    previous = current;
    current = next;
    numMoves++;
  }  
  print(numMoves / 2);
}

void main() {
  File('puzzle_10/example.txt').readAsString().then((String contents) {
    puzzle(contents);
  });

  File('puzzle_10/input.txt').readAsString().then((String contents) {
    puzzle(contents);
  });
}