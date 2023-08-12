#!/usr/bin/env python3

from aoc import *


def part1(input):
    answer = 0
    containers = comb(input, r"(\d+)", int, True)
    for r in range(len(containers) + 1):
        for c in itertools.combinations(containers, r):
            if sum(c) == 150:
                answer += 1
    return answer


def part2(input):
    containers = comb(input, r"(\d+)", int, True)
    for r in range(len(containers) + 1):
        answer = 0
        for c in itertools.combinations(containers, r):
            if sum(c) == 150:
                answer += 1
        if answer:
            return answer


if __name__ == "__main__":
    main()
