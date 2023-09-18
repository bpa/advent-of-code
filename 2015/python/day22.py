#!/usr/bin/env python3

from aoc import *
from heapq import heappop, heappush

(SPENT, HP, MANA, ARMOR, BOSS_HP, SHIELD, POISON, RECHARGE) = range(8)


def poison(state):
    if state[POISON]:
        state[POISON] -= 1
        state[BOSS_HP] -= 3


def shield(state):
    if state[SHIELD]:
        state[SHIELD] -= 1
    if state[SHIELD]:
        state[ARMOR] = 7
    else:
        state[ARMOR] = 0


def recharge(state):
    if state[RECHARGE]:
        state[MANA] += 101
        state[RECHARGE] -= 1


def cast(states, state, mana, dmg=0, heal=0, effect=0):
    if state[MANA] >= mana:
        next = state[:]
        next[MANA] -= mana
        next[SPENT] += mana
        next[BOSS_HP] -= dmg
        next[HP] += heal
        if effect and next[effect]:
            return
        if effect == SHIELD:
            next[ARMOR] = 7
            next[SHIELD] = 6
        elif effect == POISON:
            next[POISON] = 6
        elif effect == RECHARGE:
            next[RECHARGE] = 5
        heappush(states, next)


def solve(input, hard):
    (boss_hp, damage) = comb(input, r"\d+", int, one_line=True)
    # Give additional health because this has the boss go first
    states = [[0, 50 + damage, 500, 0, boss_hp, 0, 0, 0]]

    while True:
        curr = heappop(states)

        poison(curr)
        shield(curr)
        recharge(curr)
        if curr[BOSS_HP] <= 0:
            return curr[SPENT]

        curr[HP] -= max(1, damage - curr[ARMOR])
        if hard:
            curr[HP] -= 1
        if curr[HP] <= 0:
            continue

        poison(curr)
        shield(curr)
        recharge(curr)
        if curr[BOSS_HP] <= 0:
            return curr[SPENT]

        cast(states, curr, 53, dmg=4)
        cast(states, curr, 73, dmg=2, heal=2)
        cast(states, curr, 113, effect=SHIELD)
        cast(states, curr, 173, effect=POISON)
        cast(states, curr, 229, effect=RECHARGE)


if __name__ == "__main__":
    main(False, True)
