#!/usr/bin/env python3

from aoc import *


def look_and_say(word):
    expanded = ""
    count = 0
    curr = word[0]
    for c in word:
        if c == curr:
            count += 1
        else:
            expanded += str(count)
            expanded += curr
            curr = c
            count = 1
    expanded += str(count)
    expanded += curr
    return expanded


def part1(input):
    for _ in range(40):
        input = look_and_say(input)
    return len(input)


def test_expand():
    assert look_and_say("1") == "11"
    assert look_and_say("11") == "21"
    assert look_and_say("21") == "1211"
    assert look_and_say("1211") == "111221"
    assert look_and_say("111221") == "312211"


def part2(input):
    for _ in range(50):
        input = look_and_say(input)
    return len(input)


if __name__ == "__main__":
    main()
