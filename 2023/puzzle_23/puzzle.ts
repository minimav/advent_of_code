const { readFileSync } = require('fs')
 
function puzzle(input: string) {
  console.log(input);
}

puzzle(readFileSync('puzzle_23/example.txt', 'utf8'))
puzzle(readFileSync('puzzle_23/input.txt', 'utf8'))
    