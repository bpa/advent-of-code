#!/usr/bin/env python3

from aoc import *


def part1(input):
    lights = [[False for _ in range(1000)] for _ in range(1000)]
    for (cmd, *rng) in regex(input, r(r'(\D+) (\d+),(\d+) through (\d+),(\d+)')):
        (x0, y0, x1, y1) = (int(n) for n in rng)

        for y in range(y0, y1+1):
            for x in range(x0, x1+1):
                if cmd == 'toggle':
                    lights[y][x] = not lights[y][x]
                elif cmd == 'turn on':
                    lights[y][x] = True
                elif cmd == 'turn off':
                    lights[y][x] = False
    count = 0
    for row in lights:
        for l in row:
            if l:
                count += 1
    return count


def part2(input):
    lights = [[0 for _ in range(1000)] for _ in range(1000)]
    for (cmd, *rng) in regex(input, r(r'(\D+) (\d+),(\d+) through (\d+),(\d+)')):
        (x0, y0, x1, y1) = (int(n) for n in rng)

        for y in range(y0, y1+1):
            for x in range(x0, x1+1):
                if cmd == 'toggle':
                    lights[y][x] += 2
                elif cmd == 'turn on':
                    lights[y][x] += 1
                elif cmd == 'turn off':
                    if lights[y][x] > 0:
                        lights[y][x] -= 1

    return sum([sum(r) for r in lights])


if __name__ == '__main__':
    main()
