#!/usr/bin/env python3

from aoc import *


def part1(input: str):
    total = 0
    for c in comb(input, r"\d"):
        if c:
            total += int(c[0] + c[-1])
    return total


def toInt(input: str):
    match input:
        case "one":
            return "1"
        case "two":
            return "2"
        case "three":
            return "3"
        case "four":
            return "4"
        case "five":
            return "5"
        case "six":
            return "6"
        case "seven":
            return "7"
        case "eight":
            return "8"
        case "nine":
            return "9"
        case _:
            return input


def part2(input: str):
    total = 0
    exp = r"(\d|one|two|three|four|five|six|seven|eight|nine)"
    first = re.compile(exp)
    last = re.compile(".*" + exp)
    for line in input.splitlines():
        a = toInt(first.search(line).group(1))
        b = toInt(last.match(line).group(1))
        total += int(a + b)
    return total


if __name__ == "__main__":
    main()
