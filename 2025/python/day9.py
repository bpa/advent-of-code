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
    y0 = 1_000_000_000
    y1 = 0
    for line in input.splitlines():
        x, y = line.split(',')
        x, y = int(x), int(y)
        tiles.append((x, y))
        if last_x > 0 and abs(x - last_x) > 5000:
            y0 = min(y0, y)
            y1 = max(y1, y)
            print(abs(x-last_x), last_x, x,y)
        last_x = x
    print(y0, y1)
        
    max_size = 0
    for i, (x, y) in enumerate(tiles[:-1]):
        for a, b in tiles[i+1:]:
            s = min(y, b)
            l = max(y, b)
            # if s < y0 < l or s < y1 < l:
            if (s < y0 and l == y0) or (s == y1 and l > y1):
                dist = (abs(x - a)+1) * (abs(y - b)+1)
                if dist > max_size and dist > 2984129596:
                    print(dist, y, b, s, l, y0, y1)
                max_size = max(max_size, dist)
    return max_size

# 4605538168
# 173627760 - too low
# 2996714391
# 3043170477

if __name__ == '__main__':
    main()
