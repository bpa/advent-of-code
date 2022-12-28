#!/usr/bin/env python3

from aoc import *
from heapq import heappush, heappop

names = []
distances = []
valves = {}
(BOTH_READY, IM_TRAVELING, HES_TRAVELING, DONE) = range(4)
states = ['BOTH', '<---', '--->', 'DONE']


class Node:
    def __init__(self, index, name, parent, time, flows, current):
        global distances
        self.index = index
        self.name = name
        self.parent = parent
        self.time = time
        self.flows = flows
        self.current = current
        times = [time - distances[index][i] for i in range(len(flows))]
        self.potential = current + \
            sum([v * t for v, t in zip(flows, times)])

    def __repr__(self):
        return f'{self.potential}|{self.time}|{self.name}'

    def __str__(self):
        return f'{self.name} t: {self.time} current: {self.current} potential: {self.potential}'

    def __lt__(self, other):
        if self.potential == other.potential:
            return self.time < other.time
        else:
            return self.potential > other.potential


@dataclass
class Elephant:
    def __init__(self, me, elephant, parent, current, time, state, travel, next_flow, flows, opened):
        self.me = me
        self.elephant = elephant
        self.parent = parent
        self.time = time
        self.state = state
        self.travel = travel
        self.next_flow = next_flow
        self.flows = flows
        self.opened = opened
        self.current = current
        next_time = time - travel
        self.potential = current + next_time * (self.next_flow + sum(flows))

    def __repr__(self):
        v = ' '.join([f'{v:2d}' for v in self.flows])
        return f'{self.time:2d}|{self.opened:3d} [{self.current:4d}/{self.potential:4d}] ({names[self.me]},{names[self.elephant]})|{states[self.state]:4}|{v}|{self.travel}->{self.next_flow}'

    def __lt__(self, other):
        if self.potential == other.potential:
            return self.time < other.time
        else:
            return self.potential > other.potential


def build_distances(neighbors):
    global distances
    distances = [[1000] * len(neighbors) for m in range(len(neighbors))]
    for n in range(len(neighbors)):
        queue = [(0, n)]
        while queue:
            (d, m) = queue.pop()
            if d < distances[n][m]:
                distances[n][m] = d
                queue.extend([(d + 1, o) for o in neighbors[m]])


def ind(valves, v):
    if v in valves:
        return valves[v]
    else:
        i = len(valves)
        valves[v] = i
        return i


def parse(input):
    global names, valves
    valves = {}
    size = len(input.splitlines())
    neighbors = [[] for i in range(size)]
    flows = [0] * size
    names = [''] * size
    for valve in regex(input, 'Valve (\w+).*=(\d+).*valves? (.*)'):
        i = ind(valves, valve[0])
        valves[valve[0]] = i
        flows[i] = int(valve[1])
        names[i] = valve[0]
        for n in valve[2].split(', '):
            v = ind(valves, n)
            neighbors[i].append(v)
    return valves['AA'], neighbors, flows, names


def part1(input):
    global distances
    AA, neighbors, flows, names = parse(input)
    build_distances(neighbors)
    from heapq import heappop, heappush
    best = None
    queue = [Node(AA, 'AA', None, 30, flows, 0, distances)]
    while queue:
        node = heappop(queue)
        if not best or node.current > best.current:
            best = node

        for i, v in enumerate(node.flows):
            if v > 0:
                time = node.time - distances[node.index][i] - 1
                if time > 0:
                    flows = node.flows[:]
                    flows[i] = 0
                    current = node.current + time * v
                    heappush(queue, Node(
                        i, names[i], node, time, flows, current, distances))
    return best.current


def part2(input):
    global distances
    AA, neighbors, flows, names = parse(input)
    build_distances(neighbors)
    queue = [Elephant(AA, AA, None, 0, 26, BOTH_READY, 0, 0, flows, 0)]

    def push(l, p, m, e, node):
        flows = node.flows[:]
        flows[m[0]] = 0
        flows[e[0]] = 0
        if l == p:
            time = node.time - l
            if time >= 0:
                current = node.current + time * (m[1] + e[1])
                heappush(queue, Elephant(m[0], e[0], node,
                                         current, time, BOTH_READY, 0, 0, flows, m[1] + e[1]))
        else:
            if l < p:
                time = node.time - l
                if time >= 0:
                    heappush(queue, Elephant(
                        m[0], e[0], node, node.current + time * m[1], time, HES_TRAVELING, p - l, e[1], flows, m[1]))
            else:
                time = node.time - p
                if time >= 0:
                    heappush(queue, Elephant(
                        m[0], e[0], node, node.current + time * e[1], time, IM_TRAVELING, l - p, m[1], flows, e[1]))

    best = None
    while queue:
        node = heappop(queue)
        if node.current == node.potential:
            return node.current
        options = list(filter(lambda v: v[1], enumerate(node.flows)))

        if not options:
            if node.state in [DONE, BOTH_READY]:
                continue
            time = node.time - node.travel
            if time >= 0:
                current = node.current + time * node.next_flow
                heappush(queue, Elephant(0, 0, node, current, time,
                                         DONE, 0, 0, node.flows, node.next_flow))
            continue

        if len(options) == 1:
            i, flow = options[0]

            if node.state == BOTH_READY:
                flows = node.flows[:]
                flows[i] = 0
                l = distances[node.me][i] + 1
                p = distances[node.elephant][i] + 1
                if l < p:
                    time = node.time - l
                else:
                    time = node.time - p
                heappush(queue, Elephant(i, i, node, node.current,
                                         time, DONE, 0, 0, flows, flow))
            else:
                if node.state == IM_TRAVELING:
                    l = node.travel
                    p = distances[node.elephant][i] + 1
                    m = (node.me, node.next_flow)
                    e = options[0]
                else:
                    l = distances[node.me][i] + 1
                    p = node.travel
                    m = options[0]
                    e = (node.elephant, node.next_flow)
                push(l, p, m, e, node)
            continue

        if node.state == BOTH_READY:
            if node.me == node.elephant:
                for m, e in itertools.combinations(options, 2):
                    l = distances[node.me][m[0]] + 1
                    p = distances[node.elephant][e[0]] + 1
                    push(l, p, m, e, node)
            else:
                for m, e in itertools.permutations(options, 2):
                    l = distances[node.me][m[0]] + 1
                    p = distances[node.elephant][e[0]] + 1
                    push(l, p, m, e, node)
        else:
            for o in options:
                if node.state == IM_TRAVELING:
                    l = node.travel
                    p = distances[node.elephant][o[0]] + 1
                    m = (node.me, node.next_flow)
                    e = o
                else:
                    l = distances[node.me][o[0]] + 1
                    p = node.travel
                    m = o
                    e = (node.elephant, node.next_flow)
                push(l, p, m, e, node)
    visited = []
    x = best
    while x:
        visited.append(x)
        x = x.parent
    for v in reversed(visited):
        debug(v)
    return best.current


if __name__ == '__main__':
    main()
