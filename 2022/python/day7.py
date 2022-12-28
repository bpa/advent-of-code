#!/usr/bin/env python3

from aoc import *


def find_sizes(input):
    sizes = defaultdict(int)
    cwd = []
    for cmd in input.splitlines():
        tokens = cmd.split()
        if tokens[0] == '$':
            if tokens[1] == 'cd':
                if tokens[2] == '/':
                    cwd = []
                elif tokens[2] == '..':
                    cwd.pop()
                else:
                    cwd.append(tokens[2])
        elif tokens[0] != 'dir':
            file = int(tokens[0])
            path = ''
            sizes[path] += file
            for p in cwd:
                path += '/'
                path += p
                sizes[path] += file
    return sizes


def part1(input):
    sizes = find_sizes(input)
    return sum(filter(lambda v: v <= 100000, sizes.values()))


def part2(input):
    sizes = find_sizes(input)
    available = 70000000 - sizes['']
    required = 30000000
    to_delete = required - available
    candidates = sorted(filter(lambda v: v >= to_delete, sizes.values()))
    return sorted(candidates)[0]


if __name__ == '__main__':
    main()
