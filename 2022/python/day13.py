#!/usr/bin/env python3

from aoc import *


def cmp(a, b):
    if isinstance(a, list):
        if isinstance(b, list):
            for (l, r) in zip(a, b):
                c = cmp(l, r)
                if c != 0:
                    return c
            return len(a) - len(b)
        else:
            return cmp(a, [b])
    elif isinstance(b, list):
        return cmp([a], b)
    else:
        return a - b


def part1(input):
    sum = 0
    for i, packets in enumerate(input.split('\n\n')):
        (a, b) = [eval(line) for line in packets.splitlines()]
        if cmp(a, b) <= 0:
            sum += i + 1
    return sum


def part2(input):
    packets = input.splitlines()
    packets.extend(['[[2]]', '[[6]]'])
    packets = [eval(line) for line in filter(None, packets)]
    packets.sort(key=cmp_to_key(cmp))
    one = packets.index([[2]])
    two = packets.index([[6]])
    return (one+1)*(two+1)


if __name__ == '__main__':
    main()
