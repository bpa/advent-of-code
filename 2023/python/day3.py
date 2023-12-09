#!/usr/bin/env python3

from aoc import *


def find(p: Point, seen: dict):
    while p.w() and p.w().get().isdigit():
        p = p.w()
    n = 0
    if (p.x, p.y) in seen:
        return 0
    seen.add((p.x, p.y))
    while p and p.get().isdigit():
        n *= 10
        n += int(p.get())
        p = p.e()
    return n


def part1(input: str):
    engine = grid(input)
    seen = set()
    total = 0
    for e in engine:
        if not e.get().isdigit() and e != ".":
            for p in e.adjacent_8():
                if p.get().isdigit():
                    total += find(p, seen)
    return total


def part2(input: str):
    engine = grid(input)
    total = 0
    for e in engine:
        if e == "*":
            seen = set()
            nums = []
            for p in e.adjacent_8():
                if p.get().isdigit():
                    n = find(p, seen)
                    if n > 0:
                        nums.append(n)
            if len(nums) == 2:
                total += nums[0] * nums[1]
    return total


if __name__ == "__main__":
    main()
