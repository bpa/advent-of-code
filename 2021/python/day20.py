#!/usr/bin/env python3

from aoc import *
# re.findall(exp, text)
# re.search() -> anywhere
# re.match -> beginning
# re.split(exp, text)
# re.compile(exp)


def enhance(x0, y0, x1, y1, on, lookup):
    next = set()
    for y in range(y0, y1+1):
        for x in range(x0, x1+1):
            ind = 0
            for j in range(y-1, y+2):
                for i in range(x-1, x+2):
                    ind *= 2
                    if (i, j) in on:
                        ind += 1
            if lookup[ind] == '#':
                if x == x1:
                    if (x-1, y) in next:
                        next.add((x, y))
                else:
                    next.add((x, y))
    return next


def part1(times):
    g = puzzle.blocks()
    lookup = list(next(g))
    input = next(g).splitlines()
    (min_y, max_y) = (-5, len(input)+5)
    (min_x, max_x) = (-5, len(input[0])+5)
    on = set()
    for y in range(len(input)):
        for x in range(len(input[0])):
            if input[y][x] == '#':
                on.add((x, y))
    for i in range(times):
        if i % 2 == 0:
            min_x -= 2
            max_x += 2
            min_y -= 2
            max_y += 2
        on = enhance(min_x, min_y, max_x, max_y, on, lookup)
    return len(on)


def part2():
    answer = 0
    for line in puzzle.lines():
        pass
    return answer


print("Part 1:", part1(2))
print("Part 2:", part1(50))
