#!/usr/bin/env python3

from itertools import combinations_with_replacement
from aoc import *


def dist(a, b):
    return (a[0] - b[0])**2 + (a[1] - b[1])**2 + (a[2] - b[2])**2


def match(a, b):
    if a[0] == b[0]:
        return (a[0], a[1], b[1])
    elif a[0] == b[1]:
        return (a[0], a[1], b[0])
    elif a[1] == b[0]:
        return (a[1], a[0], b[1])
    else:
        return (a[1], a[0], b[0])


def update_orientation(translate, scanner, a0, b0, a1, b1):
    pairs = list(zip(match(a0, a1), match(b0, b1)))
    for v in itertools.combinations(pairs, 2):
        p0 = translate(v[0][0])
        p1 = translate(v[1][0])
        q0 = v[0][1]
        q1 = v[1][1]
        a = [p0[i] - p1[i] for i in range(3)]
        b = [q0[i] - q1[i] for i in range(3)]
        am = [abs(a[i]) for i in range(3)]
        bm = [abs(b[i]) for i in range(3)]
        if set(am) == set(bm) and len(set(am)) == 3:
            orientation = []
            for i in range(3):
                for j in range(3):
                    if am[i] == bm[j]:
                        if a[i] == b[j]:
                            orientation.append(j+1)
                        else:
                            orientation.append(-(j+1))
                        break
            location = []
            for i in range(3):
                j = abs(orientation[i])
                m = orientation[i] // j
                location.append(p0[i] - q0[j-1] * m)
            scanner['orientation'] = tuple(orientation)
            scanner['location'] = tuple(location)
            return True
    return False


def think(known, unknown):
    common = known['distances'].intersection(unknown['distances'])
    if len(common) < 12:
        return False
    translate = translator(known)
    dist0 = known['vertex']
    dist1 = unknown['vertex']
    for k in common:
        a0 = dist0[k]
        b0 = dist1[k]
        for d in known['neighbors'][a0[0]]:
            if d != k and d in common:
                a1 = dist0[d]
                b1 = dist1[d]
                if update_orientation(translate, unknown, a0, b0, a1, b1):
                    return True
    return False


def translator(scanner):
    orientation = scanner['orientation']
    location = scanner['location']
    m = [0, 0, 0]
    s = [0, 0, 0]
    for (i, o) in enumerate(orientation):
        a = abs(o)
        m[i] = a - 1
        s[i] = o // a

    def translate(b):
        return tuple([b[m[i]] * s[i] + location[i] for i in range(3)])

    return translate


def solve():
    scanners = []
    for scanner in puzzle.blocks():
        lines = scanner.splitlines()
        beacons = {tuple(int(i) for i in b.split(',')) for b in lines[1:]}
        distances = set()
        vertex = {}
        neighbors = defaultdict(dict)
        for (a, b) in itertools.combinations(beacons, 2):
            distance = dist(a, b)
            distances.add(distance)
            vertex[distance] = (a, b)
            neighbors[a][distance] = b
            neighbors[b][distance] = a
        scanners.append({
            "location": (0, 0, 0),
            "orientation": (1, 2, 3),
            "beacons": beacons,
            "vertex": vertex,
            "neighbors": neighbors,
            "distances": distances})

    calibrated = [scanners[0]]
    unknown = scanners[1:]
    beacons = set(calibrated[0]['beacons'])
    while unknown:
        resolved = []
        work = unknown
        unknown = []
        for scanner in work:
            for known in calibrated:
                if think(known, scanner):
                    translate = translator(scanner)
                    for b in sorted(scanner['beacons']):
                        beacons.add(translate(b))

                    resolved.append(scanner)
                    break
            else:
                unknown.append(scanner)
        calibrated.extend(resolved)

    print("Part 1:", len(beacons))

    max_d = 0
    for (a, b) in itertools.combinations(calibrated, 2):
        d = sum([abs(a['location'][i] - b['location'][i]) for i in range(3)])
        if d > max_d:
            max_d = d
    print("Part 2:", max_d)


if __name__ == '__main__':
    solve()
