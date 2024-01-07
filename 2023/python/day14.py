#!/usr/bin/env python3

from aoc import *


def part1(input: str):
    g = grid(input)
    total = 0
    for m in g:
        if m == "O":
            while x := m.n():
                if x == ".":
                    x.set("O")
                    m.set(".")
                else:
                    break
                m = x
    for m in g:
        if m == "O":
            total += g.height - m.y
    return total


def tilt(grid, d):
    for m in grid:
        if m == "O":
            while x := d(m):
                if x == ".":
                    x.set("O")
                    m.set(".")
                else:
                    break
                m = x


def part2(input: str):
    g = grid(input)
    total = 0
    seen = {}
    i = 0
    while i < 1_000_000_000:
        tilt(iter(g), Point.n)
        tilt(iter(g), Point.w)
        tilt(g.iter_reverse(), Point.s)
        tilt(g.iter_reverse(), Point.e)
        state = "".join(["".join(l) for l in g.data])
        s = seen.get(state)
        if s:
            diff = i - s
            rev = (1_000_000_000 - i) // diff
            i += diff * rev
        else:
            seen[state] = i
        i += 1
    for m in g:
        if m == "O":
            total += g.height - m.y
    return total


if __name__ == "__main__":
    main()
