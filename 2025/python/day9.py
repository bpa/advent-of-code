#!/usr/bin/env python3

from aoc import *


def part1(input: str):
    farthest = 0
    tiles = []
    for line in input.splitlines():
        x, y = line.split(',')
        x, y = int(x), int(y)
        for a, b in tiles:
            dist = (abs(x - a)+1) * (abs(y - b)+1)
            farthest = max(farthest, dist)
        tiles.append((x, y))
    return farthest


def part2(input: str):
    tiles = []
    last_x = 0
    ya = 1_000_000_000
    yb = 0
    for line in input.splitlines():
        x, y = line.split(',')
        x, y = int(x), int(y)
        tiles.append((x, y))
        if last_x > 0 and abs(x - last_x) > 5000:
            ya = min(ya, y)
            yb = max(yb, y)
        last_x = x
    max_size = 0
    for i, (x, y) in enumerate(tiles[:-1]):
        for j, (a, b) in enumerate(tiles[i+1:]):
            x0 = min(x, a)
            x1 = max(x, a)
            y0 = min(y, b)
            y1 = max(y, b)

            if y0 < ya < y1 or y0 < yb < y1:
                continue
            for k, (xx, yy) in enumerate(tiles):
                if k == i or k == j:
                    continue
                if x0 < xx < x1 and y0 < yy < y1:
                    break
            else:
                dist = (abs(x - a)+1) * (abs(y - b)+1)
                max_size = max(max_size, dist)
    return max_size


if __name__ == '__main__':
    main()
