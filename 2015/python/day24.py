#!/usr/bin/env python3

from aoc import *


def solve(input, compartments):
    presents = [int(l) for l in input.splitlines()]
    presents.sort(reverse=True)
    desired = sum(presents) // compartments
    total_presents = len(presents)
    included = [False] * len(presents)
    (best_count, best_qe) = (1000, 1000)
    (i, count, qe, weight) = (0, 0, 1, 0)
    while True:
        while i < total_presents and weight < desired and count < best_count:
            weight += presents[i]
            count += 1
            qe *= presents[i]
            included[i] = True
            i += 1
        if weight == desired:
            if count < best_count or (count == best_count and qe < best_qe):
                best_count = count
                best_qe = qe
        while True:
            if i == 0:
                return best_qe
            i -= 1
            if included[i]:
                included[i] = False
                weight -= presents[i]
                count -= 1
                qe //= presents[i]
                i += 1
                break

    return best_qe


if __name__ == "__main__":
    main(3, 4)
