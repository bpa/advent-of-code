#!/usr/bin/env python3

from aoc import *


def part1():
    answer = 0
    for p in puzzle.grid(map=int):
        if all(map(lambda n: n.get() > p.get(), p.adjacent_4())):
            answer += 1 + p.get()
    return answer


def fill(p):
    size = 0
    queue = [p]
    while queue:
        p = queue.pop()
        if p.get() < 9:
            size += 1
            p.set(9)
            queue.extend(p.adjacent_4())
    return size


def part2():
    basins = []
    for p in puzzle.grid(map=int):
        size = fill(p)
        if size > 0:
            basins.append(size)
    return reduce(lambda a, b: a * b, list(sorted(basins))[-3:])


print("Part 1:", part1())
print("Part 2:", part2())
