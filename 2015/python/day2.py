#!/usr/bin/env python3

from aoc import *


def part1(input):
    total = 0
    for d in delimited(input, 'x', int):
        d = sorted(d)
        total += sum([2*a*b for (a, b) in itertools.combinations(d, 2)])
        total += d[0] * d[1]
    return total


def part2(input):
    total = 0
    for d in delimited(input, 'x', int):
        d = sorted(d)
        total += math.prod(d)
        total += 2 * (d[0] + d[1])
    return total


if __name__ == '__main__':
    main()
