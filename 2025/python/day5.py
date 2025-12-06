#!/usr/bin/env python3

from aoc import *


def part1(input: str):
    lo = input.splitlines()
    lines = iter(lo)
    fresh = []
    while True:
        line = next(lines)
        if not line:
            break
        (a, b) = line.split('-')
        fresh.append((int(a), int(b)))
    total = 0
    for ingredient in lines:
        for (a, b) in fresh:
            if a <= int(ingredient) <= b:
                total += 1
                break
    return total
    

def part2(input: str):
    fresh = []
    for line in input.splitlines():
        if not line:
            break
        (a, b) = line.split('-')
        fresh.append([int(a), int(b)])

    total = 0
    fresh = sorted(fresh)
    ranges = iter(fresh)
    (a, b) = next(ranges)
    for (x, y) in ranges:
        if a <= x <= b:
            b = max(b, y)
        else:
            total += b - a + 1
            a = x
            b = y
    total += b - a + 1
    return total


if __name__ == '__main__':
    main()
