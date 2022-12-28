#!/usr/bin/env python3

from typing import List, Tuple
from aoc import *


def parse(input) -> Tuple[List[List[str]], List[List[int]]]:
    stacks = [[] for i in range(9)]
    moves = []
    lines = iter(input.splitlines())

    while True:
        line = next(lines)
        if line[1] == '1':
            next(lines)
            break
        for i, crate in enumerate(chunks(line, 4)):
            if crate[1].isalpha():
                stacks[i].insert(0, crate[1])

    for move in lines:
        tokens = move.split()
        moves.append((int(tokens[1]), int(tokens[3]), int(tokens[5])))

    return stacks, moves


def part1(input):
    (stacks, moves) = parse(input)
    for move in moves:
        for i in range(move[0]):
            stacks[move[2]-1].append(stacks[move[1]-1].pop())
    return ''.join([s[-1] if s else '' for s in stacks])


def part2(input):
    (stacks, moves) = parse(input)
    for move in moves:
        stacks[move[2]-1].extend(stacks[move[1]-1][-move[0]:])
        del stacks[move[1]-1][-move[0]:]
    return ''.join([s[-1] if s else '' for s in stacks])


if __name__ == '__main__':
    main()
