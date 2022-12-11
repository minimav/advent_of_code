from collections import defaultdict
from functools import reduce


def parse_monkeys(lines: str) -> dict:
    monkeys = {}
    for line in lines:
        if not line:
            monkeys[monkey_index] = dict(
                items=items,
                op=op,
                test=test,
                true_index=true_index,
                false_index=false_index,
            )
        elif "Monkey" in line:
            monkey_index = int(line.rstrip(":").split(" ")[-1])
        elif "Starting" in line:
            _, raw_items = line.split("items: ")
            items = [int(i) for i in raw_items.split(",")]
        elif "Operation" in line:
            _, raw_op = line.split("new =")
            if "old * old" in raw_op:
                op = ("square",)
            elif "*" in raw_op:
                multiplier = int(line.split("* ")[-1])
                op = ("*", multiplier)
            elif "+" in raw_op:
                adder = int(line.split("+ ")[-1])
                op = ("+", adder)
            elif "-" in raw_op:
                minuser = int(line.split("- ")[-1])
                op = ("-", minuser)
            else:
                raise ValueError
        elif "Test" in line:
            test = int(line.split(" ")[-1])
        elif "true" in line:
            true_index = int(line.split(" ")[-1])
        elif "false" in line:
            false_index = int(line.split(" ")[-1])

    monkeys[monkey_index] = dict(
        items=items, op=op, test=test, true_index=true_index, false_index=false_index
    )
    return monkeys


def process_monkeys(monkeys: dict, num_rounds: int = 20, divide_by_3: bool = False):
    num_items_per_monkey = defaultdict(int)
    max_modulo = reduce(
        lambda a, b: a * b, [monkey["test"] for monkey in monkeys.values()]
    )
    for _ in range(1, num_rounds + 1):
        for monkey_index in monkeys:
            monkey = monkeys[monkey_index]
            num_items_per_monkey[monkey_index] += len(monkey["items"])
            for item in monkey["items"]:
                if monkey["op"] == ("square",):
                    new_item = item * item
                else:
                    op, value = monkey["op"]
                    if op == "*":
                        new_item = item * value
                    elif op == "+":
                        new_item = item + value
                    elif op == "*":
                        new_item = item - value

                if divide_by_3:
                    new_item = int(new_item / 3)

                if new_item % monkey["test"] == 0:
                    monkeys[monkey["true_index"]]["items"].append(new_item % max_modulo)
                else:
                    monkeys[monkey["false_index"]]["items"].append(
                        new_item % max_modulo
                    )
            monkey["items"] = []

    a, b, *_ = sorted(num_items_per_monkey.values(), key=lambda x: -x)
    return a * b


if __name__ == "__main__":
    for file_name in ("example.txt", "input.txt"):
        with open(file_name) as f:
            lines = [l.rstrip("\n") for l in f.readlines()]

        monkeys = parse_monkeys(lines)
        part_1_answer = process_monkeys(monkeys, num_rounds=20, divide_by_3=True)

        monkeys = parse_monkeys(lines)
        part_2_answer = process_monkeys(monkeys, num_rounds=10000, divide_by_3=False)

        print(f"For {file_name}:")
        print(f"Part 1 answer: {part_1_answer}")
        print(f"Part 2 answer: {part_2_answer}")
