#!/usr/bin/env python3

from aoc import *


def range(line):
    return [int(i) for i in line.split('-')]


def part1(input):
    answer = 0
    for l in delimited(input, ',', range):
        if l[0][0] <= l[1][0] and l[0][1] >= l[1][1]:
            answer += 1
        elif l[0][0] >= l[1][0] and l[0][1] <= l[1][1]:
            answer += 1

    return answer


def part2(input):
    answer = 0
    for l in delimited(input, ',', range):
        if l[0][0] >= l[1][0] and l[0][0] <= l[1][1]:
            answer += 1
        elif l[0][1] >= l[1][0] and l[0][1] <= l[1][1]:
            answer += 1
        elif l[1][0] >= l[0][0] and l[1][0] <= l[0][1]:
            answer += 1
        elif l[1][1] >= l[0][0] and l[1][1] <= l[0][1]:
            answer += 1

    return answer


if __name__ == '__main__':
    main()


def test_part1():
    assert part1('1-1,1-1') == 1
    assert part1('1-2,1-1') == 1
    assert part1('1-1,1-2') == 1
    assert part1('1-3,2-2') == 1
    assert part1('2-2,1-3') == 1
    assert part1('1-5,1-2') == 1
    assert part1('1-2,1-5') == 1
    assert part1('1-2,2-5') == 0
    assert part1('2-5,1-2') == 0
    assert part1('2-5,1-1') == 0
    assert part1('1-1,2-5') == 0
    assert part1('1-3,2-5') == 0
    assert part1('2-5,1-3') == 0


def test_part2():
    assert part2('1-1,1-1') == 1
    assert part2('1-2,1-1') == 1
    assert part2('1-1,1-2') == 1
    assert part2('1-3,2-2') == 1
    assert part2('2-2,1-3') == 1
    assert part2('1-5,1-2') == 1
    assert part2('1-2,1-5') == 1
    assert part2('1-2,2-5') == 1
    assert part2('2-5,1-2') == 1
    assert part2('2-5,1-1') == 0
    assert part2('1-1,2-5') == 0
    assert part2('1-3,2-5') == 1
    assert part2('2-5,1-3') == 1
