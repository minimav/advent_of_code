const { readFileSync } = require('fs')
 
function puzzle(input) {
  console.log(input);
}

puzzle(readFileSync('example.txt', 'utf8'))
puzzle(readFileSync('input.txt', 'utf8'))
    