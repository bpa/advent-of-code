#!/usr/bin/env python3

from aoc import *


def part1(input):
    size = 1000000
    target = int(input)
    presents = [10] * size
    for x in range(2, size):
        for y in range(x, size, x):
            presents[y] += x * 10
        if presents[x] >= target:
            return x

    return 1


def part2(input):
    size = 1000000
    target = int(input)
    presents = [11] * size
    for x in range(2, size):
        y = 0
        for _ in range(50):
            y += x
            if y >= size:
                break
            presents[y] += x * 11
        if presents[x] >= target:
            return x

    return 1


if __name__ == "__main__":
    main()
