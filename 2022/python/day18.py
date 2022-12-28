#!/usr/bin/env python3

from aoc import *


def part1(input):
    area = 0
    droplets = set()
    for a, b, c in comb(input, r'\d+', int):
        area += 6
        for n in range(-1, 2, 2):
            if (n+a, b, c) in droplets:
                area -= 2
            if (a, n+b, c) in droplets:
                area -= 2
            if (a, b, n+c) in droplets:
                area -= 2
        droplets.add((a, b, c))
    return area


def bubbles(droplets):
    xl = [f[0] for f in droplets]
    yl = [f[1] for f in droplets]
    zl = [f[2] for f in droplets]
    x0 = min(xl)
    x1 = max(xl)
    y0 = min(yl)
    y1 = max(yl)
    z0 = min(zl)
    z1 = max(zl)
    trapped = set()
    free = set()

    def cells(x0, x1, y0, y1, z0, z1):
        for x in range(x0, x1+1, 1):
            for y in range(y0, y1+1, 1):
                for z in range(z0, z1+1, 1):
                    yield x, y, z

    for p in cells(x0, x1, y0, y1, z0, z1):
        if p in trapped or p in free or p in droplets:
            continue
        queue = [p]
        is_free = False
        visited = set()
        while queue:
            o = queue.pop()
            if o in visited or o in droplets:
                continue
            else:
                visited.add(o)

            x, y, z = o
            if x0 > x or x1 < x or y0 > y or y1 < y or z0 > z or z1 < z:
                is_free = True
                break
            for n in range(-1, 2, 2):
                queue.append((n+x, y, z))
                queue.append((x, n+y, z))
                queue.append((x, y, n+z))

        queue = [p]
        visited = set()
        while queue:
            o = queue.pop()
            if o in visited or o in droplets:
                continue
            else:
                visited.add(o)
            x, y, z = o
            if x0 > x or x1 < x or y0 > y or y1 < y or z0 > z or z1 < z:
                continue
            if is_free:
                free.add(o)
            else:
                trapped.add(o)
            for n in range(-1, 2, 2):
                queue.append((n+x, y, z))
                queue.append((x, n+y, z))
                queue.append((x, y, n+z))

    return trapped


def part2(input):
    area = 0
    droplets = set()
    for a, b, c in comb(input, r'\d+', int):
        area += 6
        for n in range(-1, 2, 2):
            if (n+a, b, c) in droplets:
                area -= 2
            if (a, n+b, c) in droplets:
                area -= 2
            if (a, b, n+c) in droplets:
                area -= 2
        droplets.add((a, b, c))

    for a, b, c in bubbles(droplets):
        area += 6
        for n in range(-1, 2, 2):
            if (n+a, b, c) in droplets:
                area -= 2
            if (a, n+b, c) in droplets:
                area -= 2
            if (a, b, n+c) in droplets:
                area -= 2
        droplets.add((a, b, c))

    return area


if __name__ == '__main__':
    main()
