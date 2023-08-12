#!/usr/bin/env python3

from aoc import *

SKIP = [ord(c) - ord("a") for c in ["i", "o", "l"]]


def is_valid(password):
    """Does not check for i, l, o because generator will not create them"""
    has_sequence = False
    seq_len = 0
    doubles = set()
    curr = -2
    for c in password:
        if c == curr:
            doubles.add(c)
        if c == curr + 1:
            seq_len += 1
            if seq_len == 3:
                has_sequence = True
        else:
            seq_len = 1
        curr = c
    if len(doubles) > 1 and has_sequence:
        return True
    else:
        return False


def test_banned():
    assert to_str(inc_banned(to_arr("abcdefgi"))) == "abcdefgj"
    assert to_str(inc_banned(to_arr("abcdeogi"))) == "abcdepaa"
    assert to_str(inc_banned(to_arr("lbcdeogi"))) == "maaaaaaa"
    assert to_str(inc_banned(to_arr("ghijklmn"))) == "ghjaaaaa"


def test_valid():
    assert is_valid(to_arr("hijklmmn")) == False
    assert is_valid([ord(c) for c in "hijkkmmn"]) == True
    assert is_valid([ord(c) for c in "hijkkhkk"]) == False
    assert is_valid([ord(c) for c in "abbceffg"]) == False
    assert is_valid([ord(c) for c in "abbcegjk"]) == False
    assert is_valid([ord(c) for c in "abcdefhh"]) == False
    assert is_valid(to_arr("ghjaabcc")) == True


def test_next():
    assert part1("abcdefgh") == "abcdffaa"
    assert part1("ghijklmn") == "ghjaabcc"


def to_arr(chars):
    return [ord(c) - ord("a") for c in chars]


def to_str(chars):
    return "".join([chr(c + ord("a")) for c in chars])


def inc_banned(chars):
    for i in range(8):
        if chars[i] in SKIP:
            chars[i] += 1
            i += 1
            while i < 8:
                chars[i] = 0
                i += 1
    return chars


def inc_password(chars):
    i = 7
    while True:
        x = chars[i] + 1
        if x in SKIP:
            x = x + 1
        if x > 25:
            chars[i] = 0
            if i == 0:
                i = 7
            else:
                i = i - 1
            continue
        chars[i] = x
        if is_valid(chars):
            return chars
        i = 7


def part1(input):
    chars = to_arr(input)
    inc_banned(chars)
    inc_password(chars)
    return to_str(chars)


def part2(input):
    return part1(part1(input))


if __name__ == "__main__":
    main()
