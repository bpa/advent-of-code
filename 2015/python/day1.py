#!/usr/bin/env python3

from aoc import *


def part1(input):
    floor = 0
    for d in input:
        if d == '(':
            floor += 1
        elif d == ')':
            floor -= 1
    return floor


def part2(input):
    floor = 0
    for (i, d) in enumerate(input):
        if d == '(':
            floor += 1
        elif d == ')':
            floor -= 1
            if floor == -1:
                return i + 1
    return None


if __name__ == '__main__':
    main()
