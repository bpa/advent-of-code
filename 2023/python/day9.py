#!/usr/bin/env python3

from aoc import *


def predict(seq, forward):
    if not any(seq):
        return 0
    else:
        row = [b - a for a, b in itertools.pairwise(seq)]
        next_diff = predict(row, forward)
        if forward:
            return seq[-1] + next_diff
        else:
            return seq[0] - next_diff


def solve(input: str, forward):
    total = 0
    for seq in comb(input, r"-?\d+", int):
        total += predict(list(seq), forward)
    return total


if __name__ == "__main__":
    main(True, False)
