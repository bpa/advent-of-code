#!/usr/bin/env python3

from aoc import *


class Monkey():
    def __init__(self, input):
        data = input.splitlines()
        self.items = delimit(data[1].split(': ')[1], ', ', int)
        self.op = data[2].split(' = ')[1]
        self.divisible = int(data[3].split()[-1])
        self.yes = int(data[4].split()[-1])
        self.no = int(data[5].split()[-1])
        self.inspected = 0


def solve(input, rounds):
    monkeys = [Monkey(b) for b in input.split('\n\n')]
    total_div = math.prod([m.divisible for m in monkeys])
    do_divide = rounds <= 20
    for _ in range(rounds):
        for m in monkeys:
            for old in m.items:
                m.inspected += 1
                level = eval(m.op) % total_div
                if do_divide:
                    level //= 3
                if level % m.divisible == 0:
                    monkeys[m.yes].items.append(level)
                else:
                    monkeys[m.no].items.append(level)
            m.items = []

    business = sorted([m.inspected for m in monkeys])
    return business[-1] * business[-2]


if __name__ == '__main__':
    main(20, 10000)
