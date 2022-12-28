#!/usr/bin/env python3

from aoc import *


def points(input):
    return [int(i) for i in input.split(',')]


def disp(occupied, x0, x1, y):
    for yy in range(y+1):
        debug(
            ''.join(['#' if (x, yy) in occupied else '.' for x in range(x0, x1+1)]))


def rocks(input):
    low = 0
    occupied = set()
    for line in delimited(input, ' -> ', points):
        low = max(low, max([p[1] for p in line]))
        first = line[0]
        for p in line[1:]:
            dx = p[0] - first[0]
            if dx:
                for x in range(first[0], p[0], dx // abs(dx)):
                    occupied.add((x, p[1]))
            else:
                dy = p[1] - first[1]
                for y in range(first[1], p[1], dy//abs(dy)):
                    occupied.add((p[0], y))
            first = p
        occupied.add(tuple(p))
    return occupied, low


def part1(input):
    occupied, low = rocks(input)
    sand = 0
    p = (500, 0)
    while p[1] < low:
        for l in [(p[0], p[1]+1), (p[0]-1, p[1]+1), (p[0]+1, p[1]+1)]:
            if l not in occupied:
                p = l
                break
        else:
            sand += 1
            occupied.add(p)
            p = (500, 0)

    return sand


def part2(input):
    occupied, low = rocks(input)
    low += 1
    sand = 0
    p = (500, 0)
    while True:
        for l in [(p[0], p[1]+1), (p[0]-1, p[1]+1), (p[0]+1, p[1]+1)]:
            if p[1] < low and l not in occupied:
                p = l
                break
        else:
            sand += 1
            occupied.add(p)
            if p[1] == 0:
                break
            p = (500, 0)

    return sand


if __name__ == '__main__':
    main()
