#!/usr/bin/env python3

from aoc import *


def part1(input):
    line = 2000000
    coverage = []
    beacons_on_line = set()
    for (sx, sy, bx, by) in comb(input, r'-?\d+', int):
        if by == line:
            beacons_on_line.add(bx)
        d = abs(sx - bx) + abs(sy - by)
        i = d - abs(sy - line)
        if i >= 0:
            cover = [sx - i, sx + i]
            p = 0
            while p < len(coverage):
                c = coverage[p]
                if c[0] <= cover[0] <= c[1] or c[0] <= cover[1] <= c[1] or cover[0] <= c[1] <= cover[1]:
                    coverage.pop(p)
                    cover[0] = min(cover[0], c[0])
                    cover[1] = max(cover[1], c[1])
                else:
                    p += 1
            coverage.append(cover)
    return sum([x1 - x0 + 1 for x0, x1 in coverage]) - len(beacons_on_line)


def part2(input):
    end = 4000000
    points = []
    for (sx, sy, bx, by) in comb(input, r'-?\d+', int):
        points.append((sx, sy, abs(sx - bx) + abs(sy - by)))
    y = 0
    while y <= end:
        x = 0
        while x <= end:
            for sx, sy, d in points:
                dy = abs(sy - y)
                if abs(sx - x) + dy <= d:
                    x = sx + d - dy + 1
                    break
            else:
                return x * 4000000 + y
        y += 1


if __name__ == '__main__':
    main()
