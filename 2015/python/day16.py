#!/usr/bin/env python3

from aoc import *

analysis = {
    "children": 3,
    "cats": 7,
    "samoyeds": 2,
    "pomeranians": 3,
    "akitas": 0,
    "vizslas": 0,
    "goldfish": 5,
    "trees": 3,
    "cars": 2,
    "perfumes": 1,
}


def check(sue):
    for prop, value in sue[1:]:
        if analysis[prop] != int(value):
            return False
    return True


def part1(input):
    for sue in comb(input, r"(\w+):? (\d+)"):
        if check(sue):
            return sue[0][1]


def check2(sue):
    for prop, value in sue[1:]:
        if prop in ["cats", "trees"]:
            if int(value) <= analysis[prop]:
                return False
        elif prop in ["pomeranians", "goldfish"]:
            if int(value) >= analysis[prop]:
                return False
        elif analysis[prop] != int(value):
            return False
    return True


def part2(input):
    for sue in comb(input, r"(\w+):? (\d+)"):
        if check2(sue):
            return sue[0][1]


if __name__ == "__main__":
    main()
