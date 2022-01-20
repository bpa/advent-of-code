#!/usr/bin/env python3

from collections import defaultdict

input = "../input/2020/day23.txt"

cups = [0] * 10


def remove(current):
    global cups
    three = []
    right = cups[current]
    for _ in range(3):
        three.append(right)
        right = cups[right]
    cups[current] = right
    return three


def cups_as_str():
    global cups
    s = []
    c = cups[1]
    while c != 1:
        s.append(str(c))
        c = cups[c]
    return ''.join(s)


def play(current, rounds, top):
    global cups
    for _ in range(rounds):
        three = remove(current)
        ins = current - 1
        while ins in three or ins < 1:
            if ins < 1:
                ins = top
            else:
                ins -= 1
        cups[three[-1]] = cups[ins]
        cups[ins] = three[0]
        current = cups[current]


def init(circle):
    global cups
    current = int(circle[0])
    for v in circle:
        c = int(v)
        right = cups[current]
        cups[c] = right
        cups[current] = c
        current = c


with open(input) as file:
    circle = file.readline().strip()

init(circle)
play(int(circle[0]), 100, 9)
print("Part 1:", cups_as_str())

init(circle)
last = int(circle[-1])
right = cups[last]
cups[last] = 10
cups.extend(range(11, 1_000_001))
cups.append(right)
play(int(circle[0]), 10_000_000, 1_000_000)
one = cups[1]
two = cups[one]
print("Part 2:", one * two)
