#!/usr/bin/env python3

from aoc import *


def part1(input):
    numbers = [[int(n), i] for i, n in enumerate(input.splitlines())]
    l = len(numbers) - 1
    decrypted = numbers[:]
    zero = [0, 0]
    for n in numbers:
        if n[0] == 0:
            zero = n
        i = decrypted.index(n)
        j = (n[0]+i) % l
        z = decrypted.pop(i)
        decrypted.insert(j, z)
    i = decrypted.index(zero)
    return sum([decrypted[(1000*j+i) % len(numbers)][0] for j in range(1, 4)])


def part2(input):
    numbers = [[int(n)*811589153, i] for i, n in enumerate(input.splitlines())]
    l = len(numbers) - 1
    decrypted = numbers[:]
    zero = [0, 0]
    for _ in range(10):
        for n in numbers:
            if n[0] == 0:
                zero = n
            i = decrypted.index(n)
            j = (n[0]+i) % l
            z = decrypted.pop(i)
            decrypted.insert(j, z)
    i = decrypted.index(zero)
    return sum([decrypted[(1000*j+i) % len(numbers)][0] for j in range(1, 4)])


if __name__ == '__main__':
    main()
