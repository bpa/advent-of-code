#!/usr/bin/env python3

from aoc import *
dirs = [Point.e, Point.s, Point.w, Point.n]
wrap = [
    lambda p: p.grid.at(0, p.y),
    lambda p: p.grid.at(p.x, 0),
    lambda p: p.grid.at(p.grid.width - 1, p.y),
    lambda p: p.grid.at(p.x, p.grid.height - 1)
]


def walk(p, steps, dir):
    d = dirs[dir]
    w = wrap[dir]
    for _ in range(steps):
        n = d(p)
        if not n:
            n = w(p)
        while n.get() == '~':
            o = d(n)
            if not o:
                o = w(n)
            n = o
        if n.get() == '#':
            return p
        p = n
    return p


def part1(input):
    grove, code = input.split('\n\n')
    code = code.strip()
    width = max(len(l) for l in grove.splitlines())
    pad = '~' * width
    grove = '\n'.join([(l + pad)[:width] for l in grove.splitlines()])
    grove = grid(grove.replace(' ', '~'))

    p = grove.at(0, 0)
    dir = 0
    while p.get() != '.':
        p = p.e()
    i = 0
    while i < len(code):
        if code[i] == 'R':
            dir = (dir+1) % 4
            i += 1
        elif code[i] == 'L':
            dir = (dir-1) % 4
            i += 1
        else:
            steps = 0
            while i < len(code) and code[i].isnumeric():
                steps *= 10
                steps += int(code[i])
                i += 1
            p = walk(p, steps, dir)

    return 1000 * (p.y + 1) + 4 * (p.x + 1) + dir


@dataclass
class Region:
    x: int
    y: int
    sides: List = field(default_factory=lambda: [None, None, None, None])
    is_face: bool = False

    def __str__(self):
        return f'({self.x}, {self.y}) {self.sides}'


def link_face(regions, f0, f1, dir0, rot0):
    if f1 < 0 or not regions[f1].is_face or regions[f0].sides[dir0]:
        return

    r0 = regions[f0]
    r0.sides[dir0] = (f1, rot0)

    r1 = regions[f1]
    dir1 = dir0 + rot0
    l = (dir1+3) % 4
    b = (dir1+2) % 4
    r = (dir1+1) % 4
    sr = r1.sides[r]
    if sr:
        link_face(regions, f0, sr[0], (dir0+1) % 4, (sr[1]+3 + rot0) % 4)
    sl = r1.sides[l]
    if sl:
        link_face(regions, f0, sl[0], (dir0+3) % 4, (sl[1]+1 + rot0) % 4)
    link_face(regions, f1, f0, b, rot0 if rot0 % 2 == 0 else (rot0 + 2) % 4)


def test_link_face():
    faces = [Region(x % 4, x//4) for x in range(16)]

    def assert_face(face, links):
        nonlocal faces
        assert faces[face].sides == links

    def assert_link(face, links):
        nonlocal faces
        faces[face].is_face = True
        if face % 4 > 0:
            link_face(faces, face, face - 1, 2, 0)
        link_face(faces, face, face - 4, 3, 0)
        for f in range(16):
            if f in links:
                assert_face(f, links[f])
            else:
                assert_face(f, [None, None, None, None])

    assert_link(2, {})
    assert_link(4, {})

    assert_link(5, {
        4: [(5, 0), None,  None,  None],
        5: [None,   None, (4, 0), None],
    })

    assert_link(6, {
        2: [None,   (6, 0), (5, 3), (4, 2)],
        4: [(5, 0),  None,   None,  (2, 2)],
        5: [(6, 0),  None,  (4, 0), (2, 1)],
        6: [None,    None,  (5, 0), (2, 0)]
    })

    assert_link(10, {
        2:  [None,   (6, 0),  (5, 3), (4, 2)],
        4:  [(5, 0), (10, 2),  None,  (2, 2)],
        5:  [(6, 0), (10, 3), (4, 0), (2, 1)],
        6:  [None,   (10, 0), (5, 0), (2, 0)],
        10: [None,   (4, 2),  (5, 1), (6, 0)],
    })

    assert_link(11, {
        2:  [(11, 2), (6, 0),  (5, 3),  (4, 2)],
        4:  [(5, 0),  (10, 2), (11, 1), (2, 2)],
        5:  [(6, 0),  (10, 3), (4, 0),  (2, 1)],
        6:  [(11, 1), (10, 0), (5, 0),  (2, 0)],
        10: [(11, 0), (4, 2),  (5, 1),  (6, 0)],
        11: [(2, 2),  (4, 3),  (10, 0), (6, 3)],
    })


STEP = [(1, 0), (0, 1), (-1, 0), (0, -1)]


def switch_face(x0, y0, x1, y1, dir0, size, regions):
    r0 = regions[4 * (y0 // size) + x0 // size]
    (r1, rotation) = r0.sides[dir0]

    if rotation % 2 == 1:
        (x1, y1) = (y1, x1)
    rx = r1 % 4
    ry = r1 // 4
    xm = x1 % size
    ym = y1 % size
    dir1 = (dir0 + rotation) % 4
    if (dir0 in [0, 3] and dir1 not in [0, 3]) or (dir0 in [1, 2] and dir1 not in [1, 2]):
        xm = size - xm - 1
        ym = size - ym - 1
    if dir1 == 0:
        return rx * size, ry * size + ym, dir1
    if dir1 == 1:
        return rx * size + xm, ry * size, dir1
    if dir1 == 2:
        return (rx + 1) * size - 1, ry * size + ym, dir1
    else:
        return rx * size + xm, (ry+1) * size - 1, dir1


def walk_cube(x0, y0, steps, dir0, size, grove, regions):
    for _ in range(steps):
        x1 = x0 + STEP[dir0][0]
        if x1 < 0:
            x1 += size * 4
        if x1 == size * 4:
            x1 = 0
        y1 = y0 + STEP[dir0][1]
        if y1 < 0:
            y1 += size * 4
        if y1 == size * 4:
            y1 = 0
        dir1 = dir0
        if dir0 == 0 and x1 % size == 0 \
                or dir0 == 1 and y1 % size == 0 \
                or dir0 == 2 and x0 % size == 0 \
                or dir0 == 3 and y0 % size == 0:
            x1, y1, dir1 = switch_face(x0, y0, x1, y1, dir0, size, regions)
        if grove[y1][x1] != '.':
            return x0, y0, dir0
        else:
            x0 = x1
            y0 = y1
            dir0 = dir1

    return x1, y1, dir0


def part2(input):
    grove, code = input.split('\n\n')
    grove = grove.splitlines()
    code = code.strip()
    size = 4 if len(grove) < 50 else 50
    row = 0
    regions = [Region(x % 4, x//4) for x in range(16)]
    for row in range(4):
        if row * size >= len(grove):
            break
        y = size * row
        for col in range(4):
            x = col * size
            if x >= len(grove[y]):
                break
            if grove[y][x] == ' ':
                continue
            ind = row * 4 + col
            regions[ind].is_face = True
            link_face(regions, ind, ind - 4, 3, 0)
            if ind % 4 > 0:
                link_face(regions, ind, ind - 1, 2, 0)

    first_region = next(filter(lambda x: x.is_face, regions))
    x = first_region.x * size
    y = first_region.y * size
    dir = 0
    i = 0
    while i < len(code):
        if code[i] == 'R':
            dir = (dir+1) % 4
            i += 1
        elif code[i] == 'L':
            dir = (dir-1) % 4
            i += 1
        else:
            steps = 0
            while i < len(code) and code[i].isnumeric():
                steps *= 10
                steps += int(code[i])
                i += 1
            x, y, dir = walk_cube(x, y, steps, dir, size, grove, regions)

    return 1000 * (y + 1) + 4 * (x + 1) + dir


if __name__ == '__main__':
    main()
