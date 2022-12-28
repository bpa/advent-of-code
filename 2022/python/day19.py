#!/usr/bin/env python3

from aoc import *
from heapq import heappop, heappush

(POTENTIAL, MINUTE, SCORE, ORE, CLAY, OBSIDIAN, GEODE,
 BOT_ORE, BOT_CLAY, BOT_OBS, BOT_GEODE, PARENT) = range(12)
NAMES = {
    "ore": ORE,
    "clay": CLAY,
    "obsidian": OBSIDIAN,
    "geode": GEODE
}


def bot_cost(matches):
    costs = [0, 0, 0, 0, 0, 0, 0]
    for c, t in matches:
        costs[NAMES[t]] = int(c)
    return costs


def costs(line):
    b, costs = line.split(': ')
    blueprint = int(re.findall(r'\d+', b)[0])
    return blueprint, [bot_cost(re.findall(r'(\d+) (\w+)', p)) for p in costs.split('. ')]


def potential(s):
    return s[SCORE] + (s[BOT_GEODE] + s[BOT_GEODE] + s[MINUTE]-1) * s[MINUTE] // 2


def test_potential():
    state = [0, 0, 3,  # potential, time, score
             0, 0, 0, 0,  # inventory
             0, 0, 0, 0,  # bots
             None]  # parent

    state[MINUTE] = 3
    assert potential(state) == 6
    state[MINUTE] = 2
    assert potential(state) == 4
    state[MINUTE] = 1
    assert potential(state) == 3

    state[BOT_ORE+3] = 1
    state[MINUTE] = 3
    assert potential(state) == 9
    state[MINUTE] = 2
    assert potential(state) == 6
    state[MINUTE] = 1
    assert potential(state) == 4


def build(queue, state, bot, cost, max_bots):
    turns = 0
    for i in range(4):
        if state[BOT_ORE+i]:
            if state[BOT_ORE+i] == max_bots[ORE+bot]:
                return
            turns = max(turns, ceil(
                (cost[ORE+i] - state[ORE+i]) / state[BOT_ORE+i]))
        elif cost[ORE+i]:
            return
    turns += 1
    if turns > state[MINUTE]:
        return

    next = state[:]
    if bot == 3:
        next[SCORE] += state[MINUTE] - turns

    next[PARENT] = state
    for i in range(4):
        next[ORE+i] = state[ORE+i] + turns * state[BOT_ORE+i] - cost[ORE+i]
    next[BOT_ORE + bot] += 1

    next[MINUTE] -= turns
    next[POTENTIAL] = -potential(next)
    heappush(queue, next)


def test_build():
    state = [0] * 12
    limits = [10] * 6
    cost = [0] * 7
    state[MINUTE] = 10
    queue = []
    build(queue, state, 0, cost, limits)
    assert len(queue) == 1
    assert queue.pop()[1:] == [9, 0, 0, 0, 0, 0, 1, 0, 0, 0, state]

    state[BOT_ORE] = 1
    cost[ORE] = 1
    build(queue, state, 0, cost, limits)
    assert len(queue) == 1
    assert queue.pop()[1:] == [8, 0, 1, 0, 0, 0, 2, 0, 0, 0, state]

    state[ORE] = 2
    state[CLAY] = 1
    cost[ORE] = 2
    state[BOT_CLAY] = 1
    build(queue, state, 1, cost, limits)
    assert len(queue) == 1
    assert queue.pop()[1:] == [9, 0, 1, 2, 0, 0, 1, 2, 0, 0, state]

    state[ORE] = 1
    state[CLAY] = 6
    cost[ORE] = 3
    cost[CLAY] = 14
    state[BOT_ORE] = 1
    state[BOT_CLAY] = 3
    build(queue, state, 2, cost, limits)
    assert len(queue) == 1
    assert queue.pop()[1:] == [6, 0, 2, 4, 0, 0, 1, 3, 1, 0, state]


def best(costs, minutes=24, dbg=False):
    max_bots = [max(x) for x in zip(*costs)]
    max_bots[GEODE] = None
    initial = [0] * 12
    initial[MINUTE] = minutes
    initial[BOT_ORE] = 1
    initial[POTENTIAL] = -potential(initial)

    queue = [initial]
    best = 0
    while queue:
        s = heappop(queue)
        if best < s[SCORE]:
            best = s[SCORE]
        elif best >= -s[POTENTIAL]:
            return best
        if s[MINUTE] == 0:
            return s[GEODE]

        for i, c in enumerate(costs):
            build(queue, s, i, c, max_bots)
    return best


def part1(input):
    quality = 0
    for line in input.splitlines():
        n, cost = costs(line)
        quality += n * best(cost)
    return quality


def part2(input):
    total = 1
    for line in input.splitlines()[:3]:
        _, cost = costs(line)
        total *= best(cost, 32)
    return total


if __name__ == '__main__':
    main()
