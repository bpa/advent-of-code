#!/usr/bin/env python3

from aoc import *
from dataclasses import dataclass, field
from typing import List
from functools import cache


@dataclass
class Brick:
    n: int
    x0: int
    y0: int
    z0: int
    x1: int
    y1: int
    z1: int
    supporting: List = field(default_factory=lambda: [])
    resting_on: List = field(default_factory=lambda: [])

    def collides(self, o):
        return (
            self.x0 <= o.x0 <= self.x1
            or self.x0 <= o.x1 <= self.x1
            or o.x0 <= self.x0 <= o.x1
            or o.x0 <= self.x1 <= o.x1
        ) and (
            self.y0 <= o.y0 <= self.y1
            or self.y0 <= o.y1 <= self.y1
            or o.y0 <= self.y0 <= o.y1
            or o.y0 <= self.y1 <= o.y1
        )

    def free(self):
        for o in self.supporting:
            if len(o.resting_on) == 1:
                return False
        return True

    def str(self):
        return f"{self.x0},{self.y0},{self.z0}"

    def __repr__(self):
        return f"{self.n}: {self.x0},{self.y0},{self.z0}"

    def __lt__(self, o):
        return self.z1 < o.z1

    def __hash__(self) -> int:
        return self.n


def stack(input):
    line = 1
    bricks = []
    for l in comb(input, r"\d+", int):
        bricks.append(Brick(line, *l))
        line += 1
    for a, b in itertools.combinations(bricks, 2):
        if a.collides(b):
            if a < b:
                a.supporting.append(b)
                b.resting_on.append(a)
            else:
                b.supporting.append(a)
                a.resting_on.append(b)
    bricks.sort()
    for b in bricks:
        if b.resting_on:
            top = max([x.z1 for x in b.resting_on])
            drop = b.z0 - top - 1
            b.z0 -= drop
            b.z1 -= drop
            remove = list(filter(lambda o: o.z1 != top, b.resting_on))
            for r in remove:
                r.supporting.remove(b)
                b.resting_on.remove(r)
        else:
            drop = b.z0 - 1
            b.z0 -= drop
            b.z1 -= drop
    return bricks


def part1(input: str):
    bricks = stack(input)
    total = 0
    for b in bricks:
        if b.free():
            total += 1
    return total


@cache
def disintigrate(block) -> int:
    disintegrated = set()
    queue = [block]
    while queue:
        x = queue.pop()
        disintegrated.add(x)
        for o in x.supporting:
            if o in disintegrated:
                continue
            for z in o.resting_on:
                if z not in disintegrated:
                    break
            else:
                disintegrated.add(o)
                queue.append(o)
    return len(disintegrated) - 1


def part2(input: str):
    bricks = stack(input)
    total = 0

    for b in bricks:
        total += disintigrate(b)
    return total


if __name__ == "__main__":
    main()
