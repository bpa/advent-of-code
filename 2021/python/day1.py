#!/usr/bin/env python3

from aoc import *


def part1():
    answer = 0
    last = 1000000
    for depth in puzzle.lines(int):
        if depth > last:
            answer += 1
        last = depth
    return answer


def part2():
    answer = 0
    last = 10000000
    for depth in [sum(w) for w in windows(puzzle.lines(int), 3)]:
        if depth > last:
            answer += 1
        last = depth
    return answer


print("Part 1:", part1())
print("Part 2:", part2())
