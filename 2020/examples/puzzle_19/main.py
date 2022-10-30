from pathlib import Path
from typing import List, Set, Tuple


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
                        tuple(raw_first_rule.split()),
                        tuple(raw_second_rule.split()),
                    )
                elif '"' in line:
                    # looks like '"a"' or '"b"', extract character
                    rules[rule_number] = raw_rule[1]
                else:
                    rules[rule_number] = tuple(raw_rule.split())
            elif not line:
                continue
            else:
                messages.append(line)
    return {"rules": rules, "messages": messages}


def find_valid_messages(rules, messages: Set[str], verbose: bool = False) -> Set[str]:
    """Iteratively find valid messages."""
    answers = set()
    max_len = max(len(m) for m in messages)

    def worker(possible_messages: Set[tuple], iteration=0):
        if verbose:
            print(f"{iteration=} {len(possible_messages)} to process")
        next_possible_messages = set()
        for partial_message, rule_numbers in possible_messages:
            rule_number, *remaining_rule_numbers = rule_numbers
            remaining_rule_numbers = tuple(remaining_rule_numbers)
            rule = rules[rule_number]
            if isinstance(rule, str):
                new_partial_message = partial_message + rule
                if len(new_partial_message) > max_len:
                    continue
                elif new_partial_message in messages:
                    if not remaining_rule_numbers:
                        answers.add(new_partial_message)
                    else:
                        next_possible_messages.add(
                            (new_partial_message, remaining_rule_numbers)
                        )
                elif not remaining_rule_numbers:
                    continue
                elif any(m.startswith(new_partial_message) for m in messages):
                    # only keep this if at least one message starts in this way
                    # need this to keep the possible messages list small enough
                    next_possible_messages.add(
                        (new_partial_message, remaining_rule_numbers)
                    )
            elif isinstance(rule[0], tuple):
                next_possible_messages.add(
                    (partial_message, rule[0] + remaining_rule_numbers)
                )
                next_possible_messages.add(
                    (partial_message, rule[1] + remaining_rule_numbers)
                )
            else:
                next_possible_messages.add(
                    (partial_message, rule + remaining_rule_numbers)
                )

        if not next_possible_messages:
            return
        return worker(possible_messages=next_possible_messages, iteration=iteration + 1)

    worker(possible_messages=[("", rules["0"])], iteration=0)
    return answers


def part_1(data: dict) -> int:
    valid_messages = find_valid_messages(
        rules=data["rules"], messages=set(data["messages"])
    )
    return len(valid_messages)


def part_2(data: dict) -> int:
    data["rules"]["8"] = (("42",), ("42", "8"))
    data["rules"]["11"] = (("42", "31"), ("42", "11", "31"))
    return part_1(data)


if __name__ == "__main__":
    data = parse_file(file_name="input.txt")
    answer = part_1(data)
    print(f"Part 1: {answer=}")
    answer = part_2(data)
    print(f"Part 2: {answer=}")
