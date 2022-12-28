#!/usr/bin/env python3

from aoc import *


def part1(input):
    interesting = [20, 60, 100, 140, 180, 220]
    cycle = 1
    value = 1
    strength = 0
    for line in input.splitlines():
        toks = line.split()
        if toks[0] == 'noop':
            cycle += 1
        else:
            if cycle + 1 in interesting:
                strength += (cycle+1) * value
            cycle += 2
            value += int(toks[1])
        if cycle in interesting:
            strength += cycle * value
    return strength


def draw(cycle, value):
    cycle = cycle % 40
    pixel = '' if cycle else '\n'
    if (cycle - 1) <= value <= (cycle+1):
        return pixel + '#'
    else:
        return pixel + ' '


def part2(input):
    cycle = 0
    value = 1
    output = ''
    for line in input.splitlines():
        output += draw(cycle, value)
        cycle += 1
        toks = line.split()
        if toks[0] != 'noop':
            output += draw(cycle, value)
            cycle += 1
            value += int(toks[1])
    return output


if __name__ == '__main__':
    main()
