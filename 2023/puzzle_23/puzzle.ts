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

function getIgnoreList(outgoing: Map<String, Set<Complex>>, incoming: Map<String, Set<Complex>>, start: Complex, end: Complex) {
  let ignoreList: Set<String> = new Set();
  
  // Add dead ends to ignore list
  outgoing.forEach((outs, node) => {
    if (node === start.toString() || node === end.toString()) {
      return 
    }
    let ins = incoming.get(node)!;
    if ((outs.size === 1) && (ins.size === 1)) {
      ignoreList.add(node)
    }
  });
  
  // Walk backwards from dead ends
  let queue: String[] = Array.from(ignoreList);
  let visited: Set<String> = new Set();
  visited.add(start.toString());
  visited.add(end.toString());

  while (queue.length > 0) {
    let node = queue.shift()!;
    visited.add(node);
    let ins = incoming.get(node)!;
    let outs = outgoing.get(node)!;
    if ((ins.size == 2) && (outs.size == 2)) {
      ins.forEach((inNode) => {
        if (!visited.has(inNode.toString())) {
          queue.push(inNode.toString());
        }
      });
    }
  }

  return ignoreList;
}

function part2(input: string) {
  let {incoming, outgoing, start, end} = part2Parse(input);
  let graph = outgoing;

  var longest: number = 0
  var nodesProcessed: number = 0;
  let queue = new PriorityQueue({
    comparator: function(a: Path, b: Path) {
        return a.distance - b.distance;
    }
  });
  let seenBefore: Map<String, number> = new Map();
  let startPath: Path = {node: start, distance: 0, visited: new Set([start.toString()])}
  queue.queue(startPath);

  while (queue.length > 0) {
    nodesProcessed++;
    let path = queue.dequeue()!;
    if (nodesProcessed % 20000 === 0) {
      console.log(`Queue length ${queue.queue.length}, nodes processed ${nodesProcessed}, current node ${path.node}, path ${path.distance}`);
    }
    
    // let key = path.node.toString();
    // if (seenBefore.has(key) && (seenBefore.get(key)! > path.distance)) {
    //   //console.log(`Seen before, skipping ${key}`);
    //   continue;
    // }
    seenBefore.set(path.node.toString(), path.distance);
    
    if (path.node.real === end.real && path.node.imaginary === end.imaginary) {
      if (path.distance < longest) {
        console.log(
          `Path of ${path.distance} got to the end, queue length ${queue.queue.length}, nodes processed ${nodesProcessed}`
        );
        longest = path.distance;
      }
      continue;
    }
    
    let neighbours = graph.get(path.node.toString());
    neighbours!.forEach((neighbour) => {
      if (!path.visited.has(neighbour.toString())) {
        let newVisited: Set<String> = new Set(path.visited);
        newVisited.add(neighbour.toString());
        let newDistance = path.distance - 1;
        
        let newPath: Path = {
          node: neighbour,
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
//part1(example);
//part1(input);
part2(example);
part2(input);