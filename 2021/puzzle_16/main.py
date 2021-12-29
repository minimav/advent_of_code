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


@dataclass
class State:
    depth: int = 0
    index: int = 0
    index_per_depth: dict = field(default_factory=lambda: {0: 0})
    type_id: tp.Optional[int] = None
    num_packets_remaining_per_depth: dict = field(default_factory=dict)
    num_bits_remaining_per_depth: dict = field(default_factory=dict)
    answer: int = 0
    output: tp.List[int] = field(default_factory=list)
    log: str = field(default="", repr=False)

    def append_log(self, s):
        self.log += f"{s}\n"


def print_state(func):
    @functools.wraps(func)
    def wrapped(*args, **kwargs):
        _, state, *_ = args
        state.append_log(f"* Call {func.__name__}, {state}")
        return func(*args, **kwargs)

    return wrapped


@print_state
def parse_type_4_packet(data: str, state: State):
    binary_str = ""
    start_index = state.index

    while True:
        next_group = data[state.index : state.index + 5]
        binary_str += next_group[1:]
        state.index += 5
        if next_group[0] == "0":
            break

    if state.depth in state.num_packets_remaining_per_depth:
        assert state.depth > 0
        state.num_packets_remaining_per_depth[state.depth] -= 1
        if state.num_packets_remaining_per_depth[state.depth] == 0:
            del state.num_packets_remaining_per_depth[state.depth]
            del state.index_per_depth[state.depth]
            state.depth -= 1

    if state.depth in state.num_bits_remaining_per_depth:
        assert state.depth > 0
        state.num_bits_remaining_per_depth[state.depth] -= state.index - start_index + 6
        if state.num_bits_remaining_per_depth[state.depth] == 0:
            del state.num_bits_remaining_per_depth[state.depth]
            del state.index_per_depth[state.depth]
            state.depth -= 1

    # add trailing 0s so packet has length % 8 = 0
    if state.depth == 0:
        state.index += 8 - ((state.index - state.index_per_depth[0]) % 8)
        state.index_per_depth[0] = state.index

    assert state.depth >= 0
    state.output.append(int(binary_str, base=2))
    return state


@print_state
def parse_length_type_0(data: str, state: State):
    bits_in_packet = int(data[state.index : state.index + 15], base=2)
    state.num_bits_remaining_per_depth[state.depth] = bits_in_packet
    state.index += 15
    return parse_packet(data, state)


@print_state
def parse_length_type_1(data: str, state: State):
    num_packets = int(data[state.index : state.index + 11], base=2)
    state.num_packets_remaining_per_depth[state.depth] = num_packets
    state.index += 11
    return parse_packet(data, state)


@print_state
def parse_operator_packet(data: str, state: State):
    length_type_id = data[state.index]
    state.index += 1
    state.depth += 1
    state.index_per_depth[state.depth] = state.index
    if length_type_id == "0":
        return parse_length_type_0(data, state)
    else:
        return parse_length_type_1(data, state)


@print_state
def parse_packet(data: str, state: State):
    packet_version = int(data[state.index : state.index + 3], base=2)
    packet_type_id = int(data[state.index + 3 : state.index + 6], base=2)
    state.append_log(f"# parsing {packet_version=}, {packet_type_id=}")
    state.index += 6
    state.type_id = packet_type_id
    state.answer += packet_version

    if packet_type_id == 4:
        state = parse_type_4_packet(data, state)
    else:
        state = parse_operator_packet(data, state)
    return state


def parse_packets(data, state):
    try:
        while state.index < len(data):
            state = parse_packet(data, state)
        return state
    except:
        print(state.log)
        raise


def main():
    with open("data/input_16.txt", "r") as f:
        raw_data = f.read().rstrip("\n")

    for example, output in examples.items():
        data = decode_hex(example)
        print(f"{'#' * len(example)}\n{example}\n{data}\n{'#' * len(example)}")
        initial_state = State()
        final_state = parse_packets(data, initial_state)
        print(f"{final_state=}")
        assert final_state.answer == output
        print()

    initial_state = State()
    final_state = parse_packets(decode_hex(raw_data), initial_state)
    print(final_state)


if __name__ == "__main__":
    main()
