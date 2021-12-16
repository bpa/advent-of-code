#!/usr/bin/env python3

from aoc import *


def build_chain(iterations):
    lines = puzzle.lines()
    template = next(lines)
    next(lines)

    rules = {}
    for line in lines:
        (p, l) = line.split(' -> ')
        rules[p] = l

    pairs = {}
    for k in rules.keys():
        pairs[k] = 0

    for p in windows(template, 2):
        pairs[p[0] + p[1]] = 1

    for _ in range(iterations):
        polymer = {k: 0 for k in rules.keys()}
        for (p, c) in pairs.items():
            (a, b) = list(p)
            i = rules[p]
            polymer[a + i] += c
            polymer[i + b] += c
        pairs = polymer

    letters = defaultdict(lambda: 0)
    omega = template[-1]
    letters[omega] += 1
    for (p, c) in pairs.items():
        letters[p[0]] += c

    counts = sorted([(v, k) for (k, v) in letters.items()])
    return int(counts[-1][0] - counts[0][0])


def part2():
    answer = 0
    for line in puzzle.lines():
        pass
    return answer


print("Part 1:", build_chain(10))
print("Part 2:", build_chain(40))
