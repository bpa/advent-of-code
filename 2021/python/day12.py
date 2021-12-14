#!/usr/bin/env python3

from aoc import *


def get_node(caves, a):
    if a in caves:
        n = caves[a]
    else:
        n = {'big': bool(re.match('^[A-Z]+$', a)),
             'exits': [], 'visited': 0}
        caves[a] = n
    return n


def delve(nodes, name, path, all, twice):
    if name == 'end':
        all.add(','.join(path))
        return

    cave = nodes[name]
    if name == twice:
        if cave['visited'] > 1:
            return
    elif cave['visited']:
        return

    if not cave['big']:
        cave['visited'] += 1

    path.append(name)
    for e in cave['exits']:
        delve(nodes, e, path, all, twice)

    if not cave['big']:
        cave['visited'] -= 1
    path.pop()


def part1():
    caves = {}
    for (a, b) in puzzle.delimited('-'):
        start = get_node(caves, a)
        end = get_node(caves, b)
        start['exits'].append(b)
        end['exits'].append(a)
    for n in caves.values():
        n['exits'].sort()

    all = set()
    delve(caves, 'start', [], all, None)
    return len(all)


def part2():
    caves = {}
    for (a, b) in puzzle.delimited('-'):
        start = get_node(caves, a)
        end = get_node(caves, b)
        start['exits'].append(b)
        end['exits'].append(a)
    for n in caves.values():
        n['exits'].sort()

    paths = set()
    for twice in caves.keys():
        if twice != 'start':
            delve(caves, 'start', [], paths, twice)
    return len(paths)


print("Part 1:", part1())
print("Part 2:", part2())
