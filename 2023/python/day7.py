#!/usr/bin/env python3

from aoc import *


def strength(c):
    match c:
        case "A":
            return 14
        case "K":
            return 13
        case "Q":
            return 12
        case "J":
            return 11
        case "T":
            return 10
        case _:
            return int(c)


def power(hand):
    matches = defaultdict(lambda: 0)
    for c in hand[0]:
        matches[c] += 1
    sets = sorted(matches.values(), reverse=True)
    match sets[0]:
        case 5:
            p = 7
        case 4:
            p = 6
        case 3:
            p = 5 if sets[1] == 2 else 4
        case 2:
            p = 3 if sets[1] == 2 else 2
        case _:
            p = 1

    return (p, *[strength(c) for c in hand[0]])


def part1(input: str):
    hands = list(delimited(input))
    held = zip(range(1, len(hands) + 1), sorted(hands, key=power))
    return sum([h[0] * int(h[1][1]) for h in held])


def strength_joker_low(c):
    match c:
        case "A":
            return 14
        case "K":
            return 13
        case "Q":
            return 12
        case "J":
            return 1
        case "T":
            return 10
        case _:
            return int(c)


def power_joker(hand):
    matches = defaultdict(lambda: 0)
    for c in hand[0]:
        matches[c] += 1
    j = matches.pop("J", 0)
    sets = sorted(matches.values(), reverse=True)
    try:
        sets[0] += j
    except:
        sets.append(5)
    match sets[0]:
        case 5:
            p = 7
        case 4:
            p = 6
        case 3:
            p = 5 if sets[1] == 2 else 4
        case 2:
            p = 3 if sets[1] == 2 else 2
        case _:
            p = 1

    return (p, *[strength_joker_low(c) for c in hand[0]])


def part2(input: str):
    hands = list(delimited(input))
    held = zip(range(1, len(hands) + 1), sorted(hands, key=power_joker))
    return sum([h[0] * int(h[1][1]) for h in held])


if __name__ == "__main__":
    main()
