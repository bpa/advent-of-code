#!/usr/bin/env python3

from aoc import *


def solve(input, length):
    seen = defaultdict(int)
    for i in range(length):
        seen[input[i]] += 1
    start = 0
    end = length
    while len(seen) < length:
        if seen[input[start]] == 1:
            del seen[input[start]]
        else:
            seen[input[start]] -= 1
        start += 1
        seen[input[end]] += 1
        end += 1

    return end


if __name__ == '__main__':
    main(4, 14)


def test_solve():
    assert solve('bvwbjplbgvbhsrlpgdmjqwftvncz', 4) == 5
    assert solve('nppdvjthqldpwncqszvftbrmjlhg', 4) == 6
    assert solve('nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg', 4) == 10
    assert solve('zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw', 4) == 11
    assert solve('mjqjpqmgbljsphdztnvjfqwrcgsmlb', 14) == 19
    assert solve('bvwbjplbgvbhsrlpgdmjqwftvncz', 14) == 23
    assert solve('nppdvjthqldpwncqszvftbrmjlhg', 14) == 23
    assert solve('nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg', 14) == 29
    assert solve('zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw', 14) == 26
