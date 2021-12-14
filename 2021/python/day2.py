#!/usr/bin/env python3

from aoc import *


def part1():
    depth = 0
    position = 0
    def up(u): nonlocal depth; depth -= int(u)
    def down(u): nonlocal depth; depth += int(u)
    def forward(u): nonlocal position; position += int(u)
    switch(forward=forward, up=up, down=down)(puzzle.delimited())
    return depth * position


def part2():
    aim = 0
    depth = 0
    position = 0
    def up(u): nonlocal aim; aim -= int(u)
    def down(u): nonlocal aim; aim += int(u)

    def forward(u):
        nonlocal position, depth
        position += int(u)
        depth += int(u) * aim

    switch({
        'forward': forward,
        'up': up,
        'down': down,
    })(puzzle.delimited())
    return depth * position


print("Part 1:", part1())
print("Part 2:", part2())
