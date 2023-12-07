from collections import Counter


def determine_hand_type(counts: dict, strengths: dict) -> int:
    if len(counts) == 1:
        hand_type_value = 7
    elif len(counts) == 2:
        if max(counts.values()) == 4:
            # Four of a kind
            hand_type_value = 6
        else:
            # Full house case
            hand_type_value = 5
    elif len(counts) == 3:
        if max(counts.values()) == 3:
            # Three of a kind case
            hand_type_value = 4
        else:
            # Two pairs case
            hand_type_value = 3
    elif len(counts) == 4:
        # Single pair case
        hand_type_value = 2
    else:
        # High card case
        hand_type_value = 1
    return hand_type_value


def parse_hand_part_1(hand: str) -> tuple:
    strengths = {char: i for i, char in enumerate(reversed("AKQJT98765432"))}
    card_strengths = tuple(strengths[card] for card in hand)
    counts = Counter(card_strengths)
    hand_type_value = determine_hand_type(counts, strengths)
    return (hand_type_value,) + card_strengths


def parse_hand_part_2(hand: str) -> tuple:
    strengths = {char: i for i, char in enumerate(reversed("AKQT98765432J"))}
    card_strengths = tuple(strengths[card] for card in hand)
    counts = Counter(hand)

    # Deal with joker case
    if "J" in counts and len(counts) == 1:
        counts["A"] = 5
        del counts["J"]
    elif "J" in counts:
        max_non_j_count = max(
            [count for card, count in counts.items() if card != "J"],
        )
        cards_with_max_count = max(
            [card for card in counts if counts[card] == max_non_j_count and card != "J"]
        )
        # Can pick any here as only relevant for hand type ordering, not second phase
        max_non_j = cards_with_max_count[-1]
        counts[max_non_j] += counts["J"]
        del counts["J"]

    hand_type_value = determine_hand_type(counts, strengths)
    return (hand_type_value,) + card_strengths


def solve_puzzle(path: str, parser: callable):
    with open(path) as f:
        lines = f.readlines()
        hands_with_bids = []
        for line in lines:
            hand, bid, *_ = line.strip().split(" ")
            hands_with_bids.append((parser(hand), int(bid)))

        sorted_hands = sorted(hands_with_bids, key=lambda x: x[0])
        answer = 0
        for rank, (_, bid) in enumerate(sorted_hands, start=1):
            answer += bid * rank
        print(answer)


solve_puzzle("puzzle_7/example.txt", parse_hand_part_1)
solve_puzzle("puzzle_7/input.txt", parse_hand_part_1)
solve_puzzle("puzzle_7/example.txt", parse_hand_part_2)
solve_puzzle("puzzle_7/input.txt", parse_hand_part_2)
