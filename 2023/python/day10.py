#!/usr/bin/env python3

from aoc import *
from heapq import heappop, heappush


def part1(input: str):
    pipes = grid(input)
    s = pipes.index("S")["S"]
    seen = set([s])
    neighbors = []
    if n := s.n():
        if n in ["|", "7", "F"]:
            neighbors.append(n)
            seen.add(n)
    if n := s.e():
        if n in ["-", "J", "7"]:
            neighbors.append(n)
            seen.add(n)
    if n := s.s():
        if n in ["|", "L", "J"]:
            neighbors.append(n)
            seen.add(n)
    if n := s.w():
        if n in ["-", "L", "F"]:
            neighbors.append(n)
            seen.add(n)

    dist = 0
    adjacent = []
    while neighbors:
        dist += 1
        adjacent.clear()
        for n in neighbors:
            match n.get():
                case "|":
                    adjacent.extend([n.n(), n.s()])
                case "-":
                    adjacent.extend([n.e(), n.w()])
                case "L":
                    adjacent.extend([n.n(), n.e()])
                case "J":
                    adjacent.extend([n.n(), n.w()])
                case "7":
                    adjacent.extend([n.s(), n.w()])
                case "F":
                    adjacent.extend([n.s(), n.e()])
        neighbors.clear()
        for m in adjacent:
            if m not in seen:
                neighbors.append(m)
                seen.add(m)

    return dist


def part2(input: str):
    pipes = grid(input)
    s = pipes.index("S")["S"]
    seen = set([s])
    neighbors = []
    if n := s.n():
        if n in ["|", "7", "F"]:
            neighbors.append("n")
    if n := s.e():
        if n in ["-", "J", "7"]:
            neighbors.append("e")
    if n := s.s():
        if n in ["|", "L", "J"]:
            neighbors.append("s")
    if n := s.w():
        if n in ["-", "L", "F"]:
            neighbors.append("w")

    match neighbors:
        case ["n", "e"]:
            s.set("L")
        case ["n", "s"]:
            s.set("|")
        case ["n", "w"]:
            s.set("J")
        case ["e", "s"]:
            s.set("F")
        case ["e", "w"]:
            s.set("-")
        case ["s", "w"]:
            s.set("7")
    neighbors = [s]

    adjacent = []
    while neighbors:
        adjacent.clear()
        for n in neighbors:
            match n.get():
                case "|":
                    adjacent.extend([n.n(), n.s()])
                case "-":
                    adjacent.extend([n.e(), n.w()])
                case "L":
                    adjacent.extend([n.n(), n.e()])
                case "J":
                    adjacent.extend([n.n(), n.w()])
                case "7":
                    adjacent.extend([n.s(), n.w()])
                case "F":
                    adjacent.extend([n.s(), n.e()])
        neighbors.clear()
        for m in adjacent:
            if not m in seen:
                neighbors.append(m)
                seen.add(m)

    for p in pipes:
        if not p in seen:
            p.set(".")

    for p in pipes:
        if p != ".":
            start = p
            break

    debug("start", start)
    p = start
    d = 0
    captured = 0
    seen.clear()
    while p not in seen:
        seen.add(p)
        match p.get():
            case "|":
                if d == 0:
                    captured += capture(p.e())
                    p = p.n()
                else:
                    captured += capture(p.w())
                    p = p.s()
            case "-":
                if d == 1:
                    captured += capture(p.s())
                    p = p.e()
                else:
                    captured += capture(p.n())
                    p = p.w()
            case "L":
                if d == 2:
                    captured += capture(p.s())
                    captured += capture(p.w())
                    p = p.e()
                    d = 1
                else:
                    p = p.n()
                    d = 0
            case "J":
                if d == 1:
                    captured += capture(p.s())
                    captured += capture(p.e())
                    p = p.n()
                    d = 0
                else:
                    p = p.w()
                    d = 3
            case "7":
                if d == 1:
                    p = p.s()
                    d = 2
                else:
                    captured += capture(p.n())
                    captured += capture(p.e())
                    p = p.w()
                    d = 3
            case "F":
                if d == 0:
                    p = p.e()
                    d = 1
                else:
                    captured += capture(p.n())
                    captured += capture(p.w())
                    p = p.s()
                    d = 2
    debug(pipes)
    return captured


def capture(p):
    if not p:
        return 0
    captured = 0
    queue = [p]
    while queue:
        n = queue.pop()
        if n and n == ".":
            captured += 1
            n.set("I")
            queue.extend(n.adjacent_4())
    return captured


if __name__ == "__main__":
    main()
