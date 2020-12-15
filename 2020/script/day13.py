#!/usr/bin/env python3

import math

input = "../input/2020/day13.txt"

with open(input) as file:
    now = int(file.readline())
    buses = [int(b) for b in filter(
        lambda b: not b.startswith('x'), file.readline().split(','))]
first = 1_000_000_000
bus = 0
for b in buses:
    next = ((now // b) + 1) * b
    if next < first:
        first = next
        bus = b

print("Part 1:", (first - now) * bus)


with open(input) as file:
    file.readline()
    buses = list(enumerate(file.readline().split(',')))
    buses = [(-i, int(b))
             for i, b in filter(lambda b: not b[1].startswith('x'), buses)]


def mul_inv(a, b):
    b0 = b
    x0 = 0
    x1 = 1
    while a > 1:
        q = a // b
        t = b
        b = a % b
        a = t
        t = x0
        x0 = x1 - q * x0
        x1 = t
    if x1 < 0:
        x1 += b0
    return x1


def part2(buses):
    prod = math.prod([b for i, b in buses])
    sum = 0

    for i, b in buses:
        p = prod // b
        sum += i * mul_inv(p, b) * p

    return sum % prod


print("Part 2:", part2(buses))
