#!/usr/bin/env python3

from aoc import *


def reflect(lines):
    i = 0
    for a, b in itertools.pairwise(lines):
        i += 1
        if a == b:
            for x in range(1, i):
                try:
                    if lines[i - x - 1] != lines[i + x]:
                        break
                except:
                    pass
            else:
                return i

    return 0


def mirror(note):
    lines = note.splitlines()
    if h := reflect(lines):
        return h * 100
    else:
        return reflect(list(zip(*lines)))


def part1(input: str):
    total = 0
    for note in input.split("\n\n"):
        total += mirror(note)
    return total


def eq(a, b, smudged):
    if a == b:
        return smudged, True
    if smudged:
        diff = False
        for i, j in zip(a, b):
            if i != j:
                if diff:
                    return True, False
                else:
                    diff = True
        return False, True
    return smudged, False


def partly_reflect(lines):
    i = 0
    for a, b in itertools.pairwise(lines):
        i += 1
        smudged, same = eq(a, b, True)
        if same:
            for x in range(1, i):
                try:
                    smudged, same = eq(lines[i - x - 1], lines[i + x], smudged)
                    if not same:
                        break
                except:
                    pass
            else:
                if not smudged:
                    return i

    return 0


def smudged(note):
    lines = note.splitlines()
    if h := partly_reflect(lines):
        debug("h", h * 100)
        return h * 100
    else:
        return partly_reflect(list(zip(*lines)))


def part2(input: str):
    total = 0
    for note in input.split("\n\n"):
        total += smudged(note)
    return total


# 38110

if __name__ == "__main__":
    main()
