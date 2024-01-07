#!/usr/bin/env python3

from aoc import *


def hash(input: str):
    hash = 0
    for i in input:
        hash += ord(i)
        hash *= 17
        hash %= 256
    return hash


def part1(input: str):
    total = 0
    for l in input.splitlines():
        for tok in l.split(","):
            x = hash(tok)
            total += x
    return total


def part2(input: str):
    box_labels = [[] for _ in range(256)]
    box_lenses = [[] for _ in range(256)]
    for l in input.splitlines():
        for instr in l.split(","):
            if instr[-1] == "-":
                label = instr[:-1]
                box = hash(label)
                try:
                    ind = box_labels[box].index(label)
                    box_labels[box].pop(ind)
                    box_lenses[box].pop(ind)
                except:
                    pass
            else:
                label = instr[:-2]
                box = hash(label)
                lens = int(instr[-1])
                try:
                    ind = box_labels[box].index(label)
                    box_lenses[box][ind] = lens
                except:
                    box_labels[box].append(label)
                    box_lenses[box].append(lens)
    power = 0
    for box, lenses in enumerate(box_lenses):
        for slot, focal_length in enumerate(lenses):
            power += (box + 1) * (slot + 1) * focal_length
    return power


if __name__ == "__main__":
    main()
