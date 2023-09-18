#!/usr/bin/env python3

from aoc import *


def solve(input, a):
    code = list(comb(input, "[+-]?\w+"))
    i = 0
    r = [a, 0]

    while i >= 0 and i < len(code):
        inst = code[i]
        if inst[0] == "jmp":
            i += int(inst[1])
            continue

        reg = 0 if inst[1] == "a" else 1
        if inst[0] == "jie" and r[reg] % 2 == 0:
            i += int(inst[2])
            continue
        if inst[0] == "jio" and r[reg] == 1:
            i += int(inst[2])
            continue

        i += 1
        if inst[0] == "hlf":
            r[reg] //= 2
        elif inst[0] == "tpl":
            r[reg] *= 3
        elif inst[0] == "inc":
            r[reg] += 1
    return r[1]


if __name__ == "__main__":
    main(0, 1)
