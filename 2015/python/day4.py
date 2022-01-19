#!/usr/bin/env python3

from aoc import *
import hashlib


def solve(input, prefix):
    input = input.rstrip()
    l = 1
    while True:
        for s in range(1, 10):
            for t in range(10**l):
                ans = '{0}{1:0{2}d}'.format(s, t, l)
                md5 = hashlib.md5(f'{input}{ans}'.encode('utf-8')).hexdigest()
                if md5.startswith(prefix):
                    return ans
        l += 1


if __name__ == '__main__':
    main('00000', '000000')
