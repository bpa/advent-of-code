#!/usr/bin/env python3

from aoc import *


def part1(input: str):
    parse = comb(input, r"\d+", int)
    times = next(parse)
    distance = next(parse)
    answer = 1
    for t, d in zip(times, distance):
        count = 0
        for i in range(t):
            if (i * (t - i)) > d:
                count += 1
        answer *= count
    return answer


def part2(input: str):
    parse = comb(input, r"\d+")
    t = int("".join(next(parse)))
    d = int("".join(next(parse)))
    start = d // t
    while True:
        if (start * (t - start)) > d:
            break
        start += 1
    end = t
    while True:
        if (end * (t - end)) > d:
            break
        end -= 1
    print(f"Distance: {d}")
    start -= 1
    print(f"{start}: {start * (t-start)}")
    start += 1
    print(f"{start}: {start * (t-start)}")
    print(f"{end}: {end * (t-end)}")
    end += 1
    print(f"{end}: {end * (t-end)}")
    print(start, end)
    return end - start


# 25113600
if __name__ == "__main__":
    main()
