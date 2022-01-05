#!/usr/bin/env python3

from aoc import *
from heapq import heappop, heappush
from copy import deepcopy


def open(hall, a, b):
    b = 2 + 2 * b
    if a > b:
        (a, b) = (b, a)
    for i in range(a+1, b):
        if hall[i] != None:
            return False
    return True


class State:
    def __init__(self, corridors):
        self.size = len(corridors[0]) + 1
        self.hall = [None] * 11
        self.depth = [None] * 4
        self.energy = 0
        self.corridors = corridors
        self.required = self.required_energy()

    def required_energy(self):
        energy = 0
        depths = [self.size-1 for _ in range(4)]
        for c in range(4):
            diff = self.size - len(self.corridors[c]) - 1
            for i in range(len(self.corridors[c])-1, -1, -1):
                if self.corridors[c][i] == c:
                    depths[c] = i + diff
                else:
                    break

        for (i, s) in enumerate(self.corridors):
            for (d, c) in enumerate(s):
                diff = self.size - len(self.corridors[i]) - 1
                if c != i or d + diff < depths[i]:
                    dist = max(2, 2 * abs(c - i))
                    depth = depths[c] + d + 1 + diff
                    depths[c] -= 1
                    energy += 10**c * (dist + depth)

        for (i, c) in enumerate(self.hall):
            if c != None:
                energy += 10**c * (depths[c] + abs(i - (2 + 2 * c)))
                depths[c] -= 1
        return energy

    def __iter__(self):
        for (i, c) in enumerate(self.hall):
            if c != None and self.depth[c] != None and open(self.hall, i, c):
                moves = self.depth[c] + abs(i - (2 + 2 * c))
                after = deepcopy(self)
                after.hall[i] = None
                after.depth[c] -= 1
                after.corridors[c].insert(0, c)
                energy_used = 10**c * moves
                after.energy += energy_used
                after.required -= energy_used
                yield after
        for (i, c) in enumerate(self.corridors):
            if self.depth[i] == None:
                for h in [0, 1, 3, 5, 7, 9, 10]:
                    if self.hall[h] == None and open(self.hall, h, i):
                        moves = self.size - len(c) + abs(h - (2 + 2 * i))
                        after = deepcopy(self)
                        shell = after.corridors[i].pop(0)
                        after.hall[h] = shell
                        energy_used = 10**shell * moves
                        after.energy += energy_used
                        x = 2 + 2 * shell
                        req = max(2, 2 * abs(shell - i)) + \
                            self.size - len(c) - abs(x-h)
                        after.required -= 10**shell * req
                        if all(y == i for y in after.corridors[i]):
                            after.depth[i] = self.size - len(c)
                        yield after

    def __hash__(self):
        return hash(tuple(self.hall)) * 57 + hash(str(self.corridors))

    def __eq__(self, other):
        return self.hall == other.hall and self.corridors == other.corridors

    def __lt__(self, other):
        f = self.energy + self.required
        o = other.energy + other.required
        if f == o:
            return self.required < other.required
        else:
            return f < o

    def __str__(self):
        from io import StringIO
        s = StringIO()
        s.write(
            f'E: {self.energy} R: {self.required} F: {self.energy + self.required}')
        s.write("\n")
        s.write("#" * 13)
        s.write("\n#")
        for e in self.hall:
            if e != None:
                s.write(chr(65 + e))
            else:
                s.write('.')
        s.write("#\n")
        for d in range(self.size):
            s.write("##")
            for c in range(4):
                s.write('#')
                if len(self.corridors[c]) > d:
                    s.write(chr(self.corridors[c][d]+65))
                else:
                    s.write('.')
            s.write('###\n')
        s.write("\n")
        return s.getvalue()


def solve(corridors):
    seen = set()
    queue = [State(corridors)]
    while queue:
        state = heappop(queue)
        if state.required == 0:
            return state.energy
        for s in state:
            if not s in seen:
                seen.add(s)
                heappush(queue, s)
    return None


def corridors(lines, extra=False):
    next(lines)
    next(lines)
    stack = []
    stack.append(
        [ord(c) - ord('A') for c in re.findall('\w', next(lines))])
    if extra:
        stack.append([3, 2, 1, 0])
        stack.append([3, 1, 0, 2])
    stack.append(
        [ord(c) - ord('A') for c in re.findall('\w', next(lines))])
    ret = [[0] * len(stack) for _ in range(4)]
    for (i, row) in enumerate(stack):
        for (j, c) in enumerate(row):
            ret[j][i] = c
    return ret


def part1():
    return solve(corridors(puzzle.lines()))


def part2():
    return solve(corridors(puzzle.lines(), True))


print("Part 1:", part1())
print("Part 2:", part2())
