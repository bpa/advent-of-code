#!/usr/bin/env python3

input = "../input/2020/day18.txt"

ops = {
    '+': lambda a, b: a + b,
    '-': lambda a, b: a - b,
    '*': lambda a, b: a * b,
    '/': lambda a, b: a // b,
}


def tokens(line):
    it = iter(line)
    for c in it:
        if c.isdigit():
            val = int(c)
            while True:
                try:
                    c = next(it)
                except:
                    c = ' '
                if c.isdigit():
                    val *= 10
                    val += int(c)
                else:
                    yield val
                    break
        if not c in '\r\n ':
            yield c


def _evaluate(tokens):
    acc = 0
    op = '+'
    for c in tokens:
        if c == '(':
            acc = ops[op](acc, _evaluate(tokens))
        elif c == ')':
            break
        elif c in ops.keys():
            op = c
        else:
            acc = ops[op](acc, c)

    return acc


def evaluate(line):
    return _evaluate(tokens(line))


with open(input) as file:
    print("Part 1:", sum([evaluate(l) for l in file.readlines()]))


def _evaluate_plus(tokens, i):
    t = tokens[i]
    i += 1
    if t == '(':
        acc, i = _evaluate_plus(tokens, i)
        i += 1
    else:
        acc = t

    while i < len(tokens):
        t = tokens[i]
        i += 1
        if t == ')':
            return acc, i-1

        if t == '+':
            if tokens[i] == '(':
                t, i = _evaluate_plus(tokens, i+1)
                i += 1
            else:
                t = tokens[i]
                i += 1
            acc += t

        else:
            mul, i = _evaluate_plus(tokens, i)
            acc *= mul

    return acc, i


def evaluate_plus(line):
    return _evaluate_plus(list(tokens(line)), 0)[0]


with open(input) as file:
    print("Part 2:", sum([evaluate_plus(l) for l in file.readlines()]))
