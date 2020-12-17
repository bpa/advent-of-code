#!/usr/bin/env python3

from collections import defaultdict

input = "../input/2020/day17.txt"

active = set()
with open(input) as file:
    y = 0
    for line in file.readlines():
        line.strip()
        for x, c in enumerate(line):
            if c == '#':
                active.add((x, y, 0))
        y += 1


def neighbors(cell):
    for x in range(-1, 2):
        for y in range(-1, 2):
            for z in range(-1, 2):
                yield (c[0] + x, c[1] + y, c[2] + z)


round = 0
while round < 6:
    next = set()
    candidates = defaultdict(lambda: 0)
    for c in active:
        for n in neighbors(c):
            candidates[n] += 1

    for c, n in candidates.items():
        if c in active:
            if 3 <= n <= 4:
                next.add(c)
        elif n == 3:
            next.add(c)

    active = next
    round += 1

print("Part 1:", len(active))

active = set()
with open(input) as file:
    y = 0
    for line in file.readlines():
        line.strip()
        for x, c in enumerate(line):
            if c == '#':
                active.add((x, y, 0, 0))
        y += 1


def neighbors_4d(cell):
    for x in range(-1, 2):
        for y in range(-1, 2):
            for z in range(-1, 2):
                for w in range(-1, 2):
                    yield (c[0] + x, c[1] + y, c[2] + z, c[3] + w)


round = 0
while round < 6:
    next = set()
    candidates = defaultdict(lambda: 0)
    for c in active:
        for n in neighbors_4d(c):
            candidates[n] += 1

    for c, n in candidates.items():
        if c in active:
            if 3 <= n <= 4:
                next.add(c)
        elif n == 3:
            next.add(c)

    active = next
    round += 1

print("Part 2:", len(active))
