#!/usr/bin/env python3

from aoc import *


class Tail():
    def __init__(self, tail):
        self.dx = 0
        self.dy = 0
        self.x = 0
        self.y = 0
        self.visited = set([(0, 0)])
        self.tail = tail

    def move(self, mv):
        for i in range(int(mv[2:])):
            if mv[0] == 'R':
                self._move(1, 0)
            elif mv[0] == 'L':
                self._move(-1, 0)
            elif mv[0] == 'U':
                self._move(0, 1)
            elif mv[0] == 'D':
                self._move(0, -1)

    def _move(self, x, y):
        self.dx += x
        self.dy += y
        if abs(self.dx) == 2 or abs(self.dy) == 2:
            dx = 0
            dy = 0
            if self.dx:
                dx = int(self.dx / abs(self.dx))
                self.dx -= dx
                self.x += dx
            if self.dy:
                dy = int(self.dy / abs(self.dy))
                self.dy -= dy
                self.y += dy
            if self.tail:
                self.tail._move(dx, dy)
        self.visited.add((self.x, self.y))


def part1(input):
    t = Tail(None)
    for line in input.splitlines():
        t.move(line)
    return len(t.visited)


def part2(input):
    t = Tail(None)
    h = t
    for i in range(8):
        h = Tail(h)
    for line in input.splitlines():
        h.move(line)
    return len(t.visited)


if __name__ == '__main__':
    main()
