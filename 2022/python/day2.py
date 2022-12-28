#!/usr/bin/env python3

from aoc import *


def rps(a):
    play = ord(a[0]) - ord('A')
    if play > 3:
        return play + ord('A') - ord('X')
    else:
        return play


def part1(input):
    score = 0
    for x, y in delimited(input, ' ', rps):
        diff = (x - y + 3) % 3
        if diff == 0:
            score += y + 4
        elif diff == 1:
            score += y + 1
        else:
            score += y + 7
    return score


def part2(input):
    score = 0
    for x, y in delimited(input, ' ', rps):
        score += 1 + y * 3
        if y < 2:
            y = (y + 1) % 2
        play = (x - y + 3) % 3
        score += play
    return score


if __name__ == '__main__':
    main()
