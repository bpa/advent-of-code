#!/usr/bin/env python3

input = "../input/2020/day10.txt"

with open(input) as file:
    adapters = sorted([int(j) for j in file.readlines()])
adapters.append(adapters[-1] + 3)

one = 0
three = 0
last = 0
for i in adapters:
    diff = i - last
    last = i
    if diff == 1:
        one += 1
    if diff == 3:
        three += 1

print("Part 1:", one * three)

permutations = {
    4: 7,
    3: 4,
    2: 2,
    1: 1,
    0: 1
}

answer = 1
width = 0
last = 0
for i in adapters:
    diff = i - last
    last = i
    if diff == 3:
        answer *= permutations[width]
        width = 0
    else:
        width += 1

print("Part 2:", answer)
