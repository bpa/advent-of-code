#!/usr/bin/env python3

from aoc import *


def part1(input, signals={}):
    instructions = [i.split() for i in input.splitlines()]
    left = len(instructions) - len(signals)

    def get(i):
        if i.isnumeric():
            return int(i)
        else:
            return signals.get(i, None)

    while left:
        for (i, inst) in enumerate(instructions):
            if inst[-1] in signals:
                continue
            if inst[1] == '->':
                src = get(inst[0])
                if src != None:
                    signals[inst[2]] = src
                    left -= 1
            elif inst[0] == 'NOT':
                src = get(inst[1])
                if src != None:
                    signals[inst[3]] = ~src & 0xFFFF
                    left -= 1
            else:
                a = get(inst[0])
                b = get(inst[2])
                if a == None or b == None:
                    continue
                if inst[1] == 'AND':
                    signals[inst[4]] = a & b
                elif inst[1] == 'OR':
                    signals[inst[4]] = a | b
                elif inst[1] == 'LSHIFT':
                    signals[inst[4]] = (a << b) & 0xFFFF
                elif inst[1] == 'RSHIFT':
                    signals[inst[4]] = (a >> b) & 0xFFFF
                else:
                    print("unknown instruction: ", inst)
                    return 0
                left -= 1
    return signals['a']


def part2(input):
    a = part1(input)
    return part1(input, {'b': a})


if __name__ == '__main__':
    main()
