#!/usr/bin/env python3

import itertools
from aoc import *

def can_fit(line, shapes):
    toks = line.split()
    x, y = [int(n) for n in toks[0][:-1].split('x')]
    size = x * y
    required = sum(a[0] * int(b) for a, b in zip(shapes, toks[1:]))
    if size < required:
        return False
    total = sum([int(s) for s in toks[1:]])
    squares = (x // 3) * (y // 3)
    if squares >= total:
        return True
    print("???")
    return True

def part1(input: str):
    total = 0
    shapes = []
    lines = iter(input.split("\n\n"))
    for x in lines:
        count = 0
        if x[1] == ':':
            shape = []
            for line in x.splitlines()[1:]:
                for v in line:
                    if (v == '#'):
                        count += 1
                    shape.append(v == '#')
            shapes.append((count, shape))
        else:
            for line in x.splitlines():
                if can_fit(line, shapes):
                    total += 1
    return total


def part2(input: str):
    return 0


if __name__ == '__main__':
    main()
