#!/usr/bin/env python3

from aoc import *


def part1(input: str):
    crossed = 0
    pos = 50
    for turn in input.split():
        amount = int(turn[1:])
        if turn[0] == 'L':
            pos -= amount
        else:
            pos += amount
        pos = pos % 100
        if pos == 0:
            crossed += 1
    return crossed


def part2(input: str):
    crossed = 0
    pos = 50
    for turn in input.split():
        amount = int(turn[1:])
        if turn[0] == 'L':
            extra = 1 if pos > 0 else 0
            pos -= amount
            if pos <= 0:
                crossed += pos // -100 + extra
        else:
            pos += amount
            crossed += pos // 100
        pos = pos % 100
    return crossed


if __name__ == '__main__':
    main()
