#!/usr/bin/env python3

from aoc import *

DIRS = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
]
DIR = "RDLU"


def solve(input: str, use_hex):
    x = 0
    y = 0
    area = 0
    perimiter = 0
    for d, l, hex in delimited(input):
        if use_hex:
            l = int(hex[2:-2], 16)
            d = int(hex[-2])
        else:
            l = int(l)
            d = DIR.index(d)
        (dx, dy) = DIRS[d]
        perimiter += l
        nx = x + dx * l
        ny = y + dy * l
        area += x * ny
        area -= y * nx
        x = nx
        y = ny

    return abs(area // 2) + perimiter // 2 + 1


if __name__ == "__main__":
    main(False, True)
