#!/usr/bin/env python3

from collections import defaultdict

input = "../input/2020/day9.txt"
answer = 0
with open(input) as file:
    numbers = [int(i) for i in file.readlines()]

held = defaultdict(lambda: 0)
for i in numbers[:25]:
    held[i] += 1
next = 25

exists = True
while exists:
    answer = numbers[next]
    exists = False
    for i in range(next-25, next):
        wanted = answer - numbers[i]
        if held[wanted] > 0:
            exists = True
    held[numbers[next - 25]] -= 1
    held[numbers[next]] += 1
    next += 1

print("Part 1:", answer)

start = 0
end = 0
sum = numbers[0]
while True:
    if sum < answer:
        end += 1
        sum += numbers[end]
    elif sum > answer:
        sum -= numbers[start]
        start += 1
    else:
        break

print("Part 2:", min(numbers[start:end]) + max(numbers[start:end]))
