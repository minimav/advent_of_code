def function(input: str):
    print(input)


if __name__ == "__main__":
    with open("example.txt", "r") as f:
        function(f.read())

    with open("input.txt", "r") as f:
        function(f.read())
