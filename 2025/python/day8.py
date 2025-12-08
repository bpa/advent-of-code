#!/usr/bin/env python3

from heapq import heappop, heappush
from math import sqrt
from aoc import *

def merge(a, b, circuits, boxes) -> tuple[int, int] | None:
    if c := circuits.get(a):
        if d := circuits.get(b):
                if c is not d:
                    for n in d:
                        c.add(n)
                        circuits[n] = c
                    boxes.remove(d)
        else:
            c.add(b)
            circuits[b] = c
        return c

def make_new_circuit(a, b, circuits, boxes):
    c = set()
    c.add(a)
    c.add(b)
    circuits[a] = c
    circuits[b] = c
    boxes.append(c)
    return c

def part1(input: str):
    junctions = []
    distances = []
    for node in input.split():
        x, y, z = [int(n) for n in node.split(',')]
        for j in junctions:
            dx = j[0] - x
            dy = j[1] - y
            dz = j[2] - z
            dist = sqrt(dx * dx + dy * dy + dz * dz)
            heappush(distances, (dist, j, (x, y, z)))
        junctions.append((x, y, z))
    circuits = {}
    boxes = []
    for _ in range(1000):
        dist, a, b = heappop(distances)
        merge(a, b, circuits, boxes) or merge(b, a, circuits, boxes) or make_new_circuit(a, b, circuits, boxes)
    boxes = sorted([len(c) for c in boxes], reverse=True)
    return boxes[0] * boxes[1] * boxes[2]


def part2(input: str):
    junctions = []
    distances = []
    for node in input.split():
        x, y, z = [int(n) for n in node.split(',')]
        for j in junctions:
            dx = j[0] - x
            dy = j[1] - y
            dz = j[2] - z
            dist = sqrt(dx * dx + dy * dy + dz * dz)
            heappush(distances, (dist, j, (x, y, z)))
        junctions.append((x, y, z))
    circuits = {}
    boxes = []
    while True:
        dist, a, b = heappop(distances)
        c = merge(a, b, circuits, boxes) or merge(b, a, circuits, boxes) or make_new_circuit(a, b, circuits, boxes)
        debug(c)
        if len(c) == len(junctions):
            return a[0] * b[0]
    

if __name__ == '__main__':
    main()
