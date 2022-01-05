#!/usr/bin/env python3

from aoc import *
from functools import partial


def stack(i):
    num = []
    while i > 0:
        c = i % 26
        num.insert(0, chr(65+c))
        i //= 26
    return ''.join(num)


def alu(code):
    program_input = []
    registers = [0] * 4
    instructions = []
    sta = ''
    ix = 0

    def inp(r):
        nonlocal program_input, registers, sta, ix
        registers[r] = int(next(program_input))
        # n = stack(registers[3])
        # if len(n) == len(sta):
        #     print(ix, "is wrong", sta, n)
        # else:
        #     sta = n
        # ix += 1
        # print(registers)

    def add(a, b):
        registers[a] = registers[a] + b

    def addr(a, b):
        registers[a] = registers[a] + registers[b]

    def mul(a, b):
        registers[a] = registers[a] * b

    def mulr(a, b):
        registers[a] = registers[a] * registers[b]

    def div(a, b):
        registers[a] = registers[a] // b

    def divr(a, b):
        registers[a] = registers[a] // registers[b]

    def mod(a, b):
        registers[a] = registers[a] % b

    def modr(a, b):
        registers[a] = registers[a] % registers[b]

    def eql(a, b):
        registers[a] = 1 if registers[a] == b else 0

    def eqlr(a, b):
        registers[a] = 1 if registers[a] == registers[b] else 0

    lookup = {
        'add': [add, addr],
        'mul': [mul, mulr],
        'div': [div, divr],
        'mod': [mod, modr],
        'eql': [eql, eqlr],
    }

    def r(a):
        return ord(a) - ord('w')

    for line in code:
        (inst, *args) = line.split()
        if inst == 'inp':
            instructions.append(partial(inp, r(args[0])))
        else:
            i = lookup[inst]
            (a, b) = args
            try:
                v = int(b)
                instructions.append(partial(i[0], r(a), v))
            except:
                instructions.append(partial(i[1], r(a), r(b)))

    def run(input):
        nonlocal program_input, registers, instructions
        registers = [0] * 4
        program_input = iter(input)
        for i in instructions:
            i()
        return registers

    return run


def find_pair(input, a, b, cpu, z):
    for i in range(9, 0, -1):
        input[a] = str(i)
        for j in range(9, 0, -1):
            input[b] = str(j)
            st = cpu(input)[3]
            l = stack(st)
            if len(l) == 6 - z:
                print(l)
                return
    print("failed:", a, b)
    print(l)


def part1():
    cpu = alu(puzzle.lines())

    #             01234567890123
    input = list('41299994879959')
    input = list('11189561113216')
    print(stack(cpu(input)[3]))
    # , (1, 4), (6, 7), (9, 10), (8, 11), (5, 12), (0, 13)]):
    # for (i, (a, b)) in enumerate([(1, 2)]):
    #     find_pair(input, a, b, cpu, i)

    # with open('input/2021/day24.test') as file:
    #     input = list(file.read())[:14]
    return ''.join(input)


def part2():
    answer = 0
    for line in puzzle.lines():
        pass
    return answer


print("Part 1:", part1())
print("Part 2:", part2())
