#!/usr/bin/env python3

from aoc import *


def die(sides):
    while True:
        for i in range(1, sides+1):
            yield i


def part1():
    lines = puzzle.lines()
    one = int(next(lines)[-2:]) - 1
    two = int(next(lines)[-2:]) - 1
    players = [one, two]
    points = [0, 0]
    d = die(100)
    player = 0
    rolls = 0
    while True:
        if points[0] >= 1000 or points[1] >= 1000:
            break

        move = next(d) + next(d) + next(d)
        rolls += 3
        tile = (players[player] + move) % 10
        points[player] += tile + 1
        players[player] = tile
        player = (player + 1) % 2
    return min(points) * rolls


rolls = [
    (3, 1),
    (4, 3),
    (5, 6),
    (6, 7),
    (7, 6),
    (8, 3),
    (9, 1),
]


def quantify(turn, player, scores):
    wins = [0, 0]
    for (roll, freq) in rolls:
        tile = (player[turn] + roll) % 10
        score = scores[turn] + tile + 1
        if score >= 21:
            wins[turn] += freq
        else:
            next = (turn + 1) % 2
            if turn:
                (a, b) = quantify(next, (player[0], tile), (scores[0], score))
            else:
                (a, b) = quantify(next, (tile, player[1]), (score, scores[1]))
            wins[0] += a * freq
            wins[1] += b * freq
    return wins


def part2():
    lines = puzzle.lines()
    one = int(next(lines)[-2:]) - 1
    two = int(next(lines)[-2:]) - 1
    return max(quantify(0, (one, two), (0, 0)))


if __name__ == '__main__':
    print("Part 1:", part1())
    print("Part 2:", part2())
