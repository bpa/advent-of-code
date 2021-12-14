#!/usr/bin/env python3

from aoc import *


def part1():
    answer = 0
    octo = puzzle.grid(map=int)
    for i in range(100):
        flash = []
        for o in octo:
            e = o.get() + 1
            if e == 10:
                o.set(0)
                flash.append(o)
                answer += 1
            else:
                o.set(e)

        while flash:
            o = flash.pop()
            for n in o.adjacent_8():
                e = n.get() + 1
                if e == 10:
                    n.set(0)
                    flash.append(n)
                    answer += 1
                elif e > 1:
                    n.set(e)
    return answer


def part2():
    answer = 0
    octo = puzzle.grid(map=int)
    total = octo.width * octo.height
    while True:
        answer += 1

        flash = []
        flashed = 0
        for o in octo:
            e = o.get() + 1
            if e == 10:
                o.set(0)
                flash.append(o)
                flashed += 1
            else:
                o.set(e)

        while flash:
            o = flash.pop()
            for n in o.adjacent_8():
                e = n.get() + 1
                if e == 10:
                    n.set(0)
                    flash.append(n)
                    flashed += 1
                elif e > 1:
                    n.set(e)

        if flashed == total:
            return answer


print("Part 1:", part1())
print("Part 2:", part2())
