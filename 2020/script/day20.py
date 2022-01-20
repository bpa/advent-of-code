#!/usr/bin/env python3

import math
from collections import defaultdict
from pprint import pprint
from typing import List

input = "../input/2020/day20.txt"
# input = "test.txt"

mirror_map = [4, 7, 6, 5, 0, 3, 2, 1]


def get_orientation(o, dir):
    if o < 4:
        return (o + dir) % 4
    else:
        return (o + dir) % 4 + 4


class Tile:
    def __init__(self, file, offset):
        header = file.readline().strip()
        self.id = int(header[5:-1])
        self.offset = offset
        self.lines = [file.readline().strip() for _ in range(10)]
        self.signatures = [0] * 8
        self.orientation = None
        self.placed = False
        self.links = 0
        left = 1
        right = 1 << 9
        for i in range(10):
            self.signatures[0] += left if self.lines[0][i] == '#' else 0
            self.signatures[1] += left if self.lines[i][9] == '#' else 0
            self.signatures[2] += right if self.lines[9][i] == '#' else 0
            self.signatures[3] += right if self.lines[i][0] == '#' else 0
            self.signatures[4] += right if self.lines[0][i] == '#' else 0
            self.signatures[5] += left if self.lines[i][0] == '#' else 0
            self.signatures[6] += left if self.lines[9][i] == '#' else 0
            self.signatures[7] += right if self.lines[i][9] == '#' else 0
            left *= 2
            right //= 2

    def signature_on(self, dir):
        orientation = get_orientation(self.orientation, dir)
        return self.signatures[orientation]

    def mirror_on(self, dir):
        orientation = get_orientation(self.orientation, dir)
        mirror = mirror_map[orientation]
        return self.signatures[mirror]

    def __repr__(self):
        # return f'{self.id} {self.signatures}'
        return str(self.id)


tiles = []
with open(input) as file:
    while True:
        try:
            tile = Tile(file, len(tiles))
            tiles.append(tile)
            file.readline()
        except:
            break

matches = defaultdict(set)
for i, t in enumerate(tiles):
    for o, s in enumerate(t.signatures):
        matches[s].add((i, o))

for i, t in enumerate(tiles):
    neighbors = set()
    for s in t.signatures:
        neighbors.update({m[1] for m in matches[s] if m[0] != i})
    t.links = len(neighbors)

width = int(math.sqrt(len(tiles)))
end = width - 1
placed: List[List[Tile]] = [[None] * width for i in range(width)]


def stitch(candidates, x, y):
    global matches, placed
    for ind, o in candidates:
        t = tiles[ind]
        if t.placed:
            continue
        t.placed = True
        placed[x][y] = t
        t.orientation = o
        if y > 0:
            above = placed[x][y - 1]
            if above.mirror_on(2) != t.signature_on(0):
                t.placed = False
                continue

        if x == end:
            if y == end:
                finished = True
            else:
                signature = placed[0][y].mirror_on(2)
                finished = stitch(matches[signature], 0, y + 1)
        else:
            rotated = [(i, get_orientation(o, 1)) for i, o in matches[t.mirror_on(1)]]
            finished = stitch(rotated, x + 1, y)
        if finished:
            return True
        else:
            t.placed = False
    return False


stitch(((t.offset, o) for t in sorted(tiles, key=lambda t: t.links) for o in [0, 1, 2, 3]), 0, 0)
answer = math.prod([placed[x][y].id for x in [0, width - 1] for y in [0, width - 1]])
print("Part 1:", answer)
