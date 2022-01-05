#!/usr/bin/env python3

from aoc import *


def part1():
    answer = 0
    blocks = puzzle.blocks()
    numbers = string.delimit(next(blocks), ',', int)

    boards = [string.grid(b, r('\s+'), int, "{0:3}") for b in blocks]
    return answer


def part2():
    answer = 0
    for line in puzzle.lines():
        pass
    return answer


print("Part 1:", part1())
print("Part 2:", part2())
