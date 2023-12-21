const { readFileSync } = require('fs')
 
function puzzle(input) {
  console.log(input);
}

puzzle(readFileSync('puzzle_25/example.txt', 'utf8'))
puzzle(readFileSync('puzzle_25/input.txt', 'utf8'))
    