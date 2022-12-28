#!/usr/bin/env python3

from aoc import *

NUM = ['2', '1', '0', '-', '=']
VAL = [2, 1, 0, -1, -2]


def to_snafu(value):
    s = ''
    while value:
        r = value % 5
        if r >= 3:
            value += 5
            if r == 4:
                s = '-' + s
            else:
                s = '=' + s
        else:
            s = str(r) + s
        value //= 5
    return s


def snafu(line):
    n = 0
    for c in line:
        n *= 5
        n += VAL[NUM.index(c)]
    return n


def part1(input):
    val = sum([snafu(l) for l in input.splitlines()])
    return to_snafu(val)


if __name__ == '__main__':
    main()
