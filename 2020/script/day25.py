#!/usr/bin/env python3

from collections import defaultdict
import math
import re
# re.findall(exp, text)
# re.search() -> anywhere
# re.match -> beginning
# re.split(exp, text)
# re.compile(exp)

input = "../input/2020/day25.txt"
magic_number = 20201227
subject_number = 7

with open(input) as file:
    public_key_a = int(file.readline())
    public_key_b = int(file.readline())
    # public_key_a = 5764801
    # public_key_b = 17807724


def loop_size(public_key):
    size = 0
    value = 1
    while value != public_key:
        value = (subject_number * value) % magic_number
        size += 1
    return size


def encryption_key(loop_size, subject_number):
    value = 1
    for _ in range(loop_size):
        value = (subject_number * value) % magic_number
    return value


loop_size_a = loop_size(public_key_a)
loop_size_b = loop_size(public_key_b)

print("Part 1:", encryption_key(loop_size_a, public_key_b))
print("Part 2:", loop_size(public_key_b))
