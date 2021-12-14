#!/usr/bin/env python3

from aoc import *

def part1():
    answer = 1
    for x in puzzle.regex(r'(\d+),(\d+) -> (\d+),(\d+)'):
        debug(x)
    return answer


def part2():
    answer = 2
    with open(sys.argv[1]) as file:
        for line in file.readlines():
            pass
    return answer


print("Part 1:", part1())
print("Part 2:", part2())
