#!/usr/bin/env python3

from aoc import *


def part1(input):
    count = 0
    nice = r(r'(?=.*[aeiou].*[aeiou].*[aeiou])(?=.*(.)\1)(?!.*(ab|cd|pq|xy))')
    for line in input.splitlines():
        if nice.match(line):
            count += 1
    return count


def part2(input):
    count = 0
    nice = r(r'(?=.*(..).*\1)(?=.*(.).\2)')
    for line in input.splitlines():
        if nice.match(line):
            count += 1
    return count


if __name__ == '__main__':
    main()
