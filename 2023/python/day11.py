#!/usr/bin/env python3

from aoc import *


def part1(input: str):
    g = grid(input)
    rows = set()
    cols = set()
    for p in g:
        if p == "#":
            rows.add(p.y)
            cols.add(p.x)
    for y in range(g.height - 1, -1, -1):
        if not y in rows:
            g.height += 1
            g.data.insert(y, ["."] * g.width)
    for x in range(g.width - 1, -1, -1):
        if not x in cols:
            g.width += 1
            for row in g.data:
                row.insert(x, ".")

    points = filter(lambda p: p == "#", g)
    total = 0
    for a, b in itertools.combinations(points, 2):
        total += a.manhattan_distance(b)

    return total


def part2(input: str):
    g = grid(input)
    rows = set()
    cols = set()
    for p in g:
        if p == "#":
            rows.add(p.y)
            cols.add(p.x)

    points = filter(lambda p: p == "#", g)
    total = 0
    for a, b in itertools.combinations(points, 2):
        total += a.manhattan_distance(b)
        r = sorted([a.x, b.x])
        for i in range(*r):
            if i not in cols:
                total += 999_999
        r = sorted([a.y, b.y])
        for i in range(*r):
            if i not in rows:
                total += 999_999

    return total


if __name__ == "__main__":
    main()
