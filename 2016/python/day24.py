#!/usr/bin/env python3

from aoc import *


def part1():
    answer = 0
    map = puzzle.grid()
    points = map.index(*list('1234567890'))
    debug(points)
    return answer


def part2():
    answer = 0
    for line in puzzle.lines():
        pass
    return answer


print("Part 1:", part1())
print("Part 2:", part2())
