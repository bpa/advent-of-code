#!/usr/bin/env python3

from dataclasses import dataclass
from aoc import *

@dataclass
class Node:
    name: str
    inputs: list[str]
    outputs: list[str]
    paths: int
    depth: int

def parse_input(input: str) -> dict[str, Node]:
    nodes = {}
    for line in input.splitlines():
        parts = line.split()
        name = parts[0][:-1]
        node = nodes.get(name)
        if not node:
            node = Node(name, [], [], 0, 0)
            nodes[name] = node
        node.outputs = parts[1:]
        for output in node.outputs:
            out = nodes.get(output)
            if not out:
                out = Node(output, [], [], 0, 0)
                nodes[output] = out
            out.inputs.append(node.name)
    return nodes

def find_paths(start: str, end: str, nodes: dict[str, Node]) -> int:
    for n in nodes.values():
        n.paths = 0
    in_progress = set(nodes[start].outputs)
    queue = list(in_progress)
    nodes[start].paths = 1
    while queue:
        name = queue.pop(0)
        in_progress.remove(name)
        node = nodes[name]
        paths = sum([nodes[inp].paths for inp in node.inputs])
        if paths != node.paths:
            node.paths = paths
            for inp in node.outputs:
                if inp not in in_progress:
                    in_progress.add(inp)
                    queue.append(inp)
    return nodes[end].paths

def part1(input: str):
    return 0
    nodes = parse_input(input)
    return find_paths('you', 'out', nodes)


def part2(input: str):
    nodes = parse_input(input)
    dac = find_paths('svr', 'dac', nodes)
    fft = nodes['fft'].paths
    print(f"Paths to DAC: {dac}, Paths to FFT: {fft}")
    if dac < fft:
        middle = find_paths('dac', 'fft', nodes)
        print(f"Paths from DAC to FFT: {middle}")
        return dac * middle * find_paths('fft', 'out', nodes)
    else:
        middle = find_paths('fft', 'dac', nodes)
        print(f"Paths from FFT to DAC: {middle}")
        print(find_paths('dac', 'out', nodes))
        return dac * middle * find_paths('dac', 'out', nodes)

# 207000528761440412149500
if __name__ == '__main__':
    main()
