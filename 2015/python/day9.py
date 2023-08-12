#!/usr/bin/env python3

from aoc import *


def do(input, ans, f):
    dist = Distance()
    for t in delimited(input):
        dist.set_dist(t[0], t[2], int(t[4]))
    table = dist.as_table()
    for path in itertools.permutations(range(len(table))):
        time = 0
        curr = path[0]
        for p in path:
            time += table[curr][p]
            curr = p
        ans = f(ans, time)
    return ans


def part1(input):
    return do(input, 1_000_000, min)


def part2(input):
    return do(input, 0, max)


if __name__ == "__main__":
    main()
