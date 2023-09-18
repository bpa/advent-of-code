#!/usr/bin/env python3

from aoc import *


def part1(input):
    (row, column) = comb(input, r"\d+", int, one_line=True)
    prev_diagonal = row + column - 2
    pos = prev_diagonal * (prev_diagonal + 1) // 2 + column - 1
    key = 20151125
    for _ in range(pos):
        key = key * 252533 % 33554393
    return key


if __name__ == "__main__":
    main()
