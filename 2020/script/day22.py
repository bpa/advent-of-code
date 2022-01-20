#!/usr/bin/env python3

input = "../input/2020/day22.txt"


def score(deck):
    return sum([(i+1) * v for i, v in enumerate(reversed(deck))])


def recursive_combat(p1, p2):
    configurations = set()
    while len(p1) > 0 and len(p2) > 0:
        hash = ''.join([str(i) for i in p1])
        if hash in configurations:
            return p1, p2
        else:
            configurations.add(hash)
        a = p1.pop(0)
        b = p2.pop(0)
        if len(p1) >= a and len(p2) >= b:
            p1_sub, _ = recursive_combat(p1[:a].copy(), p2[:b].copy())
            p1_won = len(p1_sub) > 0
        else:
            p1_won = a > b
        if p1_won:
            p1.append(a)
            p1.append(b)
        else:
            p2.append(b)
            p2.append(a)
    return p1, p2


with open(input) as file:
    line = file.readline()
    line = file.readline()
    p1_starting_deck = []
    while not line.isspace():
        p1_starting_deck.append(int(line))
        line = file.readline()

    line = file.readline()
    p2_starting_deck = [int(l) for l in file.readlines()]

p1_deck = p1_starting_deck.copy()
p2_deck = p2_starting_deck.copy()

while len(p1_deck) > 0 and len(p2_deck) > 0:
    a = p1_deck.pop(0)
    b = p2_deck.pop(0)
    if a < b:
        p2_deck.append(b)
        p2_deck.append(a)
    else:
        p1_deck.append(a)
        p1_deck.append(b)

print("Part 1:", score(p1_deck if len(p2_deck) == 0 else p2_deck))
p1_deck, p2_deck = recursive_combat(p1_starting_deck, p2_starting_deck)
print("Part 2:", score(p1_deck if len(p2_deck) == 0 else p2_deck))
