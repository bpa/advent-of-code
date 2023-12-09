#!/usr/bin/env python3

from aoc import *


def part1(input: str):
    blocks = iter(input.split("\n\n"))
    seeds = comb(next(blocks), r"\d+", int, one_line=True)
    for entry in blocks:
        lines = iter(entry.splitlines())
        toks = next(lines).split()
        (f, t) = toks[0].split("-to-")
        mapping = []
        for line in lines:
            toks = [int(i) for i in line.split()]
            mapping.append((toks[1], toks[1] + toks[2] - 1, toks[0] - toks[1]))
        new_indexes = []
        for s in seeds:
            for m in mapping:
                if m[0] <= s <= m[1]:
                    new_indexes.append(s + m[2])
                    break
            else:
                new_indexes.append(s)
        seeds = new_indexes
    return min(seeds)


def part2(input: str):
    blocks = iter(input.split("\n\n"))
    ranges = iter(comb(next(blocks), r"\d+", int, one_line=True))
    seeds = []
    while True:
        try:
            start = next(ranges)
            num = next(ranges)
            seeds.append((start, start + num - 1))
        except StopIteration:
            break
    for entry in blocks:
        lines = iter(entry.splitlines())
        toks = next(lines).split()
        (f, t) = toks[0].split("-to-")
        mapping = []
        for line in lines:
            toks = [int(i) for i in line.split()]
            mapping.append((toks[1], toks[1] + toks[2] - 1, toks[0] - toks[1]))
        mapping.sort()
        new_ranges = []
        batch = iter(seeds)
        curr = next(batch)
        try:
            while curr:
                for m in mapping:
                    if m[0] <= curr[0] <= m[1]:
                        if curr[1] <= m[1]:
                            new_ranges.append((curr[0] + m[2], curr[1] + m[2]))
                            curr = next(batch)
                            break
                        else:
                            new_ranges.append((curr[0] + m[2], m[1] + m[2]))
                            curr = (m[1] + 1, curr[1])
                else:
                    new_ranges.append(curr)
                    curr = next(batch)
        except StopIteration:
            pass
        seeds = new_ranges
    return min(seeds)[0]


if __name__ == "__main__":
    main()
