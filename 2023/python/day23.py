#!/usr/bin/env python3

from aoc import *
from heapq import heappop, heappush

DIRS = [(0, 0, -1), (1, 1, 0), (2, 0, 1), (3, -1, 0)]
SLIPPERY = ["^.", ">.", "v.", "<."]
NOT_SLIPPERY = ["^>v<.", "^>v<.", "^>v<.", "^>v<."]


def f(x):
    if x[0]:
        return f"[{x[0]:03},{x[1]:02},{x[2]:02}]"
    else:
        return "..........."


def neighbors(xy, forest, passable):
    for dir, dx, dy in DIRS:
        nx = xy[0] + dx
        ny = xy[1] + dy
        if (
            0 <= nx < forest.width
            and 0 <= ny < forest.height
            and forest.data[ny][nx] in passable[dir]
        ):
            yield nx, ny


def walk(xy, forest, passable):
    for path in neighbors(xy, forest, passable):
        d = 0
        walked = set([xy])
        while True:
            d += 1
            paths = list(
                filter(lambda p: p not in walked, neighbors(path, forest, passable))
            )
            if len(paths) == 1:
                walked.add(path)
                path = paths[0]
            else:
                yield path, d
                break


def find_paths(forest, passable):
    paths = {}
    queue = [(1, 0)]
    while queue:
        path = queue.pop()
        if path in paths:
            continue
        paths[path] = {}
        for e, d in walk(path, forest, passable):
            paths[path][e] = max(paths[path].get(e, 0), d)
            queue.append(e)
    return paths


def solve(input: str, passable):
    forest = grid(input)
    end = (forest.width - 2, forest.height - 1)
    paths = find_paths(forest, passable)
    queue = [(0, (1, 0), set())]
    max_len = 0
    while queue:
        d, path, walked = queue.pop()
        if path == end:
            max_len = max(d, max_len)
            continue
        for dest, length in paths[path].items():
            if not dest in walked:
                next = walked.copy()
                next.add(dest)
                queue.append((d + length, dest, next))
    return max_len


# 4126

if __name__ == "__main__":
    main(SLIPPERY, NOT_SLIPPERY)
