#!/usr/bin/env python3

from aoc import *


def part1():
    cave = puzzle.grid(map=int)
    path = cave.shortest_path(cave.at(0, 0), cave.at(
        cave.width-1, cave.height-1), wall=[], vertex_cost=lambda a, b: b)
    return sum(path)


def part2():
    cave = []
    for l in puzzle.lines():
        row = [int(i) for i in list(l)]
        big_row = []
        for i in range(5):
            for c in row:
                n = c + i
                if n > 9:
                    n -= 9
                big_row.append(n)
        cave.append(big_row)

    big_cave = []
    for i in range(5):
        for row in cave:
            big_row = []
            for c in row:
                n = c + i
                if n > 9:
                    n -= 9
                big_row.append(n)
            big_cave.append(big_row)
    cave = Grid(big_cave)
    path = cave.shortest_path(Point(cave, 0, 0), Point(
        cave, cave.width-1, cave.height-1), wall=[], vertex_cost=lambda a, b: b)

    return sum(path)


print("Part 1:", part1())
print("Part 2:", part2())
