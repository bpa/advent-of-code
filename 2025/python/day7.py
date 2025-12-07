#!/usr/bin/env python3

from aoc import *


def part1(input: str):
    total = 0
    splitter = grid(input)
    s = set([splitter.index('S')['S']])
    while s:
        next = set()
        for line in s:
            n = line.s()
            if n:
                if n.get() == '^':
                    total += 1
                    if n.e():
                        next.add(n.e())
                    if n.w():
                        next.add(n.w())
                else:
                    next.add(n)
        s = next
    return total


def part2(input: str):
    splitter = grid(input)
    s = defaultdict(int)
    s[splitter.index('S')['S']] = 1
    for _ in range(splitter.height-1):
        next = defaultdict(int)
        for p, cnt in s.items():
            n = p.s()
            if n:
                if n.get() == '^':
                    e = n.e()
                    w = n.w()
                    if e:
                        next[e] += cnt
                    if w:
                        next[w] += cnt
                else:
                    next[n] += cnt
        s = next
    return sum(s.values())

if __name__ == '__main__':
    main()
