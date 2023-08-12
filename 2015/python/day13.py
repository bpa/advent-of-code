#!/usr/bin/env python3

from aoc import *


def parse(input):
    happiness = Distance()
    for t in delimited(input):
        if t[2] == "lose":
            happiness.one_way(t[0], t[10][:-1], -int(t[3]))
        else:
            happiness.one_way(t[0], t[10][:-1], int(t[3]))
    return happiness


def seat(happiness):
    table = happiness.as_table(0)
    max_happy = 0
    for p in itertools.permutations(range(1, len(table))):
        curr = 0
        happy = 0
        for guest in p:
            happy += table[curr][guest]
            happy += table[guest][curr]
            curr = guest
        happy += table[curr][0]
        happy += table[0][curr]
        if happy > max_happy:
            max_happy = happy
    return max_happy


def part1(input):
    happiness = parse(input)
    return seat(happiness)


def part2(input):
    p1 = part1(input)
    happiness = parse(input)
    happiness.nodes.add("ME")
    return seat(happiness)


if __name__ == "__main__":
    main()
