#!/usr/bin/env python3

from aoc import *


def move(d, x, y):
    if d == '<':
        x -= 1
    elif d == '>':
        x += 1
    elif d == '^':
        y -= 1
    elif d == 'v':
        y += 1
    return x, y


def part1(input):
    houses = set()
    (x, y) = (0, 0)
    houses.add((x, y))
    for d in input:
        (x, y) = move(d, x, y)
        houses.add((x, y))
    return len(houses)


def part2(input):
    houses = set()
    (sx, sy) = (0, 0)
    (rx, ry) = (0, 0)
    step = iter(input)
    while True:
        try:
            (sx, sy) = move(next(step), sx, sy)
            houses.add((sx, sy))
            (rx, ry) = move(next(step), rx, ry)
            houses.add((rx, ry))
        except:
            break
    return len(houses)


if __name__ == '__main__':
    main()
