from collections import defaultdict

with open("input.txt") as f:
    lines = [l.rstrip("\n") for l in f.readlines()]

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

num_items_per_monkey = defaultdict(int)
num_rounds = 20
for round in range(1, num_rounds + 1):
    for monkey_index in monkeys:
        monkey = monkeys[monkey_index]
        num_items_per_monkey[monkey_index] += len(monkey["items"])
        for item in monkey["items"]:
            print(f"{monkey_index=} processing {item}")
            if monkey["op"] == ("square",):
                new_item = int(item * item / 3)
            else:
                op, value = monkey["op"]
                if op == "*":
                    new_item = int(item * value / 3)
                elif op == "+":
                    new_item = int((item + value) / 3)
                elif op == "*":
                    new_item = int((item - value) / 3)

            print(f"Testing {new_item=} with {monkey['test']}")
            if new_item % monkey["test"] == 0:
                print(f"Moving {new_item} to {monkey['true_index']}")
                monkeys[monkey["true_index"]]["items"].append(new_item)
            else:
                print(f"Moving {new_item} to {monkey['false_index']}")
                monkeys[monkey["false_index"]]["items"].append(new_item)
        monkey["items"] = []
    print(f"{round=}: {num_items_per_monkey=}")

print(num_items_per_monkey)
a, b, *_ = sorted(num_items_per_monkey.values(), key=lambda x: -x)
print(a * b)
