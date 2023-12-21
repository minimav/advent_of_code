def function(input: str):
    print(input)


if __name__ == "__main__":
    with open("puzzle_24/example.txt", "r") as f:
        function(f.read())

    with open("puzzle_24/input.txt", "r") as f:
        function(f.read())
