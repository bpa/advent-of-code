#!/usr/bin/env python3

from aoc import *

# Item:    Cost Damage Armor
equipment = """
Dagger        8      4     0
Shortsword   10      5     0
Warhammer    25      6     0
Longsword    40      7     0
Greataxe     74      8     0

Shirt         0      0     0
Leather      13      0     1
Chainmail    31      0     2
Splintmail   53      0     3
Bandedmail   75      0     4
Platemail   102      0     5

Damage +0     0      0     0
Damage +1    25      1     0
Damage +2    50      2     0
Damage +3   100      3     0
Defense +0    0      0     0
Defense +1   20      0     1
Defense +2   40      0     2
Defense +3   80      0     3
"""


def inventory():
    return [list(comb(c, r"\s(\d+)", int)) for c in equipment.strip().split("\n\n")]


def init(input):
    (hp, boss_atk, boss_def) = comb(input, r"\d+", int, one_line=True)

    def fight(plr_atk, plr_def):
        boss_hp = hp
        plr_hp = 100

        while True:
            boss_hp -= max(1, plr_atk - boss_def)
            if boss_hp <= 0:
                return True

            plr_hp -= max(1, boss_atk - plr_def)
            if plr_hp <= 0:
                return False

    return fight


def part1(input):
    you_win = init(input)
    best = 1000
    (weapons, armor, rings) = inventory()
    for w in weapons:
        for a in armor:
            for l, r in itertools.permutations(rings, 2):
                (gold, attack, defense) = [sum(x) for x in zip(w, a, l, r)]
                if you_win(attack, defense):
                    best = min(best, gold)
    return best


def part2(input):
    you_win = init(input)
    most = 0
    (weapons, armor, rings) = inventory()
    for w in weapons:
        for a in armor:
            for l, r in itertools.permutations(rings, 2):
                (gold, attack, defense) = [sum(x) for x in zip(w, a, l, r)]
                if not you_win(attack, defense):
                    most = max(most, gold)
    return most


if __name__ == "__main__":
    main()
