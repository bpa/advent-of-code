#!/usr/bin/env python3

from aoc import *
# re.findall(exp, text)
# re.search() -> anywhere
# re.match -> beginning
# re.split(exp, text)
# re.compile(exp)


def part1():
    floor = puzzle.grid()
    count = 0
    while True:
        count += 1

        moving = []
        for c in floor:
            if c == '>':
                e = c.e()
                if e == None:
                    e = Point(floor, 0, c.y)
                if e == '.':
                    moving.append((c, e))
        for (c, e) in moving:
            c.set('.')
            e.set('>')

        southern = []
        for c in floor:
            if c == 'v':
                s = c.s()
                if s == None:
                    s = Point(floor, c.x, 0)
                if s == '.':
                    southern.append((c, s))
        for (c, s) in southern:
            c.set('.')
            s.set('v')

        if not moving and not southern:
            break

    return count


def part2():
    answer = 0
    for line in puzzle.lines():
        pass
    return answer


print("Part 1:", part1())
print("Part 2:", part2())
