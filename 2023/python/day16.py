#!/usr/bin/env python3

from aoc import *


def energized_tiles(cave, start, d):
    energized = Grid.of(cave.width, cave.height, 0)
    queue = [(start, d)]
    seen = set()
    while queue:
        x = queue.pop()
        if x in seen:
            continue
        else:
            seen.add(x)
        (tile, d) = x
        if not tile:
            continue

        energized.set(tile.x, tile.y, 1)
        match tile.get():
            case ".":
                queue.append((tile.on_4(d), d))
            case "\\":
                match d:
                    case 0:
                        d = 3
                    case 1:
                        d = 2
                    case 2:
                        d = 1
                    case 3:
                        d = 0
                queue.append((tile.on_4(d), d))
            case "/":
                match d:
                    case 0:
                        d = 1
                    case 1:
                        d = 0
                    case 2:
                        d = 3
                    case 3:
                        d = 2
                queue.append((tile.on_4(d), d))
            case "-":
                if d % 2 == 0:
                    queue.append((tile.e(), 1))
                    queue.append((tile.w(), 3))
                else:
                    queue.append((tile.on_4(d), d))
            case "|":
                if d % 2 == 0:
                    queue.append((tile.on_4(d), d))
                else:
                    queue.append((tile.n(), 0))
                    queue.append((tile.s(), 2))
    return energized.sum()


def part1(input: str):
    cave = grid(input)
    return energized_tiles(cave, cave.at(0, 0), 1)


def part2(input: str):
    cave = grid(input)
    max_energy = 0
    for y in range(cave.height):
        max_energy = max(max_energy, energized_tiles(cave, cave.at(0, y), 1))
        max_energy = max(
            max_energy, energized_tiles(cave, cave.at(cave.width - 1, y), 3)
        )
    for x in range(cave.width):
        max_energy = max(max_energy, energized_tiles(cave, cave.at(x, 0), 2))
        max_energy = max(
            max_energy, energized_tiles(cave, cave.at(x, cave.height - 1), 0)
        )
    return max_energy


if __name__ == "__main__":
    main()
