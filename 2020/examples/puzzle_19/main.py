from pathlib import Path
from typing import List, Tuple


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

    def evaluate(messages: List[str], rule) -> List[str]:
        """Recursive evaluation of the rules on a message."""
        valid_messages = [message for message in messages if message]
        if not valid_messages:
            # still something to evaluate but message has been read
            return []

        next_messages = []
        if isinstance(rule, str):
            for message in valid_messages:
                if message[0] != rule:
                    continue
                if len(message) == 1:
                    raise ValueError
                else:
                    next_messages.append(message[1:])

        elif isinstance(rule, tuple):
            next_messages += evaluate(messages, rule[0])
            next_messages += evaluate(messages, rule[1])
        else:
            for rule_number in rule:
                messages = evaluate(messages, rules[rule_number])
                if not messages:
                    break

            next_messages = messages
        return next_messages

    try:
        evaluate([message], rules["0"])
        return False
    except ValueError:
        return True


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
    print(f"Part 1 {answer=}")
    answer = part_2(data)
    print(f"Part 2 {answer=}")
