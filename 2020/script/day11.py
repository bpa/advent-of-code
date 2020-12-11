#!/usr/bin/env python3

input = "../input/2020/day11.txt"
with open(input) as file:
    seats = [[c for c in line.strip()] for line in file.readlines()]

for s in seats:
    s.insert(0, '.')
    s.append('.')

empty_row = [f for f in '.' * len(seats[0])]
seats.insert(0, empty_row)
seats.append(empty_row)

backup = seats

changed = True
while changed:
    changed = False
    next = [empty_row]
    for y in range(1, len(seats)-1):
        row = ['.']
        for x in range(1, len(seats[0])-1):
            count = 0
            if seats[y][x] == 'L':
                for yy in range(y-1, y+2):
                    for xx in range(x-1, x+2):
                        if seats[yy][xx] == '#':
                            count += 1
                if count == 0:
                    changed = True
                    row.append('#')
                else:
                    row.append('L')
            elif seats[y][x] == '#':
                for yy in range(y-1, y+2):
                    for xx in range(x-1, x+2):
                        if seats[yy][xx] == '#':
                            count += 1
                if count > 4:
                    changed = True
                    row.append('L')
                else:
                    row.append('#')
            else:
                row.append(seats[y][x])
        row.append('.')
        next.append(row)
    next.append(empty_row)
    seats = next

print("Part 1:", sum(
    [sum([1 if s == '#' else 0 for s in row]) for row in seats]))


def visible(x, y):
    seen = 0
    for dx, dy in ([0, 1], [0, -1], [1, 0], [1, -1], [1, 1], [-1, 0], [-1, 1], [-1, -1]):
        xx = x + dx
        yy = y + dy
        while xx > 0 and yy > 0:
            try:
                if seats[yy][xx] == 'L':
                    break
                if seats[yy][xx] == '#':
                    seen += 1
                    break
            except:
                break
            xx += dx
            yy += dy
    return seen


seats = backup
changed = True
while changed:
    changed = False

    next = [empty_row]
    for y in range(1, len(seats)-1):
        row = ['.']
        for x in range(1, len(seats[0])-1):
            if seats[y][x] == 'L':
                if visible(x, y) == 0:
                    changed = True
                    row.append('#')
                else:
                    row.append('L')
            elif seats[y][x] == '#':
                if visible(x, y) > 4:
                    changed = True
                    row.append('L')
                else:
                    row.append('#')
            else:
                row.append(seats[y][x])
        row.append('.')
        next.append(row)
    next.append(empty_row)
    seats = next

print("Part 2:", sum(
    [sum([1 if s == '#' else 0 for s in row]) for row in seats]))
