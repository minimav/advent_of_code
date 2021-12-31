import dataclasses

from sympy import *


@dataclasses.dataclass
class NaiveALU:
    w: int = dataclasses.field(default=0, init=False)
    x: int = dataclasses.field(default=0, init=False)
    y: int = dataclasses.field(default=0, init=False)
    z: int = dataclasses.field(default=0, init=False)

    def valid(self):
        return self.z == 0

    def parse(self, value):
        if hasattr(self, value):
            return getattr(self, value)
        else:
            return int(value)

    def add(self, a, b):
        setattr(self, a, getattr(self, a) + b)

    def mul(self, a, b):
        setattr(self, a, getattr(self, a) * b)

    def div(self, a, b):
        setattr(self, a, int(getattr(self, a) / b))

    def mod(self, a, b):
        setattr(self, a, getattr(self, a) % b)

    def eql(self, a, b):
        setattr(self, a, int(getattr(self, a) == b))

    def process(self, inputs, commands):
        for method_name, *args in commands:
            if method_name == "inp":
                value, inputs = inputs[0], inputs[1:]
                setattr(self, args[0], int(value))
                continue

            a, b = args
            method = getattr(self, method_name)
            method(a, self.parse(b))


class BackwardALU:
    def __init__(self) -> None:
        self.w = symbols("w")
        self.x = symbols("x")
        self.y = symbols("y")
        self.z = symbols("z")
        self.expr = self.z

    def parse(self, value):
        if hasattr(self, value):
            return getattr(self, value)
        else:
            return int(value)

    def add(self, a, b):
        self.expr = self.expr.subs(a, a + b)

    def mul(self, a, b):
        self.expr = self.expr.subs(a, a * b)

    def div(self, a, b):
        self.expr = self.expr.subs(a, floor(a / b))

    def mod(self, a, b):
        self.expr = self.expr.subs(a, Mod(a, b))

    def eql(self, a, b):
        self.expr = self.expr.subs(a, Piecewise((1, Eq(a, b)), (0, True)))

    def process(self, commands):
        constants = []
        constant_index = 14
        for i, (method_name, *args) in enumerate(reversed(commands)):
            if method_name == "inp":
                sym_to_subs = getattr(self, args[0])
                constant = symbols(f"c_{constant_index}")
                constants.append(constant)
                constant_index -= 1
                self.expr = self.expr.subs(sym_to_subs, constant)
                continue

            a, b = args
            method = getattr(self, method_name)
            method(getattr(self, a), self.parse(b))

            self.expr = simplify(self.expr)
            print(i, method_name, args)
            print(self.expr)

        self.expr = self.expr.subs(self.x, 0)
        self.expr = self.expr.subs(self.y, 0)
        self.expr = self.expr.subs(self.z, 0)
        constants = list(reversed(constants))
        print(self.expr)


class ALU:
    def __init__(self) -> None:
        self.w = symbols("w")
        self.x = symbols("x")
        self.y = symbols("y")
        self.z = symbols("z")

        self.w_expr = self.w
        self.x_expr = self.y
        self.y_expr = self.x
        self.z_expr = self.z

    def parse(self, value):
        if hasattr(self, value):
            return getattr(self, value)
        else:
            return int(value)

    def add(self, a, b):
        attr = getattr(self, a)
        setattr(self, f"{a}_expr", getattr(self, f"{a}_expr").subs(attr, attr + b))

    def mul(self, a, b):
        attr = getattr(self, a)
        setattr(self, f"{a}_expr", getattr(self, f"{a}_expr").subs(attr, attr * b))

    def div(self, a, b):
        attr = getattr(self, a)
        setattr(
            self, f"{a}_expr", getattr(self, f"{a}_expr").subs(attr, floor(attr / b))
        )

    def mod(self, a, b):
        attr = getattr(self, a)
        setattr(self, f"{a}_expr", getattr(self, f"{a}_expr").subs(attr, Mod(attr, b)))

    def eql(self, a, b):
        attr = getattr(self, a)
        setattr(
            self,
            f"{a}_expr",
            getattr(self, f"{a}_expr").subs(
                attr, Piecewise((1, Eq(attr, b)), (0, True))
            ),
        )

    def process(self, commands):
        self.w_expr = self.w_expr.subs(self.w, 0)
        self.x_expr = self.x_expr.subs(self.x, 0)
        self.y_expr = self.y_expr.subs(self.y, 0)
        self.z_expr = self.z_expr.subs(self.z, 0)

        constants = []
        constant_index = 0
        for i, (method_name, *args) in enumerate(commands):
            if method_name == "inp":
                sym_to_subs = getattr(self, f"{args[0]}_expr")
                constant = symbols(f"c_{constant_index}")
                constants.append(constant)
                constant_index += 1
                setattr(self, sym_to_subs, constant)
                continue

            a, b = args
            method = getattr(self, method_name)
            method(a, self.parse(b))

            # self.expr = simplify(self.expr)
            print(i, method_name, args)
            print(self.w)
            print(self.x)
            print(self.y)
            print(self.z)


def naive(commands):
    model_number = 10 ** 15 - 1
    iteration = 1
    while model_number:
        model_number_str = str(model_number)
        if "0" in model_number_str:
            model_number -= 1
            iteration += 1
            continue

        alu = NaiveALU()
        alu.process(list(model_number_str), commands)
        if alu.valid():
            print(model_number)
        model_number -= 1
        iteration += 1


def main():
    with open("data/input_24.txt") as f:
        lines = [line.rstrip("\n") for line in f.readlines()]
        commands = [line.split() for line in lines]

    alu = ALU()
    alu.process(commands)


if __name__ == "__main__":

    main()
