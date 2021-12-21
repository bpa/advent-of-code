#!/usr/bin/env python3

from aoc import *


def parse(line):
    num = []
    chars = iter(line)
    next(chars)
    stack = [num]
    for c in chars:
        if c == '[':
            nested = []
            stack[-1].append(nested)
            stack.append(nested)
        elif c == ',':
            pass
        elif c == ']':
            stack.pop()
        else:
            stack[-1].append(int(c))
    return num


def explode(snail, depth):
    if depth == 4:
        return (snail[0], snail[1])
    for i in range(2):
        if isinstance(snail[i], list):
            (a, b) = explode(snail[i], depth + 1)
            if a == None:
                continue
            if i == 1:
                if isinstance(snail[0], list):
                    s = snail[0]
                    while isinstance(s[1], list):
                        s = s[1]
                    s[1] += a
                else:
                    snail[0] += a
                a = 0
            if i == 0:
                if isinstance(snail[1], list):
                    s = snail[1]
                    while isinstance(s[0], list):
                        s = s[0]
                    s[0] += b
                else:
                    snail[1] += b
                b = 0
            if depth == 3:
                snail[i] = 0
            return (a, b)
    return (None, None)


def split(snail):
    for (i, s) in enumerate(snail):
        if isinstance(snail[i], list):
            if split(snail[i]):
                return True
        elif snail[i] > 9:
            a = snail[i] // 2
            b = snail[i] - a
            snail[i] = [a, b]
            return True
    return False


def snail_add(a, b):
    snail = [a, b]
    while True:
        (a, b) = explode(snail, 0)
        if a != None:
            continue
        if split(snail):
            pass
        else:
            break
    return snail


def magnitude(snail):
    if isinstance(snail, list):
        return 3 * magnitude(snail[0]) + 2 * magnitude(snail[1])
    else:
        return snail


def part1():
    numbers = puzzle.lines()
    num = parse(next(numbers))
    for addend in numbers:
        n = parse(addend)
        num = snail_add(num, n)
    return magnitude(num)


def part2():
    numbers = [parse(n) for n in puzzle.lines()]
    best = 0
    import copy
    for (a, b) in itertools.permutations(numbers, 2):
        score = magnitude(snail_add(copy.deepcopy(a), copy.deepcopy(b)))
        if score > best:
            best = score
    return best


print("Part 1:", part1())
print("Part 2:", part2())
