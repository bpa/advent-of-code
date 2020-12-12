#!/usr/bin/env python3

input = "../input/2020/day12.txt"
dir = 1
x = 0
y = 0
compass = [[0, 1], [1, 0], [0, -1], [-1, 0]]

with open(input) as file:
    for line in file.readlines():
        line = line.strip()
        inst = line[0]
        val = int(line[1:])
        if inst == 'N':
            move = 0
        elif inst == 'E':
            move = 1
        elif inst == 'S':
            move = 2
        elif inst == 'W':
            move = 3
        elif inst == 'F':
            move = dir
        elif inst == 'R':
            dir = (dir + (val // 90)) % 4
            continue
        elif inst == 'L':
            dir = (dir + 4 - (val // 90)) % 4
            continue
        else:
            continue
        x += compass[move][0] * val
        y += compass[move][1] * val
print("Part 1:", abs(x) + abs(y))

x = 0
y = 0
wx = 10
wy = 1
with open(input) as file:
    for line in file.readlines():
        line = line.strip()
        inst = line[0]
        val = int(line[1:])
        if inst == 'N':
            wy += val
        elif inst == 'E':
            wx += val
        elif inst == 'S':
            wy -= val
        elif inst == 'W':
            wx -= val
        elif inst == 'F':
            x += val * wx
            y += val * wy
        else:
            dir = val // 90
            if inst == 'L':
                dir = 4 - dir
            if dir == 1:
                tmp = wx
                wx = wy
                wy = -tmp
            elif dir == 2:
                wx = -wx
                wy = -wy
            elif dir == 3:
                tmp = wx
                wx = -wy
                wy = tmp

print("Part 2:", abs(x) + abs(y))
