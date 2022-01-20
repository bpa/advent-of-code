#!/usr/bin/env python3

from collections import defaultdict
import math
import re
# re.findall(exp, text)
# re.search() -> anywhere
# re.match -> beginning
# re.split(exp, text)
# re.compile(exp)

input = "../input/2020/day21.txt"
# input = "test.txt"
allergens = {}
all_ingredients = set()
with open(input) as file:
    for line in file.readlines():
        if line.isspace():
            break
        ingredients, allergen_list = line.split(' (contains ')
        ingredients = set(ingredients.split(' '))
        all_ingredients.update(ingredients)
        for allergen in allergen_list.strip()[:-1].split(', '):
            if allergen in allergens:
                allergens[allergen].intersection_update(ingredients)
            else:
                allergens[allergen] = ingredients.copy()


for possible in allergens.values():
    all_ingredients.difference_update(possible)

answer = 0
with open(input) as file:
    for line in file.readlines():
        if line.isspace():
            break
        ingredients = set(line.split(' (contains ')[0].split(' '))
        answer += len(ingredients.intersection(all_ingredients))

print("Part 1:", answer)

mapping = {}
while len(allergens) > 0:
    seen = defaultdict(set)
    remove = set()
    for k, v in allergens.items():
        if len(v) == 1:
            mapping[k] = next(iter(v))
            remove.add(k)
        for allergen in v:
            seen[allergen] = k
    for k, v in seen.items():
        if len(v) == 1:
            allergen = next(iter(v))
            mapping[allergen] = k
            remove.add(allergen)
    for allergen in remove:
        del allergens[allergen]
        for v in allergens.values():
            v.discard(mapping[allergen])

print("Part 2:", ','.join([mapping[allergen]
                           for allergen in sorted(mapping.keys())]))
