#!/usr/bin/env python3

from aoc import *

most = {
    "red": 12,
    "green": 13,
    "blue": 14,
}


def part1(input: str):
    sums = 0
    for g, cubes in delimited(input, ": "):
        id = next(comb(g, "\d+", int))[0]
        try:
            for draw in cubes.split("; "):
                for cube in draw.split(", "):
                    n, color = cube.split()
                    if int(n) > most[color]:
                        raise "Nope"
            sums += id
        except:
            pass
    return sums


def part2(input: str):
    sums = 0
    for g, cubes in delimited(input, ": "):
        id = next(comb(g, "\d+", int))[0]
        maxs = {"red": 0, "green": 0, "blue": 0}
        for draw in cubes.split("; "):
            for cube in draw.split(", "):
                n, color = cube.split()
                maxs[color] = max(maxs[color], int(n))
        debug(maxs)
        x = 1
        for y in maxs.values():
            x *= y
        sums += x
    return sums


if __name__ == "__main__":
    main()

# 224
