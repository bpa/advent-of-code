#!/usr/bin/env python3

from aoc import *
from heapq import heappop, heappush


def part1(input: str):
    steps = 64
    return 0
    garden = grid(input)
    start = garden.index("S")["S"]
    seen = set()
    possible = set()
    queue = [(0, start, steps & 1 == 0)]
    while queue:
        (d, n, take) = heappop(queue)
        seen.add(n)
        if take:
            possible.add(n)
        if d == steps:
            continue

        d = d + 1
        take = not take
        for x in n.adjacent_4():
            if x not in seen and x != "#":
                heappush(queue, (d, x, take))
    return len(possible)


DIRS = [(0, 1), (1, 0), (0, -1), (-1, 0)]


def walk(garden, reachable, all):
    possible = 0
    h = garden.height
    w = garden.width
    dist = Grid.of(w, h, False)
    queue = [(0, 65, 65, reachable)]
    while queue:
        (d, x, y, reachable) = heappop(queue)
        if dist.data[y][x]:
            continue

        dist.data[y][x] = True
        if reachable and (all or d > 65):
            possible += 1

        d = d + 1
        reachable = not reachable
        for dx, dy in DIRS:
            nx = x + dx
            ny = y + dy
            if 0 <= nx < w and 0 <= ny < h and garden.data[ny][nx] != "#":
                if not dist.data[ny][nx]:
                    heappush(queue, (d, nx, ny, reachable))
    return possible


def part2(input: str):
    steps = 26501365
    garden = grid(input)
    center = garden.width // 2
    x = (steps - center) // garden.width
    possible = 0
    print(x)
    possible = walk(garden, False, True) * (x + 1) * (x + 1)
    possible += walk(garden, True, True) * x * x
    possible += walk(garden, True, False) * x
    possible -= walk(garden, False, False) * (x + 1)
    return possible


if __name__ == "__main__":
    main()
