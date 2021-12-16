#!/usr/bin/env python3

from aoc import *
exp = re.compile('fold along (\w)=(\d+)')


def fold(line, dots, w, h):
    match = exp.match(line)
    if match:
        crease = int(match.group(2))
        if match.group(1) == 'x':
            for (x, y, dot) in dots.enumerate(crease, h):
                dot |= dots.get(crease * 2 - x, y)
            return (crease, h)
        else:
            for (x, y, dot) in dots.enumerate(w, crease):
                dot |= dots.get(x, crease * 2 - y)
            return (w, crease)


def solve():
    width = 2000
    height = 2000
    paper = Grid([[False]*width for i in range(height)])

    lines = iter(puzzle.lines())
    line = next(lines)
    width = 0
    height = 0
    while line:
        (x, y) = line.split(',')
        width = max(width, int(x) + 1)
        height = max(height, int(y) + 1)
        paper.set(int(x), int(y), True)
        line = next(lines)

    line = next(lines)
    (width, height) = fold(line, paper, width, height)
    count = paper.count(lambda a: a, width, height)

    print("Part 1: ", count)

    for line in lines:
        (width, height) = fold(line, paper, width, height)

    print("Part 2:")
    print(paper.to_string(width, height))


solve()
