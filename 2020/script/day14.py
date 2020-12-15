#!/usr/bin/env python3

from collections import defaultdict

input = "../input/2020/day14.txt"


def write(registers, address, value, mask):
    def _write(pos, bit, register):
        while pos >= 0:
            if mask[pos] == 'X':
                _write(pos - 1, bit * 2, register | bit)
            pos -= 1
            bit *= 2
        registers[register] = value
    _write(len(mask)-1, 1, address)


with open(input) as file:
    registers = defaultdict(lambda: 0)
    part2_registers = defaultdict(lambda: 0)
    for line in file.readlines():
        line = line.strip()
        inst, val = line.split(' = ')
        if inst == 'mask':
            mask = val
            bitmask = 0
            bitmaskmask = 0
            for bit in val:
                bitmask *= 2
                bitmaskmask *= 2
                if bit == 'X':
                    bitmaskmask += 1
                elif bit == '1':
                    bitmask += 1
        elif inst.startswith('mem'):
            i = int(inst[4:-1])
            x = int(val) & bitmaskmask | bitmask
            registers[i] = x

            r2 = (i & ~bitmaskmask) | bitmask
            write(part2_registers, r2, int(val), mask)


print("Part 1:", sum(registers.values()))
print("Part 2:", sum(part2_registers.values()))
