from collections import defaultdict

input = "../input/2020/day7.txt"

rules = defaultdict(set)
contain = defaultdict(dict)
with open(input) as file:
    for line in file.readlines():
        bag, requirements = line.split(" contain ")
        bag = bag.rsplit(" ", 1)[0]
        for r in requirements.split(", "):
            tokens = r.split(" ")
            count = tokens[0]
            if count != 'no':
                inner_bag = f"{tokens[1]} {tokens[2]}"
                rules[inner_bag].add(bag)
                contain[bag][inner_bag] = int(count)

seen = set()
queue = list(rules['shiny gold'])
while len(queue) > 0:
    curr = queue.pop()
    seen.add(curr)
    for candidate in rules[curr]:
        if not candidate in seen:
            queue.append(candidate)
print(f"Part 1: {len(seen)}")

total_bags = 0
queue = defaultdict(lambda: 0)
queue['shiny gold'] = 1
while len(queue) > 0:
    (color, multiplier) = queue.popitem()
    for bag, count in contain[color].items():
        total_bags += count * multiplier
        queue[bag] += count * multiplier
print("Part 2:", total_bags)
