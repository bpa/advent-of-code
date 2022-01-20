#!/usr/bin/env python3

from collections import defaultdict
import math
import re
# re.findall(exp, text)
# re.search() -> anywhere
# re.match -> beginning
# re.split(exp, text)
# re.compile(exp)

dir = {
    'ne': (1, -1),
    'e': (1, 0),
    'se': (0, 1),
    'sw': (-1, 1),
    'w': (-1, 0),
    'nw': (0, -1)
}

input = "../input/2020/day24.txt"
# input = "test.txt"
answer = 0
black_tiles = set()
with open(input) as file:
    for line in file.readlines():
        i = 0
        q = 0
        r = 0
        line = line.strip()
        while i < len(line):
            if line[i] == 'n':
                i += 1
                if line[i] == 'e':
                    d = dir['ne']
                else:
                    d = dir['nw']
            elif line[i] == 's':
                i += 1
                if line[i] == 'e':
                    d = dir['se']
                else:
                    d = dir['sw']
            elif line[i] == 'e':
                d = dir['e']
            else:
                d = dir['w']
            i += 1
            q += d[0]
            r += d[1]
        if (q, r) in black_tiles:
            black_tiles.remove((q, r))
        else:
            black_tiles.add((q, r))

print("Part 1:", len(black_tiles))

for _ in range(100):
    neighbors = defaultdict(lambda: 0)
    for q, r in black_tiles:
        neighbors[(q+1, r-1)] += 1
        neighbors[(q+1, r)] += 1
        neighbors[(q, r+1)] += 1
        neighbors[(q, r-1)] += 1
        neighbors[(q-1, r+1)] += 1
        neighbors[(q-1, r)] += 1

    new_black = set()
    for k, v in neighbors.items():
        if k in black_tiles:
            if 0 < v < 3:
                new_black.add(k)
        elif v == 2:
            new_black.add(k)

    black_tiles = new_black

print("Part 2:", len(black_tiles))
