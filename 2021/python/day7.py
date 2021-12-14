#!/usr/bin/env python3

from aoc import *


def part1():
    crabs = puzzle.pandas_line(',', int)
    median = crabs.median()
    return abs(crabs - median).sum()


def part2():
    crabs = puzzle.pandas_line(',', int)
    mean = crabs.mean()
    a = abs(crabs - int(mean))
    b = abs(crabs - ceil(mean))
    return min((a * (a + 1) / 2).sum(), (b * (b + 1) / 2).sum())


print("Part 1:", part1())
print("Part 2:", part2())
