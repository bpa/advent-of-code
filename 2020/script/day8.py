from collections import defaultdict
import re
# re.findall(exp, text)
# re.search() -> anywhere
# re.match -> beginning
# re.split(exp, text)
# re.compile(exp)

input = "../input/2020/day8.txt"
answer = 0
instructions = []
with open(input) as file:
    for line in file.readlines():
        op, val = line.split(' ')
        instructions.append([op, int(val)])


def runit():
    curr = 0
    seen = set()
    acc = 0
    while True:
        if curr in seen:
            return (False, acc)
        if curr == len(instructions):
            return (True, acc)
        seen.add(curr)
        op, val = instructions[curr]
        if op == 'acc':
            acc += val
            curr += 1
        elif op == 'jmp':
            curr += val
        else:
            curr += 1


(finished, acc) = runit()
print("Part 1:", acc)

for i in range(0, len(instructions)):
    (op, val) = instructions[i]
    if op == 'nop':
        instructions[i][0] = 'jmp'
        (finished, acc) = runit()
        if finished:
            answer = acc
            break
        else:
            instructions[i][0] = 'nop'
    elif op == 'jmp':
        instructions[i][0] = 'nop'
        (finished, acc) = runit()
        if finished:
            answer = acc
            break
        else:
            instructions[i][0] = 'jmp'

print("Part 2:", answer)
