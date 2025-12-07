#!/usr/bin/env python3

from aoc import *


def part1(input: str):
    sum = 0
    for line in input.splitlines():
        ints = [int(c) for c in line]
        l = max(ints[:-1])
        i = line.index(str(l)) + 1
        r = max(ints[i:])
        sum += int(l) * 10 + r
    return sum


def part2(input: str):
    sum = 0
    for line in input.splitlines():
        joltage = 0
        start = 0
        ints = [int(c) for c in line]
        end = len(ints)
        for o in range(-11, 1):
            curr = ints[start:end+o]
            l = max(curr)
            joltage *= 10
            joltage += l
            start += curr.index(l) + 1
        sum += joltage
    return sum

if __name__ == '__main__':
    main()
