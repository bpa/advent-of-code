#!/usr/bin/env python3

from aoc import *
from heapq import heappop, heappush


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


def button_to_array(button, length):
   arr = [0] * length
   for b in button[1:-1].split(','):
      arr[int(b)] = 1
   return arr
   

def part2(input: str):
    total = 0
    for machine in input.splitlines():
      parts = machine.split()
      target = tuple([int(b) for b in parts[-1][1:-1].split(',')])
      buttons = [button_to_array(b, len(target)) for b in parts[1:-1]]
      queue = [(0, target)]
      seen = {target}
      while queue:
         depth, state = heappop(queue)
         depth += 1
         for b in buttons:
            next = []
            found = True
            too_many = True
            for i, s in enumerate(state):
               v = s - b[i]
               if v < 0:
                  break
               if v > 0:
                  found = False
               next.append(v)
            else:
               too_many = False
            
            if too_many:
               continue

            # next = tuple(next)
            # if next in seen:
            #    continue
            # seen.add(next)

            if found:
               debug("Found target:", next)
               total += depth
               queue.clear()
               break

            heappush(queue, (depth, next))
    return total


if __name__ == '__main__':
    main()
