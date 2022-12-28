#!/usr/bin/env python3

from aoc import *


class Elf:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.hash = hash((x, y))

    def __repr__(self):
        return f'E[{self.x},{self.y}]'

    def __eq__(self, other):
        if isinstance(other, Elf):
            return self.x == other.x and self.y == other.y
        else:
            return self.x == other[0] and self.y == other[1]

    def __hash__(self):
        return self.hash

    def is_alone(self, locations):
        for y in range(self.y-1, self.y+2):
            for x in range(self.x-1, self.x+2):
                l = locations.get((x, y))
                if l and l != self:
                    return False
        return True

    def proposal(self, locations, options):
        if self.is_alone(locations):
            return None

        for p in options:
            for d in p:
                if (self.x+d[0], self.y+d[1]) in locations:
                    break
            else:
                return (self.x+p[1][0], self.y+p[1][1])
        else:
            return (self.x, self.y)

    def move(self, point):
        self.hash = hash(point)
        (self.x, self.y) = point


def part1(input):
    options = [
        [(-1, -1), (0, -1), (1, -1)],
        [(-1, 1), (0, 1), (1, 1)],
        [(-1, 1), (-1, 0), (-1, -1)],
        [(1, 1), (1, 0), (1, -1)],
    ]
    elves = []
    locations = {}
    for y, line in enumerate(input.splitlines()):
        for x, c in enumerate(line):
            if c == '#':
                elf = Elf(x, y)
                elves.append(elf)
                locations[elf] = elf
    for _ in range(10):
        proposals = defaultdict(list)
        for elf in elves:
            p = elf.proposal(locations, options)
            if p:
                proposals[p].append(elf)
        for k, v in proposals.items():
            if len(v) == 1:
                elf = v[0]
                del locations[elf]
                elf.move(k)
                locations[elf] = elf
        options.append(options.pop(0))

    min_x = min([elf.x for elf in elves])
    max_x = max([elf.x for elf in elves])
    min_y = min([elf.y for elf in elves])
    max_y = max([elf.y for elf in elves])

    return (max_x - min_x+1) * (max_y - min_y+1) - len(elves)


def part2(input):
    options = [
        [(-1, -1), (0, -1), (1, -1)],
        [(-1, 1), (0, 1), (1, 1)],
        [(-1, 1), (-1, 0), (-1, -1)],
        [(1, 1), (1, 0), (1, -1)],
    ]
    elves = []
    locations = {}
    for y, line in enumerate(input.splitlines()):
        for x, c in enumerate(line):
            if c == '#':
                elf = Elf(x, y)
                elves.append(elf)
                locations[elf] = elf
    rounds = 0
    while True:
        rounds += 1
        proposals = defaultdict(list)
        for elf in elves:
            p = elf.proposal(locations, options)
            if p:
                proposals[p].append(elf)
        if not proposals:
            return rounds
        for k, v in proposals.items():
            if len(v) == 1:
                elf = v[0]
                del locations[elf]
                elf.move(k)
                locations[elf] = elf
        options.append(options.pop(0))


if __name__ == '__main__':
    main()
