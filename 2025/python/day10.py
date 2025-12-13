#!/usr/bin/env python3

from aoc import *
from heapq import heappop, heappush
from z3 import *

def machine_to_int(machine):
   value = 0
   bit = 1
   for b in machine[1:-1]:
      if b == '#':
         value += bit
      bit <<= 1
   return value


def button_to_int(button):
   value = 0
   for b in button[1:-1].split(','):
      value += (1 << int(b))
   return value


def part1(input: str):
    total = 0
    for machine in input.splitlines():
      parts = machine.split()
      target = machine_to_int(parts[0])
      buttons = [button_to_int(b) for b in parts[1:-1]]
      queue = [(0,0)]
      seen = {0}
      while queue:
         depth, state = heappop(queue)
         depth += 1
         for b in buttons:
            next = state ^ b
            if next == target:
               total += depth
               queue.clear()
               break
            if next in seen:
               continue
            seen.add(next)
            heappush(queue, (depth, next))
    return total


def part2(input: str):
    total = 0
    for machine in input.splitlines():
      parts = machine.split()
      joltages = [int(b) for b in parts[-1][1:-1].split(',')]
      buttons = [[int(b) for b in button[1:-1].split(',')] for button in parts[1:-1]]
      s = Solver()
      vars = [Int(f'a{b}') for b in range(len(buttons))]
      for v in vars:
         s.add(v >= 0)

      for i, j in enumerate(joltages):
         jvars = [vars[k] for k, b in enumerate(buttons) if i in b]
         s.add(Sum(jvars) == j)

      while s.check() == sat:
         m = s.model()
         n= sum([m[d].as_long() for d in m])
         s.add(Sum(vars) < n)

      total += n

    return total


if __name__ == '__main__':
    main()
