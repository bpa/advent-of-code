#!/usr/bin/env python3

from aoc import *


def reindeer(speed, fly, rest):
    cycle = speed * fly
    cycle_len = fly + rest

    def dist(time):
        cycles = time // cycle_len
        remainder = time % cycle_len
        return cycles * cycle + min(remainder, fly) * speed

    return dist


def test_dist():
    dist = reindeer(14, 10, 127)
    assert dist(1) == 14
    assert dist(10) == 140
    assert dist(100) == 140
    assert dist(1000) == 1120


def part1(input):
    flown = 0
    for dist in regex(input, r".*?(\d+).*?(\d+).*?(\d+)", int, reindeer):
        flown = max(flown, dist(2503))
    return flown


def part2(input):
    fleet = list(regex(input, r".*?(\d+).*?(\d+).*?(\d+)", int, reindeer))
    points = [0] * len(fleet)
    for t in range(1, 2504):
        dist = [d(t) for d in fleet]
        best = max(dist)
        for i, d in enumerate(dist):
            if d == best:
                points[i] += 1
    return max(points)


if __name__ == "__main__":
    main()
