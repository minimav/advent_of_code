from dataclasses import dataclass, field
import functools
import typing as tp


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
    "D2FE28": 6,
    "38006F45291200": 9,
    "EE00D40C823060": 14,
    "8A004A801A8002F478": 16,
    "620080001611562C8802118E34": 12,
    "C0015000016115A2E0802F182340": 23,
    "A0016C880162017C3686B18A3D4780": 31,
}


class Parser:
    def __init__(self, hex_data: str):
        self.hex_data = hex_data
        self.data = decode_hex(self.hex_data)
        self.index = 0
        self.answer = 0
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

    def parse_type_4_packet(self):
        """Parse a type 4 packet, containing a literal value."""
        binary_str = ""
        start_index = self.index
        while True:
            next_group = self.data[self.index : self.index + 5]
            binary_str += next_group[1:]
            self.index += 5
            if next_group[0] == "0":
                break

        if self.num_bits_remaining is not None:
            self.num_bits_remaining -= self.index - (start_index - 6)
        if self.num_packets_remaining is not None:
            self.num_packets_remaining -= 1

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

    def parse_packet(self):
        """Parse a packet."""
        packet_version = int(self.data[self.index : self.index + 3], base=2)
        packet_type_id = int(self.data[self.index + 3 : self.index + 6], base=2)
        self.index += 6
        self.answer += packet_version

        if packet_type_id == 4:
            self.parse_type_4_packet()
        else:
            self.parse_operator_packet()

    def parse(self):
        """Parse all packets in a BITS representation."""
        while self.index < len(self.data):
            self.parse_packet()
        return self.answer


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
