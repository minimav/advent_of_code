from pathlib import Path
from typing import Tuple


def parse_file(file_name: str = "input.txt") -> dict:
    """Parse the rules and messages from the input file."""
    with open(Path("examples") / "puzzle_19" / file_name) as f:
        rules = {}
        messages = []
        for line in f.readlines():
            line = line.rstrip("\n")
            if ":" in line:
                rule_number, raw_rule, *_ = line.split(":")
                raw_rule = raw_rule.lstrip()
                if "|" in raw_rule:
                    raw_first_rule, raw_second_rule, *_ = raw_rule.split("|")
                    rules[rule_number] = (
                        raw_first_rule.split(),
                        raw_second_rule.split(),
                    )
                elif '"' in line:
                    # looks like '"a"' or '"b"', extract character
                    rules[rule_number] = raw_rule[1]
                else:
                    rules[rule_number] = raw_rule.split()
            elif not line:
                continue
            else:
                messages.append(line)
    return {"rules": rules, "messages": messages}


def evaluate_message(message: str, rules: dict) -> bool:
    """Evaluate rules on a message."""

    def evaluate(message: str, rule) -> Tuple[str, bool]:
        """Recursive evaluation of the rules on a message."""
        if isinstance(rule, str):
            if message[0] == rule:
                return (message[1:], True)
            else:
                return (message, False)
        elif isinstance(rule, tuple):
            message_1, valid_1 = evaluate(message, rule[0])
            message_2, valid_2 = evaluate(message, rule[1])
            if valid_1 and valid_2:
                assert message_1 == message_2
                return (message_1, valid_1)
            elif valid_1:
                return (message_1, valid_1)
            elif valid_2:
                return (message_2, valid_2)
            else:
                return (message_1, False)
        else:
            for rule_number in rule:
                message, valid = evaluate(message, rules[rule_number])
                if not valid:
                    # sequence is an AND condition, so quit early
                    return (message, valid)
            return (message, True)

    final_message, valid = evaluate(message, rules["0"])
    return not final_message and valid


def part_1(data: dict) -> int:
    answer = 0
    for message in data["messages"]:
        if evaluate_message(message, data["rules"]):
            answer += 1
    return answer


def part_2(data: dict) -> int:
    # 8: 42 | 42 8
    # 11: 42 31 | 42 11 31
    data["rules"]["8"] = (["42"], ["42", "8"])
    data["rules"]["11"] = (["42", "31"], ["42", "11", "31"])
    answer = 0
    for message in data["messages"]:
        if evaluate_message(message, data["rules"]):
            answer += 1
    return answer


if __name__ == "__main__":
    data = parse_file(file_name="input.txt")
    answer = part_1(data)
    print(f"{answer=}")
    answer = part_2(data)
    print(f"{answer=}")
