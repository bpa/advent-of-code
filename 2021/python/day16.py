#!/usr/bin/env python3

from io import StringIO
from aoc import *


def to_num(s):
    v = 0
    for i in s:
        v *= 2
        if i == '1':
            v += 1
    return v


op = {
    0: sum,
    1: math.prod,
    2: min,
    3: max,
    5: lambda a: 1 if a[0] > a[1] else 0,
    6: lambda a: 1 if a[0] < a[1] else 0,
    7: lambda a: 1 if a[0] == a[1] else 0,
}


def read_packet(f, cb):
    v = to_num(f[:3])
    cb(v)
    t = to_num(f[3:6])
    if t == 4:
        i = 6
        literal = StringIO()
        l = 1
        while l:
            l = int(f[i])
            i += 1
            literal.write(f[i:i+4])
            i += 4
        return (i, to_num(literal.getvalue()))
    else:
        values = []
        length_or_count = int(f[6])
        if length_or_count == 0:
            subpackets = to_num(f[7:22])
            sp = 22
            i = sp + subpackets
            while sp < i:
                (length, value) = read_packet(f[sp:i], cb)
                values.append(value)
                sp += length
        elif length_or_count == 1:
            sp = to_num(f[7:18])
            i = 18
            for _ in range(sp):
                (length, value) = read_packet(f[i:], cb)
                values.append(value)
                i += length
        return (i, op[t](values))


def to_binary(f):
    s = StringIO()
    for c in f.read():
        s.write("{0:04b}".format(int(c, 16)))
    return s.getvalue()


def part1():
    with open(sys.argv[1]) as file:
        input = to_binary(file)
    total = 0

    def add(p):
        nonlocal total
        total += p
    read_packet(input, add)
    return total


def part2():
    with open(sys.argv[1]) as file:
        input = to_binary(file)

    def nop(a):
        pass
    (read, value) = read_packet(input, nop)
    return value


print("Part 1:", part1())
print("Part 2:", part2())
