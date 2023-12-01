import 'dart:io';

void puzzle(var input) {
  print(input);
}

void main() {
  File('example.txt').readAsString().then((String contents) {
    puzzle(contents);
  });

  File('input.txt').readAsString().then((String contents) {
    puzzle(contents);
  });
}