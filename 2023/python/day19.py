#!/usr/bin/env python3

from aoc import *
from copy import deepcopy


def check(part, rules):
    r = rules["in"]
    while True:
        for sr in r:
            if ":" in sr:
                (body, target) = sr.split(":")
                x = part[body[0]]
                match body[1]:
                    case "<":
                        if x < int(body[2:]):
                            next_rule = target
                            break
                    case ">":
                        if x > int(body[2:]):
                            next_rule = target
                            break
                    case "_":
                        raise f"Unknown op: {body[1]}"
            else:
                next_rule = sr
                break
        if next_rule == "A":
            return True
        elif next_rule == "R":
            return False
        r = rules[next_rule]


def part1(input: str):
    rules = {}
    sections = input.split("\n\n")
    for r in sections[0].splitlines():
        toks = re.split(r"{|}|,", r)
        rules[toks[0]] = toks[1:-1]

    total = 0
    for p in sections[1].splitlines():
        t = re.split(r"[{},=]", p)
        i = iter(t[1:-1])
        part = {}
        try:
            while True:
                k = next(i)
                v = int(next(i))
                part[k] = v
        except StopIteration:
            pass

        if check(part, rules):
            total += sum(part.values())
    return total


def split_constraint(constraints, c, n, op):
    orig = constraints[c]
    a = []
    b = []
    if op == "<":
        for r in orig:
            if r[1] < n:
                a.append(r)
            elif r[0] < n:
                a.append((r[0], n - 1))
                b.append((n, r[1]))
            else:
                b.append(r)
    else:
        for r in orig:
            if r[0] > n:
                a.append(r)
            elif r[1] > n:
                b.append((r[0], n))
                a.append((n + 1, r[1]))
            else:
                b.append(r)

    clone = deepcopy(constraints)
    clone[c] = a
    constraints[c] = b
    return clone


def part2(input: str):
    rules = {}
    for r in input.splitlines():
        if not r:
            break
        toks = re.split(r"[{},]", r)
        rules[toks[0]] = toks[1:-1]

    acceptable = 0
    queue = [("in", [[(1, 4000)], [(1, 4000)], [(1, 4000)], [(1, 4000)]])]
    while queue:
        (name, constraints) = queue.pop()
        if name == "A":
            total = 1
            for c in constraints:
                total *= sum([r[1] - r[0] + 1 for r in c])
            acceptable += total
            continue
        if name == "R":
            continue

        r = rules[name]
        for sr in r:
            if ":" in sr:
                body, target = sr.split(":")
                i = "xmas".index(body[0])
                a = split_constraint(constraints, i, int(body[2:]), body[1])
                queue.append((target, a))
            else:
                queue.append((sr, constraints))
                break

    return acceptable


if __name__ == "__main__":
    main()
