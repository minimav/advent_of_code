import functools


def decode_hex(input):
    return "".join([f"{int(c, base=16):04b}" for c in input])


assert decode_hex("D2FE28") == "110100101111111000101000"
assert (
    decode_hex("EE00D40C823060")
    == "11101110000000001101010000001100100000100011000001100000"
)
assert (
    decode_hex("38006F45291200")
    == "00111000000000000110111101000101001010010001001000000000"
)


examples = {
    # "C200B40A82": 3,
    # "04005AC33890": 54,
    # "880086C3E88112": 7,
    # "CE00C43D881120": 9,
    # "D8005AC2A8F0": 1,
    # "F600BC2D8F": 0,
    # "9C005AC2F8F0": 0,
    "9C0141080250320F1802104A08": 1,
}


class Parser:
    def __init__(self, hex_data: str):
        self.hex_data = hex_data
        self.data = decode_hex(self.hex_data)
        self.index = 0
        self.expression = []
        self.num_bits_remaining = None
        self.num_packets_remaining = None

    def __len__(self):
        return len(self.data)

    def parse_length_type_0(self):
        """Parse an operator package of lenght type 0."""
        self.num_bits_remaining = int(self.data[self.index : self.index + 15], base=2)
        self.index += 15
        return self.parse_packet()

    def parse_length_type_1(self):
        """Parse an operator packet of length type 1."""
        self.num_packets_remaining = int(
            self.data[self.index : self.index + 11], base=2
        )
        self.index += 11
        return self.parse_packet()

    def parse_type_4_packet(self) -> int:
        """Parse a type 4 packet, containing a literal value."""
        binary_str = ""
        start_index = self.index
        while True:
            next_group = self.data[self.index : self.index + 5]
            binary_str += next_group[1:]
            self.index += 5
            if next_group[0] == "0":
                break

        literal = int(binary_str, base=2)
        self.expression.append(literal)

        if self.num_bits_remaining is not None:
            self.num_bits_remaining -= self.index - (start_index - 6)
        if self.num_packets_remaining is not None:
            self.num_packets_remaining -= 1

        if self.num_bits_remaining == 0:
            self.num_bits_remaining = None
            # self.expression.append(")")
        elif self.num_packets_remaining == 0:
            self.num_packets_remaining = None
            # self.expression.append(")")

        # only if finishing overall
        num_trailing_zeros = 8 - (self.index % 8)
        if self.index + num_trailing_zeros == len(self):
            assert all(
                bit == "0"
                for bit in self.data[self.index : self.index + num_trailing_zeros]
            )
            self.index += num_trailing_zeros

    def parse_operator_packet(self):
        """Parse an operator packet."""
        length_type_id = self.data[self.index]
        self.index += 1
        if length_type_id == "0":
            self.parse_length_type_0()
        else:
            self.parse_length_type_1()
        self.expression.append(")")

    def parse_packet(self):
        """Parse a packet."""
        packet_version = int(self.data[self.index : self.index + 3], base=2)
        packet_type_id = int(self.data[self.index + 3 : self.index + 6], base=2)
        self.index += 6

        if packet_type_id == 4:
            self.parse_type_4_packet()
        else:
            # operator packet
            self.expression.append("(")
            if packet_type_id == 0:
                self.expression.append("+")
            elif packet_type_id == 1:
                self.expression.append("*")
            elif packet_type_id == 2:
                self.expression.append("min")
            elif packet_type_id == 3:
                self.expression.append("max")
            elif packet_type_id == 5:
                self.expression.append(">")
            elif packet_type_id == 6:
                self.expression.append("<")
            elif packet_type_id == 7:
                self.expression.append("=")
            self.parse_operator_packet()

    def solve_expression(self):
        """Solve the expression."""
        while len(self.expression) > 3:
            end_index = 0
            while end_index < len(self.expression):
                if self.expression[end_index] != ")":
                    end_index += 1
                    continue

                start_index = end_index - 2
                while start_index >= 0:
                    if self.expression[start_index] != "(":
                        start_index -= 1
                        continue
                    else:
                        break
                break

            print(f"Reducing {self.expression[start_index + 1 : end_index]}")
            operator, *numbers = self.expression[start_index + 1 : end_index]
            if operator == "+":
                value = sum(numbers)
            elif operator == "*":
                value = functools.reduce(lambda a, b: a * b, numbers, 1)
            elif operator == "min":
                value = min(numbers)
            elif operator == "max":
                value = max(numbers)
            elif operator == "<":
                value = int(numbers[0] < numbers[1])
            elif operator == ">":
                value = int(numbers[0] > numbers[1])
            elif operator == "=":
                value = int(numbers[0] == numbers[1])

            new_expression = (
                self.expression[:start_index]
                + [value]
                + self.expression[end_index + 1 :]
            )
            print(
                f"Reduced {self.expression[start_index + 1 : end_index]} to {value}, {new_expression=}"
            )
            self.expression = new_expression

        return self.expression[0]

    def parse(self):
        """Parse all packets in a BITS representation."""
        while self.index < len(self.data):
            self.parse_packet()
        print(self.expression)
        return self.solve_expression()


def main():
    with open("data/input_16.txt", "r") as f:
        raw_data = f.read().rstrip("\n")

    for example, output in examples.items():
        parser = Parser(example)
        print(f"{'#' * len(example)}\n{example}\n{parser.data}\n{'#' * len(example)}")
        assert parser.parse() == output
        print()

    parser = Parser(raw_data)
    print(parser.parse())


if __name__ == "__main__":
    main()
