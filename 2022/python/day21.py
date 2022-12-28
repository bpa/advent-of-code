#!/usr/bin/env python3

from aoc import *
from numbers import Number


def evil(monkey, all):
    if isinstance(all[monkey], int):
        return all[monkey]
    else:
        m1, op, m2 = all[monkey].split()
        a1 = evil(m1, all)
        a2 = evil(m2, all)
        if op == '+':
            return a1 + a2
        if op == '-':
            return a1 - a2
        if op == '*':
            return a1 * a2
        if op == '/':
            return a1 // a2
        return 1


def part1(input):
    monkeys = {}
    for line in input.splitlines():
        name, op = line.split(': ')
        if op.isnumeric():
            monkeys[name] = int(op)
        else:
            monkeys[name] = op
    return evil('root', monkeys)


def reduce(monkey, all):
    node = all.get(monkey, None)
    if node == None:
        return monkey
    if isinstance(node, Number):
        return node
    else:
        a1 = reduce(node[0], all)
        a2 = reduce(node[2], all)
        if isinstance(a1, Number) and isinstance(a2, Number):
            if node[1] == '=':
                return (a1, node[1], a2)
            if node[1] == '+':
                return a1 + a2
            if node[1] == '-':
                return a1 - a2
            if node[1] == '*':
                return a1 * a2
            if node[1] == '/':
                return a1 // a2
        else:
            return (a1, node[1], a2)


def left(a, op, b):
    if isinstance(a, int):
        return b, op, a, True
    else:
        return a, op, b, False


def balance(l, r):
    while l != 'humn':
        a, op, b, swapped = left(*l)
        if op == '+':
            r -= b
        elif op == '-':
            if swapped:
                r = b - r
            else:
                r += b
        elif op == '*':
            r //= b
        else:
            if swapped:
                r = b // r
            else:
                r *= b
        l = a
    return r


def part2(input):
    monkeys = {}
    for line in input.splitlines():
        name, order = line.split(': ')
        if order.isnumeric():
            monkeys[name] = int(order)
        else:
            m1, op, m2 = order.split()
            if name == 'root':
                monkeys[name] = (m1, '=', m2)
            else:
                monkeys[name] = (m1, op, m2)

    del monkeys['humn']
    l, _, r = reduce('root', monkeys)
    if isinstance(l, int):
        ans = balance(r, l)
    else:
        ans = balance(l, r)
    monkeys['humn'] = ans
    return ans


if __name__ == '__main__':
    main()
