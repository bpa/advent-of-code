#!/usr/bin/env python3

from collections import defaultdict
import math
import re
# re.findall(exp, text)
# re.search() -> anywhere
# re.match -> beginning
# re.split(exp, text)
# re.compile(exp)

input = "../input/2020/day16.txt"
# input = "test.txt"

answer = 0
keys = []
fields = set()
field_ranges = {}


def is_valid(value):
    for f in keys:
        if value >= f[0]:
            if value <= f[1]:
                return True
        else:
            return False
    return False


with open(input) as file:
    line = file.readline()
    while not line.isspace():
        line.strip()
        key, ranges = line.split(': ')
        fields.add(key)
        first, second = ranges.split(' or ')
        a, b = first.split('-')
        keys.append((int(a), int(b), key))
        c, d = second.split('-')
        keys.append((int(c), int(d), key))
        field_ranges[key] = [int(a), int(b), int(c), int(d)]
        line = file.readline()

    keys = sorted(keys)
    file.readline()  # your ticket:
    personal = [int(v) for v in file.readline().strip().split(',')]
    potential_fields = {f: set(list(range(len(personal)))) for f in fields}

    errors = 0
    file.readline()  # Blank line
    file.readline()  # nearby tickets:
    valid_tickets = []
    for ticket in file.readlines():
        valid = True
        for field in ticket.split(','):
            if not is_valid(int(field)):
                errors += int(field)
                valid = False
        if valid:
            valid_tickets.append([int(f) for f in ticket.split(',')])

    for ticket in valid_tickets:
        for field, value in enumerate(ticket):
            for k, v in potential_fields.items():
                if field in v:
                    r = field_ranges[k]
                    if not (r[0] <= value <= r[1] or r[2] <= value <= r[3]):
                        v.remove(field)

    solving = set(potential_fields.keys())
    while len(solving) > 0:
        seen = defaultdict(set)
        solved = set()
        for f in solving:
            field_possibilities = potential_fields[f]
            for pos in field_possibilities:
                seen[pos].add(f)
            if len(field_possibilities) == 1:
                solved.add(f)
        for k, v in seen.items():
            if len(v) == 1:
                solved.union(v)
                potential_fields[next(iter(v))] = set([k])

        solving.difference_update(solved)
        for s in solved:
            to_remove = potential_fields[s]
            for remaining in solving:
                potential_fields[remaining].difference_update(to_remove)

print("Part 1:", errors)
answer = 1
for k, v in potential_fields.items():
    if k.startswith('departure'):
        print(next(iter(v)), personal[next(iter(v))])
        answer *= personal[next(iter(v))]

print("Part 2:", answer)
