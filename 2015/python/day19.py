#!/usr/bin/env python3

from aoc import *


def expand(molecule, replacements):
    possible = set()
    for s, replace in delimited(replacements, " => "):
        token_len = len(s)
        for i in range(len(molecule) - token_len + 1):
            if molecule[i : i + token_len] == s:
                possible.add(f"{molecule[:i]}{replace}{molecule[i+token_len:]}")
    return possible


def part1(input):
    replacements, molecule = input.split("\n\n")
    return len(expand(molecule, replacements))


def devolve(m, possible, map):
    for i in range(len(m)):
        root = map.root
        for j in range(i, len(m)):
            root = root.get(m[j])
            if not root:
                break
            v = root.get("_value")
            if v:
                n = m[:i]
                n += v
                if j < len(m) - 1:
                    n += m[j + 1 :]
                possible.add(n)
                if n == "e":
                    return True


def test_devolve():
    map = WordMap()
    map.add("hi", "b")
    map.add("h", "a")
    possible = set()
    devolve("hiho", possible, map)
    assert possible == {"bho", "aiho", "hiao"}


def part2(input):
    replacements, molecule = input.split("\n\n")
    molecule = molecule.strip()

    subs = sorted(
        [(s, r) for s, r in delimited(replacements, " => ")], key=lambda x: -len(x[1])
    )
    rounds = 0
    while molecule != "e":
        rounds += 1
        for m in subs:
            if molecule.find(m[1]) > -1:
                molecule = molecule.replace(m[1], m[0], 1)
                break
    return rounds


if __name__ == "__main__":
    main()
