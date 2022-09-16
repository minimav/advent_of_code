"""A Python example to help get the logic correct.

This is using example 1 and 2 data already converted into an
equivalent of the data structure that is used in the Rust code
(HashMap<String, Vec<Bag>> where Bag struct ~ Tuple[str, int] in
Python).

"""

child_bags_example_1 = {
    "shiny gold": [("dark olive", 1), ("vibrant plum", 2)],
    "dark olive": [("faded blue", 3), ("dotted_black", 4)],
    "vibrant plum": [("faded blue", 5), ("dotted_black", 6)],
}


child_bags_example_2 = {
    "shiny gold": [("dark red", 2)],
    "dark red": [("dark orange", 2)],
    "dark orange": [("dark yellow", 2)],
    "dark yellow": [("dark green", 2)],
    "dark green": [("dark blue", 2)],
    "dark blue": [("dark violet", 2)],
}


def compute_contained_bags(child_bags) -> int:
    """Compute the number of bags contained in a gold one.

    If we think of children using a tree structure we're essentially
    counting the number of nodes in the tree - 1 since we disregard
    the root node.
    """
    bags = [("shiny gold", 1)]
    answer = 0
    while bags:
        bag, acc = bags.pop()
        if bag not in child_bags:
            continue

        print(f"BAG: {bag}, accumulator {acc}")
        for other_bag, other_count in child_bags[bag]:
            answer += acc * other_count
            print(
                f"\t*{other_bag}, accumulator now {acc * other_count}, total={answer}"
            )
            bags.append((other_bag, acc * other_count))

    return answer


if __name__ == "__main__":
    print(f"Example 1: {compute_contained_bags(child_bags_example_1)}")
    print(f"Example 2: {compute_contained_bags(child_bags_example_2)}")
