#!/usr/bin/env python3

from aoc import *
from json import loads


def part1(input):
    neg = False
    val = 0
    total = 0
    for c in input:
        if c == "-":
            neg = True
            continue
        if c.isdigit():
            val *= 10
            val += int(c)
        else:
            if neg:
                total -= val
            else:
                total += val
            val = 0
            neg = False
    return total


def jsum(input):
    if isinstance(input, dict):
        values = list(input.values())
        if "red" in values:
            return 0
        else:
            return jsum(values)
    if isinstance(input, list):
        total = 0
        for e in input:
            total += jsum(e)
        return total
    if isinstance(input, int):
        return input
    return 0


def part2(input):
    return jsum(loads(input))


if __name__ == "__main__":
    main()
    # print(part2('[1,{"c":"red","b":2},3]'))
