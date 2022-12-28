#!/usr/bin/env python3

from aoc import *
from termcolor import colored


def hike(a, b):
    if ord(a)-ord(b) >= -1:
        return 1
    return 10000


def part1(input):
    forest = grid(input)
    places = forest.index('S', 'E')
    s = places['S']
    e = places['E']
    s.set('a')
    e.set('z')
    path = forest.shortest_path(s, e, vertex_cost=hike)
    bk = {
        'm': 'on_yellow',
        'n': 'on_cyan',
        'o': 'on_red',
        'p': 'on_magenta'
    }
    for p in reversed(path):
        c = p.get()
        p.set(colored(p.get(), 'green', bk.get(c)))
    for e in forest:
        c = e.get()
        if c in bk:
            e.set(colored(c, None, bk[c]))
    return len(path)


def hike_back(a, b):
    if ord(b)-ord(a) >= -1:
        return 1
    return 10000


def part2(input):
    forest = grid(input)

    def X(x, y):
        return forest.get(x, y) == 'a'

    places = forest.index('E', 'S')
    e = places['E']
    e.set('z')
    path = forest.shortest_path(
        e, places['S'], vertex_cost=hike_back, distance=lambda ax, ay, bx, by: 1, found=X)
    return len(path)


if __name__ == '__main__':
    main()
