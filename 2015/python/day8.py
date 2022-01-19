#!/usr/bin/env python3

from aoc import *


def part1(input):
    code = 0
    mem = 0
    for line in input.splitlines():
        chars = iter(line)
        c = next(chars)
        code += 1
        while c != '"':
            c = next(chars)
            code += 1
        while True:
            c = next(chars)
            code += 1
            if c == '"':
                break
            mem += 1
            if c == '\\':
                c = next(chars)
                code += 1
                if c == 'x':
                    c = next(chars)
                    c = next(chars)
                    code += 2
    return code - mem


def part2(input):
    answer = 0
    for line in input.splitlines():
        answer += 2
        for c in line:
            if c in '\\"':
                answer += 1
    return answer


if __name__ == '__main__':
    main()
