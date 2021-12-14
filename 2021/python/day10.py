#!/usr/bin/env python3

from typing import Tuple
from aoc import *

pairs = {
    '(': ')',
    '<': '>',
    '[': ']',
    '{': '}',
}

score = {
    ')': 3,
    ']': 57,
    '}': 1197,
    '>': 25137,
}


def part1():
    answer = 0
    for line in puzzle.lines():
        wanted = []
        for c in line:
            if c in pairs:
                wanted.append(pairs[c])
            else:
                if wanted:
                    next = wanted.pop()
                else:
                    next = ''
                if c != next:
                    answer += score[c]
    return answer


auto_score = {
    ')': 1,
    ']': 2,
    '}': 3,
    '>': 4,
}


def part2():
    scores = set()
    for line in puzzle.lines():
        corrupt = False
        wanted = []
        for c in line:
            if c in pairs:
                wanted.append(pairs[c])
            else:
                if wanted:
                    next = wanted.pop()
                    if c != next:
                        corrupt = True
                        continue
                else:
                    corrupt = True
                    continue
        if not corrupt:
            score = 0
            while wanted:
                w = wanted.pop()
                score *= 5
                score += auto_score[w]
            scores.add(score)
    return sorted(scores)[int(len(scores)/2)]


print("Part 1:", part1())
print("Part 2:", part2())
