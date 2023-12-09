#!/usr/bin/env python3

from aoc import *


def matches(ticket):
    number = iter(ticket.split())
    next(number)
    next(number)
    winners = set()
    while True:
        x = next(number)
        if x == "|":
            break
        winners.add(x)
    have = set(number)
    return len(winners & have)


def part1(input: str):
    points = 0
    for ticket in input.splitlines():
        if winners := matches(ticket):
            points += 1 << (winners - 1)
    return points


def part2(input: str):
    tickets = input.splitlines()
    copies = [1] * len(tickets)
    for i, ticket in enumerate(tickets):
        instances = copies[i]
        if winners := matches(ticket):
            for j in range(i + 1, i + winners + 1):
                copies[j] += instances

    return sum(copies)


if __name__ == "__main__":
    main()
