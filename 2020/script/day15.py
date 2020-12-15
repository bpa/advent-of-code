#!/usr/bin/env python3

input = "../input/2020/day15.txt"


def memory(n, numbers):
    turn = 1
    seen = {}
    for i in numbers:
        seen[i] = turn
        turn += 1

    next = 0
    while turn < n:
        if next in seen:
            tmp = turn - seen[next]
            seen[next] = turn
            next = tmp
        else:
            seen[next] = turn
            next = 0
        turn += 1

    return next


with open(input) as file:
    items = [int(n) for n in file.readline().strip().split(',')]

print("Example 1:", memory(2020, [0, 3, 6]))
print("Part 1:", memory(2020, items))
print("Part 2:", memory(30_000_000, items))
