#!/usr/bin/env python3
import sys


def spawn(days):
    with open(sys.argv[1]) as file:
        fish = [0] * 10
        for line in file.readlines():
            for f in line.split(','):
                fish[int(f)] = fish[int(f)] + 1
        for _ in range(days):
            spawning = fish[0]
            for s in range(9):
                fish[s] = fish[s+1]
            fish[8] = spawning
            fish[6] += spawning
        return sum(fish)


print("Part 1:", spawn(80))
print("Part 2:", spawn(256))
