#!/usr/bin/env python3

from re import L, X
from aoc import *
import re


def solve():
    # with open(sys.argv[1]) as file:
    #     q = re.search('x=(\\d+)..(\\d+).*y=(-?\\d+)..(-?\\d+)', file.read())
    # (x0, x1, y0, y1) = (int(v) for v in q.groups())
    debug("hi")
    (x0, x1, y0, y1) = puzzle.regex(
        'x=(\\d+)..(\\d+).*y=(-?\\d+)..(-?\\d+)', one_line=True)
    max_y = -y0 - 1
    start_x = int(math.sqrt(x0*2))
    print("Part 1:", int((y0+1) * y0 / 2))

    total = set()
    for y in range(y0, max_y+1):
        step = 0
        depth = 0
        y_prime = y
        while True:
            step += 1
            depth += y_prime
            y_prime -= 1
            if depth <= y1:
                break
        while y0 <= depth <= y1:
            for x in range(start_x, x1+1):
                x_prime = x
                dist = 0
                for _ in range(step):
                    dist += x_prime
                    x_prime -= 1
                    if x_prime < 0:
                        x_prime = 0
                if x0 <= dist <= x1:
                    total.add((x, y))
            step += 1
            depth += y_prime
            y_prime -= 1

    print("Part 2:", len(total))


solve()
