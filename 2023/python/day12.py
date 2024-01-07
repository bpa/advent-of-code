#!/usr/bin/env python3

from aoc import *
from functools import cache


@cache
def variants(records: str, groups):
    if not groups:
        if "#" in records:
            return 0
        else:
            return 1

    if len(records) < (sum(groups) + len(groups)) - 1:
        return 0

    if records.startswith("."):
        return variants(records[1:], groups)

    total = 0
    if records.startswith("?"):
        total += variants(records[1:], groups)

    if "." not in records[: groups[0]] and (
        len(records) <= groups[0] or records[groups[0]] != "#"
    ):
        total += variants(records[groups[0] + 1 :], groups[1:])
    return total


def part1(input: str):
    total = 0
    for line in input.splitlines():
        springs, groups = line.split()
        groups = tuple(int(g) for g in groups.split(","))
        x = variants(springs, groups)
        debug(x)
        total += x
    return total


def part2(input: str):
    total = 0
    for line in input.splitlines():
        (springs, groups) = line.split()
        groups = tuple(int(g) for g in (",".join([groups] * 5)).split(","))
        total += variants("?".join([springs] * 5), groups)
    return total


if __name__ == "__main__":
    main()
