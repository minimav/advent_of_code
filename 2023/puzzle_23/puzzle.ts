const { readFileSync } = require('fs')
const PriorityQueue = require('js-priority-queue');
 
interface Complex {
  real: number;
  imaginary: number;
}

function Complex(real: number, imaginary: number) {
  this.real = real;
  this.imaginary = imaginary;
}

Complex.prototype.add = function(other: Complex) {
  return new Complex(this.real + other.real, this.imaginary + other.imaginary);
};

Complex.prototype.toString = function() {
  return `${this.real},${this.imaginary}`;
}

let vectors: Complex[] = [
  new Complex(-1, 0),
  new Complex(1, 0),
  new Complex(0, -1),
  new Complex(0, 1),
];

interface Path {
  node: Complex;
  distance: number;
  visited: Set<String>;
}

function part1Parse(input: string) {
  let lines: string[] = input.split('\n');
  let numRows = lines.length;
  let numCols = lines[0].length;
  let graph: Map<String, Complex[]> = new Map();
  var start: Complex, end: Complex;
  lines.forEach((line: string, rowIndex: number) => {
    line.split('').forEach((char: string, columnIndex: number) => {
      if (char !== '#') {
        let node = new Complex(columnIndex, rowIndex);
        var neighbours: Complex[] = [];
        vectors.forEach((vector: Complex) => {
          let neighbour = node.add(vector);
          // Check in bounds and not a wall
          if (neighbour.real >= 0 && neighbour.real < numCols && neighbour.imaginary >= 0 && neighbour.imaginary < numRows && lines[neighbour.imaginary].charAt(neighbour.real) !== '#') {
            let char = lines[neighbour.imaginary].charAt(neighbour.real);
            if (char === '.') {
              neighbours.push(neighbour);
            } else {
              // Need to check not going against a slide, slight fudge as 
              // grid orientation is flipped when using complex numbers
              if (
                (vector.toString() === "-1,0" && char !== '>')
                || (vector.toString() === "1,0" && char !== '<')
                || (vector.toString() === "0,-1" && char !== 'v')
                || (vector.toString() === "0,1")
              ) {
                neighbours.push(neighbour.add(vector))
              };
            }
          }
        });
        graph.set(node.toString(), neighbours);
        if (rowIndex === 0) {
          start = node;
        } else if (rowIndex === numRows - 1) {
          end = node;
        }
      }
    });
  });
  return {graph, start, end};
}

function part2Parse(input: string) {
  let lines: string[] = input.split('\n');
  let numRows = lines.length;
  let numCols = lines[0].length;
  let outgoing: Map<String, Set<Complex>> = new Map();
  let incoming: Map<String, Set<Complex>> = new Map();
  var start: Complex, end: Complex;
  lines.forEach((line: string, rowIndex: number) => {
    line.split('').forEach((char: string, columnIndex: number) => {
      if (char !== '#') {
        let node = new Complex(columnIndex, rowIndex);
        var neighbours: Complex[] = [];
        vectors.forEach((vector: Complex) => {
          let neighbour = node.add(vector);
          // Check in bounds and not a wall
          if (neighbour.real >= 0 && neighbour.real < numCols && neighbour.imaginary >= 0 && neighbour.imaginary < numRows && lines[neighbour.imaginary].charAt(neighbour.real) !== '#') {
            neighbours.push(neighbour);
          }
        });
        outgoing.set(node.toString(), new Set(neighbours));
        neighbours.forEach((neighbour) => {
          let neighbourKey = neighbour.toString();
          if (!incoming.has(neighbourKey)) {
            incoming.set(neighbourKey, new Set());
          }
          incoming.get(neighbourKey)!.add(node);
        })
        if (rowIndex === 0) {
          start = node;
        } else if (rowIndex === numRows - 1) {
          end = node;
        }
      }
    });
  });
  return {incoming, outgoing, start, end};
}

function part1(input: string) {
  let {graph, start, end} = part1Parse(input);

  var longest: number = 0
  let queue: Path[] = [
    {node: start, distance: 0, visited: new Set([start.toString()])}
  ];
  while (true) {
    let nextPath = queue.shift();
    if (nextPath === undefined) {
      break;
    }
    let path: Path = nextPath!;
    
    if (path.node.real === end.real && path.node.imaginary === end.imaginary) {
      console.log(`Path of ${path.distance} got to the end`);
      if (path.distance > longest) {
        longest = path.distance;
      }
      continue;
    }
    
    let neighbours = graph.get(path.node.toString());
    neighbours!.forEach((neighbour) => {
      if (!path.visited.has(neighbour.toString())) {
        let newVisited = new Set(path.visited);
        newVisited.add(neighbour.toString());
        let newPath: Path = {
          node: neighbour,
          distance: path.distance + Math.abs(neighbour.real - path.node.real) + Math.abs(neighbour.imaginary - path.node.imaginary),
          visited: newVisited
        };
        queue.push(newPath);
      }
    });
  }
  console.log(longest);
}

interface Edge {
  from: Complex;
  to: Complex;
  distance: number;
}

function contract(start: Complex, end: Complex, outgoing: Map<String, Set<Complex>>) {
  let graph: Map<String, Set<Edge>> = new Map();
  let queue = new PriorityQueue({
    comparator: function(a: Path, b: Path) {
        return b.distance - a.distance;
    }
  });
  let next = new Complex(start.real, start.imaginary + 1);
  let startPath =  {
    from: start,
    to: next,
    distance: 1,
    visited: new Set([
      start.toString(),
      next.toString()
    ])
  }
  let branchNodes: Set<String> = new Set([start.toString()]);
  queue.queue(startPath);

  while (queue.length > 0) {
    let path = queue.dequeue()!;
    let fromKey = path.from.toString();

    // Move till reaching a branch point
    var node = path.to
    var distance = 0
    while (true) {
      let nodeKey = node.toString();
      let neighbours = Array.from(outgoing.get(nodeKey)!.values());
      if (neighbours.length > 2) {
        // Branch node, possibly seen before
        if (!graph.has(fromKey)) {
          graph.set(fromKey, new Set());
        }
        let edge = {
          from: path.from,
          to: node,
          distance: path.distance + distance
        };
        graph.get(fromKey)!.add(edge);

        if (branchNodes.has(nodeKey)) { 
          // We've seen it before, so after making edge into it, don't add to the
          // queue from it
          break;
        }
        branchNodes.add(nodeKey);
        neighbours.forEach((neighbour) => {
          let newPath = {
            from: node,
            to: neighbour,
            distance: 1,
            visited: new Set([nodeKey, neighbour.toString()])
          };
          queue.queue(newPath);
        });
        break;
      } else if (neighbours.length === 1) {
        if (nodeKey === end.toString()) {
          if (!graph.has(fromKey)) {
            graph.set(fromKey, new Set());
          }
          let edgeToEnd = {
            from: path.from,
            to: end,
            distance: path.distance + distance
          };
          graph.get(fromKey)!.add(edgeToEnd);
        }
        break;
      }

      // Choose node not in path visited so far as one to continue along
      for (let i = 0; i < neighbours.length; i++) {
        let neighbour = neighbours[i];
        if (!path.visited.has(neighbour.toString())) {
          node = neighbour;
          break;
        }
      }
      path.visited.add(node.toString());
      distance++;
    }
  }
  return graph;
}

function part2(input: string) {
  let {outgoing, start, end} = part2Parse(input);
  let graph: Map<String, Set<Edge>> = contract(start, end, outgoing);

  var longest: number = 0
  var nodesProcessed: number = 0;
  let queue = new PriorityQueue({
    comparator: function(a: Path, b: Path) {
        return a.distance - b.distance;
    }
  });
  let startPath: Path = {node: start, distance: 0, visited: new Set([start.toString()])}
  queue.queue(startPath);

  while (queue.length > 0) {
    nodesProcessed++;
    let path = queue.dequeue()!; 
    var nodeKey = path.node.toString()

    if (nodeKey === end.toString()) {
      if (path.distance < longest) {
        console.log(
          `Path of ${path.distance} got to the end, queue length ${queue.length}, nodes processed ${nodesProcessed}`
        );
        longest = path.distance;
      }
      continue;
    }

    var outgoingEdges = graph.get(nodeKey)!;
    outgoingEdges.forEach((edge) => {
      let toKey = edge.to.toString();
      if (!path.visited.has(toKey)) {
        let newVisited: Set<String> = new Set(path.visited);
        newVisited.add(toKey);
        let newDistance = path.distance - edge.distance;
        
        let newPath: Path = {
          node: edge.to,
          distance: newDistance,
          visited: newVisited
        };
        queue.queue(newPath);
      }
    });
  }
  console.log(Math.abs(longest));
}

let example = readFileSync('example.txt', 'utf8');
let input = readFileSync('input.txt', 'utf8');
part1(example);
part1(input);
part2(example);
part2(input);