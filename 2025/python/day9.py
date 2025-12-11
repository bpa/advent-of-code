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

def is_clear(t, i, j, x1, x2, y1, y2, tiles):
    for k, (x, y) in enumerate(tiles):
        if k == i or k == j:
            continue
        if x1 < x < x2 and y1 < y < y2:
            l = abs(i - k)
            m = abs(j - k)
            if l != 1 and m != 1 and l != t and m != t:
                return False
    return True

def part2(input: str):
    tiles = []
    for line in input.splitlines():
        x, y = line.split(',')
        x, y = int(x), int(y)
        tiles.append((x, y))
    max_size = 0
    ys = set()
    t = len(tiles) - 1
    last_y = 0
    y_count = 0
    max_d = 0
    max_i = 0
    for i, j in itertools.batched(tiles, 2):
        d = abs(i[0] - j[0]) + abs(i[1] - j[1])
        if d > max_d:
            max_d = d
            max_i = tiles.index(i)
    print("Max diff:", max_d, "at", tiles[max_i], tiles[max_i+1])
    print("Ys:", ys)
    for i, (x, y) in enumerate(tiles[:-1]):
        for j in range(i+1, len(tiles)):
            a, b = tiles[j]
            if i != 4:
                continue
            if is_clear(t, i, j, min(x, a), max(x, a), min(y, b), max(y, b), tiles):
                size = (abs(x - a)+1) * (abs(y - b)+1)
                max_size = max(max_size, size)
    return max_size

# 4605538168
# 173627760 - too low

if __name__ == '__main__':
    main()
