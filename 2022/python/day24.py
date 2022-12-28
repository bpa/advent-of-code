#!/usr/bin/env python3

from aoc import *
from heapq import heappop, heappush

BLIZZARD = {
    '^': 0,
    '>': 1,
    'v': 2,
    '<': 3
}

MOVE = [(0, -1), (1, 0), (0, 1), (-1, 0)]


def new_state(height, width):
    state = [[True]*width for _ in range(height)]
    for y in range(height):
        state[y][0] = False
        state[y][width-1] = False
    for x in range(width):
        state[0][x] = False
        state[height-1][x] = False
    state[height-1][width-2] = True
    state[0][1] = True
    return state


def step(blizzards, height, width):
    state = new_state(height, width)
    for b in blizzards:
        m = MOVE[b[2]]
        nx = b[0] + m[0]
        ny = b[1] + m[1]
        if nx == 0:
            nx = width - 2
        if nx == width - 1:
            nx = 1
        if ny == 0:
            ny = height - 2
        if ny == height - 1:
            ny = 1
        b[0] = nx
        b[1] = ny
        state[ny][nx] = False
    return state


def setup(input):
    blizzards = []
    lines = input.splitlines()
    height = len(lines)
    width = len(lines[0])
    initial = new_state(height, width)
    for y, line in enumerate(lines):
        for x, b in enumerate(line):
            d = BLIZZARD.get(b)
            if d is not None:
                blizzards.append([x, y, d])
                initial[y][x] = False

    states = [initial]
    return blizzards, states, width, height


def search(time, x0, y0, x1, y1, blizzards, states):
    height = len(states[0])
    width = len(states[0][0])
    seen = set()

    #         T  DIST   X, Y
    queue = [(time, 10000, x0, y0)]
    while queue:
        l = heappop(queue)
        if l in seen:
            continue
        else:
            seen.add(l)
        if l[2] == x1 and l[3] == y1:
            return l[0]

        t = l[0] + 1
        if len(states) == t:
            states.append(step(blizzards, height, width))

        next_state = states[t]
        for m in MOVE:
            x = l[2] + m[0]
            y = l[3] + m[1]
            if y < len(next_state) and next_state[y][x]:
                heappush(queue, (t, abs(x - x1) + abs(y - y1), x, y))

        if next_state[l[3]][l[2]]:
            heappush(queue, (t, l[1], l[2], l[3]))


def part1(input):
    blizzards, states, width, height = setup(input)
    return search(0, 1, 0, width-2, height-1, blizzards, states)


def part2(input):
    blizzards, states, width, height = setup(input)
    t1 = search(0, 1, 0, width-2, height-1, blizzards, states)
    t2 = search(t1, width-2, height-1, 1, 0, blizzards, states)
    return search(t2, 1, 0, width-2, height-1, blizzards, states)


if __name__ == '__main__':
    main()
