"""
Strategy Pattern
================

I prefer to use first class functions to implement Strategies because it makes
the code terser in general.
"""


def no_quack(): print("...")


def squeak(): print("squeak")


class NormalQuack:
    def __init__(self):
        self.cycle = ["quack", "quaaaak", "QUACK!"]
        self.idx = -1

    def do_quack(self):
        self.idx = (self.idx + 1) % len(self.cycle)
        print(self.cycle[self.idx])


class Duck:
    def __init__(self, quack_behavior=no_quack):
        self.quack_behavior = quack_behavior

    def quack(self):
        self.quack_behavior()


class RubberDuck(Duck):
    def __init__(self):
        super().__init__(squeak)


class AnconaDuck(Duck):
    def __init__(self):
        super().__init__(NormalQuack().do_quack)


if __name__ == '__main__':
    for duck in [AnconaDuck(), AnconaDuck(), RubberDuck()]:
        duck.quack()
        duck.quack()
        duck.quack()
        duck.quack()
