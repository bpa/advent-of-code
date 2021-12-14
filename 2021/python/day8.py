#!/usr/bin/env python3

from aoc import *


def part1():
    with open(sys.argv[1]) as file:
        total = 0
        for line in file.read().splitlines():
            segments = line.split(' | ')[-1].split(' ')
            left = list(filter(lambda x: len(x) in [2, 3, 4, 7], segments))
            total += len(left)
        return total


def five(known, letters):
    if 1 in known and len(letters & known[1]) == 2:
        return 3
    elif 4 in known and len(letters & known[4]) == 2:
        return 2
    elif 2 in known and 3 in known:
        return 5
    else:
        return None


def six(known, letters):
    if 1 in known and len(letters & known[1]) == 1:
        return 6
    elif 4 in known and len(letters & known[4]) == 4:
        return 9
    elif 6 in known and 9 in known:
        return 0
    else:
        return None


lengths = {
    2: lambda k, b: 1,
    3: lambda k, b: 7,
    4: lambda k, b: 4,
    5: five,
    6: six,
    7: lambda k, b: 8,
}


def part2():
    with open(sys.argv[1]) as file:
        total = 0
        for line in file.read().splitlines():
            known = {}
            mapping = {}
            line = line.replace(' | ', ' ')
            segments = [''.join(sorted(s)) for s in line.split(' ')]
            for _ in range(3):
                for s in segments:
                    if s not in mapping:
                        number = lengths[len(s)](known, set(s))
                        if number is not None:
                            known[number] = set(s)
                            mapping[s] = number
            numbers = list(map(lambda l: mapping[l], segments[-4:]))
            total += reduce(lambda a, b: a*10 + b, numbers)
        return total


print("Part 1:", part1())
print("Part 2:", part2())
