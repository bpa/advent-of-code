#!/usr/bin/env python3

from aoc import *


def part1(input):
    lights = grid(input, map=lambda x: 0 if x == "." else 1)
    for _ in range(4):
        next = lights.clone()
        for a, b in zip(lights, next):
            on = sum(a.adjacent_8())
            if on == 3 or (on == 2 and a):
                b.set(1)
            else:
                b.set(0)
        lights = next
    return sum(lights)


def part2(input):
    lights = grid(input, map=lambda x: 0 if x == "." else 1)
    lights.set(0, 0, 1)
    lights.set(0, lights.width - 1, 1)
    lights.set(lights.height - 1, 0, 1)
    lights.set(lights.height - 1, lights.width - 1, 1)
    for _ in range(100):
        next = lights.clone()
        for a, b in zip(lights, next):
            on = sum(a.adjacent_8())
            if on == 3 or (on == 2 and a):
                b.set(1)
            else:
                b.set(0)
        next.set(0, 0, 1)
        next.set(0, next.width - 1, 1)
        next.set(next.height - 1, 0, 1)
        next.set(next.height - 1, next.width - 1, 1)
        lights = next
    return sum(lights)


if __name__ == "__main__":
    main()
