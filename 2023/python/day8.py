#!/usr/bin/env python3

from aoc import *


def parse(input):
    lines = comb(input, r"\w+")
    (path,) = next(lines)
    path = [0 if c == "L" else 1 for c in path]
    next(lines)
    nodes = {}
    for x in lines:
        nodes[x[0]] = x[1:]
    return path, nodes


def part1(input: str):
    path, nodes = parse(input)

    curr = "AAA"
    steps = 0
    for n in itertools.cycle(path):
        curr = nodes[curr][n]
        steps += 1
        if curr == "ZZZ":
            break
    return steps


def get_steps(start, path, nodes):
    steps = 0
    for n in itertools.cycle(path):
        start = nodes[start][n]
        steps += 1
        if start.endswith("Z"):
            return steps


def part2(input: str):
    path, nodes = parse(input)

    start = filter(lambda n: n.endswith("A"), nodes.keys())
    cycle_lengths = [get_steps(n, path, nodes) for n in start]
    return math.lcm(*cycle_lengths)


if __name__ == "__main__":
    main()
