#!/usr/bin/env python3

import unittest
import os
from aoc import *
from aoc.puzzle import regex


def part1():
    cubes = set()
    for (io, x0, x1, y0, y1, z0, z1) in puzzle.regex('(\\w+) x=(-?\\d+)..(-?\\d+),y=(-?\\d+)..(-?\\d+),z=(-?\\d+)..(-?\\d+)'):
        x0 = max(int(x0), -50)
        x1 = min(int(x1), 50)
        y0 = max(int(y0), -50)
        y1 = min(int(y1), 50)
        z0 = max(int(z0), -50)
        z1 = min(int(z1), 50)
        for x in range(x0, int(x1)+1):
            for y in range(int(y0), int(y1)+1):
                for z in range(z0, int(z1)+1):
                    if io == 'on':
                        cubes.add((x, y, z))
                    else:
                        cubes.discard((x, y, z))
    return len(cubes)


def split_left(arr, two, a, b):
    new = []
    for cube in arr:
        c = cube[:]
        c[b] = two[b]
        new.append(c)
        c = cube[:]
        c[a] = two[b] + 1
        new.append(c)
    return new


def split_center(arr, two, a, b):
    new = []
    for cube in arr:
        c = cube[:]
        c[b] = two[a] - 1
        new.append(c)
        c = cube[:]
        c[a] = two[a]
        c[b] = two[b]
        new.append(c)
        c = cube[:]
        c[a] = two[b] + 1
        new.append(c)
    return new


def split_right(arr, two, a, b):
    new = []
    for cube in arr:
        c = cube[:]
        c[b] = two[a] - 1
        new.append(c)
        c = cube[:]
        c[a] = two[a]
        new.append(c)
    return new


def break_up(one, two):
    pieces = [one[:]]
    for axis in range(3):
        a = axis * 2
        b = axis * 2 + 1
        if one[a] < two[a] <= two[b] < one[b]:
            pieces = split_center(pieces, two, a, b)
        elif one[a] < two[a] <= one[b]:
            pieces = split_right(pieces, two, a, b)
        elif one[a] <= two[b] < one[b]:
            pieces = split_left(pieces, two, a, b)
    return pieces


def score(regions):
    on = 0
    for (x0, x1, y0, y1, z0, z1) in regions:
        on += (x1 - x0 + 1) * (y1 - y0 + 1) * (z1 - z0 + 1)
    return on


def add_region(regions, io, region):
    areas = [region]
    while areas:
        (x0, x1, y0, y1, z0, z1) = areas.pop()
        for (i, (X0, X1, Y0, Y1, Z0, Z1)) in enumerate(regions):
            if x0 <= X0 and x1 >= X1 and y0 <= Y0 and y1 >= Y1 and z0 <= Z0 and z1 >= Z1:
                regions.pop(i)
                areas.append((x0, x1, y0, y1, z0, z1))
                break
            elif x0 >= X0 and x1 <= X1 and y0 >= Y0 and y1 <= Y1 and z0 >= Z0 and z1 <= Z1:
                if io == 'off':
                    regions.pop(i)
                    regions.extend(
                        break_up([X0, X1, Y0, Y1, Z0, Z1],
                                 [x0, x1, y0, y1, z0, z1]))
                    areas.append((x0, x1, y0, y1, z0, z1))
                break
            elif not(x0 > X1 or x1 < X0 or y0 > Y1 or y1 < Y0 or z0 > Z1 or z1 < Z0):
                if io == 'on':
                    areas.extend(break_up([x0, x1, y0, y1, z0, z1],
                                          [X0, X1, Y0, Y1, Z0, Z1]))
                else:
                    regions.pop(i)
                    regions.extend(
                        break_up([X0, X1, Y0, Y1, Z0, Z1], [x0, x1, y0, y1, z0, z1]))
                    areas.append((x0, x1, y0, y1, z0, z1))
                break
        else:
            if io == 'on':
                regions.append((x0, x1, y0, y1, z0, z1))


def part2():
    regions = []
    for (io, *coords) in puzzle.regex('(\\w+) x=(-?\\d+)..(-?\\d+),y=(-?\\d+)..(-?\\d+),z=(-?\\d+)..(-?\\d+)'):
        add_region(regions, io, (int(c) for c in coords))
    return score(regions)


class AocTest(unittest.TestCase):
    def test_all(self):
        regions = []
        add_region(regions, 'on', (10, 12, 10, 12, 10, 12))
        self.assertEqual(27, score(regions))
        add_region(regions, 'off', (11, 11, 11, 11, 11, 11))
        self.assertEqual(26, score(regions))
        self.assertEqual(26, len(regions))

        add_region(regions, 'on', (10, 12, 10, 12, 10, 12))
        self.assertEqual(27, score(regions))
        self.assertEqual(1, len(regions))
        add_region(regions, 'on', (11, 11, 11, 11, 11, 11))
        self.assertEqual(27, score(regions))
        self.assertEqual(1, len(regions))

        add_region(regions, 'on', (11, 13, 10, 12, 10, 12))
        self.assertEqual(36, score(regions))
        self.assertEqual(2, len(regions))

        add_region(regions, 'off', (11, 13, 10, 12, 10, 12))
        self.assertEqual(9, score(regions))
        self.assertEqual(1, len(regions))

        add_region(regions, 'on', (10, 12, 10, 12, 10, 12))
        add_region(regions, 'off', (12, 15, 11, 11, 11, 11))
        self.assertEqual(17, len(regions))
        self.assertEqual(26, score(regions))


if __name__ == '__main__':
    import sys
    test = unittest.main(argv=sys.argv[:1], exit=False)
    if test.result.wasSuccessful():
        print("Part 1:", part1())
        print("Part 2:", part2())
