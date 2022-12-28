#!/usr/bin/env python3

from aoc import *


def priority(common):
    if common.isupper():
        return ord(common) - ord('A') + 27
    else:
        return ord(common) - ord('a') + 1


def part1(input: str):
    answer = 0
    for pack in input.splitlines():
        size = int(len(pack) / 2)
        common = (set(pack[:size]) & set(pack[size:])).pop()
        answer += priority(common)
    return answer


def part2(input):
    answer = 0
    elves = input.splitlines()
    for group in chunks(elves, 3):
        badge = reduce(
            lambda x, y: x & y, [set(p) for p in group]).pop()
        answer += priority(badge)
    return answer


if __name__ == '__main__':
    main()
