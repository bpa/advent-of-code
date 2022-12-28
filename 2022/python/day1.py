#!/usr/bin/env python3


from aoc import *


def elf(input):
    return sum([int(i) for i in input.splitlines()])


def part1(input):
    return max(delimit(input, "\n\n", elf))


def part2(input):
    elves = delimit(input, "\n\n", elf)
    return sum(sorted(elves)[-3:])


if __name__ == '__main__':
    main()
