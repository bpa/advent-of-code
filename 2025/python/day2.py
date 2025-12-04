#!/usr/bin/env python3

from aoc import *


def part1(input: str):
    tot = 0
    for l in input.split(","):
        a, b  = l.split("-")
        for i in range(int(a), int(b)+1):
            s = str(i)
            n = len(s)
            if n % 2 == 1:
                continue
            if s[:n//2] == s[n//2:]:
                tot += i
    return tot


def part2(input: str):
    tot = 0
    for l in input.rstrip().split(","):
        print(f"{l}:")
        a, b  = l.split("-")
        for i in range(int(a), int(b)+1):
            s = str(i)
            n = len(s)
            for k in range(1, n//2 + 1):
                if n % k == 0 and s[:k] * (n//k) == s:
                    print(f"  {i}")
                    tot += i
                    break
    return tot


if __name__ == '__main__':
    main()
