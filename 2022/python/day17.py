#!/usr/bin/env python3

from aoc import *

PIECES = [
    [[0b1111], 4],
    [[0b010, 0b111, 0b010], 3],
    [[0b111, 0b100, 0b100], 3],
    [[1, 1, 1, 1], 1],
    [[3, 3], 2]
]


def print_cave(cave, top):
    debug(top * 9)
    for r in reversed(cave):
        s = ''
        for i in range(7):
            m = 1 << i
            if m & r == 0:
                s += ' '
            else:
                s += '#'
        debug(f'{top}{s}{top}')
    debug(top * 9)


class Piece:
    def __init__(self, i, bits, width, height, cave):
        self.i = i
        self.bits = bits
        self.width = width
        self.x = 2
        self.y = height
        self.cave = cave
        self._shift(2)

    def fall(self):
        self.y -= 1
        ok = True
        if not self.is_valid():
            self.y += 1
            ok = False
        return ok

    def apply(self):
        for i, r in enumerate(self.bits):
            self.cave[self.y + i] |= r

    def is_valid(self):
        for i, r in enumerate(self.bits):
            if self.cave[self.y + i] & r:
                return False
        return True

    def push(self, o):
        x = self.x + o
        if x < 0 or x > (7 - self.width):
            return
        self._shift(o)
        if self.is_valid():
            self.x = x
        else:
            self._shift(-o)

    def top(self):
        return self.y + len(self.bits) - 1

    def _shift(self, o):
        for j, v in enumerate(self.bits):
            if o > 0:
                self.bits[j] = v << o
            else:
                self.bits[j] = v >> -o


def supply():
    piece_index = 0

    def next(cave: List[int], height):
        nonlocal piece_index
        piece = PIECES[piece_index]
        piece_index = (piece_index + 1) % len(PIECES)
        new_height = height + 1 + len(piece[0])
        if len(cave) < new_height:
            cave.extend([0] * (new_height - len(cave)))
        return Piece(piece_index, piece[0][:], piece[1], height + 1, cave)
    return next


class Input:
    def __init__(self, input):
        self.input = input.strip()
        self.i = 0
        self.l = len(self.input)

    def next(self):
        i = self.i
        c = self.input[i]
        self.i = (i + 1) % self.l
        return c


def solve(input, count):
    cave = [0xFF]
    height = 0
    next_piece = supply()
    moves = Input(input)
    piece = Piece(0, [0], 1, 0, cave)
    cache = {}
    cache_height = 0
    while count:
        count -= 1
        piece = next_piece(cave, height)

        d = 0
        right_bound = 5 - piece.width
        for _ in range(4):
            c = moves.next()
            if c == '<':
                if d > -2:
                    d -= 1
            else:
                if d < right_bound:
                    d += 1
        piece.push(d)
        while piece.fall():
            c = moves.next()
            if c == '<':
                piece.push(-1)
            else:
                piece.push(1)
        height = max(height, piece.top())
        piece.apply()

        k = (piece.i, moves.i)
        if k in cache:
            entry = cache[k]

            diff_height = height - entry[0]
            diff_count = entry[1] - count
            if diff_height != entry[2] or diff_count != entry[3]:
                cache[k] = (height, count, diff_height, diff_count)
                continue

            skip = count // diff_count
            if skip:
                count -= diff_count * skip
                cache_height += diff_height * skip
        else:
            cache[k] = (height, count, 0, 0)

    return height + cache_height


if __name__ == '__main__':
    main(2022, 1000000000000)
