#!/usr/bin/env python3

import operator
from aoc import *
from functools import reduce


def part1(input: str):
    problems = []
    lines = input.splitlines()
    for line in lines[:-1]:
        row = line.split()
        if not problems:
            problems = [[] for _ in range(len(row))]
        for i, v in enumerate(row):
            problems[i].append(int(v))
    total = 0
    for i, op in enumerate(lines[-1].split()):
        problem = problems[i]
        match op:
            case '+':
                total += reduce(operator.add, problem)
            case '*':
                total += reduce(operator.mul, problem) 
            
    return total
    

def part2(input: str):
    lines = input.splitlines()
    total = 0
    op = '+'
    answer = 0
    for i, c in enumerate(lines[-1]):
        if c != ' ':
            total += answer
            op = c
            if c == '*':
                answer = 1
            else:
                answer = 0
        num = None
        for line in lines[:-1]:
            if line[i] != ' ':
                if num is None:
                    num = 0
                else:
                    num = num * 10
                num += int(line[i])
        if num is None:
            total += answer
            answer = 0
        else:
            if op == '+':
                answer += num
            else:
                answer *= num
    total += answer
    return total


if __name__ == '__main__':
    main()
