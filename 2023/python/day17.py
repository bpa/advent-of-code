#!/usr/bin/env python3

from aoc import *
from aoc.util import manhattan_distance as distance
from heapq import heappop, heappush

adj = [
    [(1, 0, 1), (-1, 0, 3)],
    [(0, -1, 0), (0, 1, 2)],
    [(1, 0, 1), (-1, 0, 3)],
    [(0, -1, 0), (0, 1, 2)],
]


def solve(input: str, blocks):
    city = grid(input, map=int)
    x1 = city.width - 1
    y1 = city.height - 1
    debug(x1, y1)
    visited = set()

    queue = [(0, 0, 0, 0, 1), (0, 0, 0, 0, 2)]
    while queue:
        (_, loss, x, y, dir) = heappop(queue)
        if x == x1 and y == y1:
            return loss

        if (x, y, dir) in visited:
            continue
        else:
            visited.add((x, y, dir))

        for dx, dy, nd in adj[dir]:
            new_loss = 0
            travelled = 0
            for t in range(1, blocks[1]):
                nx = x + (dx * t)
                if nx < 0 or nx >= city.width:
                    continue

                ny = y + (dy * t)
                if ny < 0 or ny >= city.height:
                    continue

                new_loss += city.data[ny][nx]
                travelled += 1
                if travelled >= blocks[0]:
                    h = distance(nx, ny, x1, y1)
                    heappush(queue, (new_loss + loss + h, new_loss + loss, nx, ny, nd))

    return 0


if __name__ == "__main__":
    main((1, 4), (4, 11))
