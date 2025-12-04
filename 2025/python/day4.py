#!/usr/bin/env python3

from aoc import *


def part1(input: str):
    total = 0
    g = grid(input, map=lambda a: a == '@')
    for r in g:
        if r.get():
            n = sum([a.get() for a in r.adjacent_8()])
            if n < 4:
                total += 1
    return total


def part2(input: str):
    total = 0
    g = grid(input, map=lambda a: a == '@')
    removed = set([1])
    while removed:
        removed.clear()
        for r in g:
            if r.get():
                n = sum([a.get() for a in r.adjacent_8()])
                if n < 4:
                    removed.add(r)
        for r in removed:
            r.set(False)
            total += 1
    return total


if __name__ == '__main__':
    main()
