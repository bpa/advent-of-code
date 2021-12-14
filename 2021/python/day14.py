#!/usr/bin/env python3

from aoc import *


def part1():
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

    alpha = template[0]
    omega = template[-1]

    for _ in range(40):
        polymer = {k: 0 for k in rules.keys()}
        for (p, c) in pairs.items():
            (a, b) = list(p)
            i = rules[p]
            polymer[a + i] += c
            polymer[i + b] += c
        pairs = polymer

    letters = defaultdict(lambda: 0)
    for (p, c) in pairs.items():
        (a, b) = list(p)
        letters[a] += c
        letters[b] += c

    letters[alpha] += 1
    letters[omega] += 1
    for k in letters.keys():
        letters[k] /= 2

    counts = sorted([(v, k) for (k, v) in letters.items()])
    debug(counts)
    return counts[-1][0] - counts[0][0]


def part2():
    answer = 0
    for line in puzzle.lines():
        pass
    return answer


print("Part 1:", part1())
print("Part 2:", part2())
