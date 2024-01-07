#!/usr/bin/env python3

from aoc import *


class Module:
    def __init__(self, name, targets) -> None:
        self.sources = {}
        self.name = name
        self.targets = targets

    def add_input(self, input):
        self.sources[input] = False

    def __call__(self, p, src):
        return None

    def __repr__(self):
        return self.name


class Broadcast(Module):
    def __call__(self, p, src):
        return p


class FlipFlop(Module):
    def __init__(self, name, targets):
        self.on = False
        Module.__init__(self, name, targets)

    def __call__(self, p, src):
        if p:
            return None
        self.on = not self.on
        return self.on


class Conjunction(Module):
    def __call__(self, p, src):
        self.sources[src] = p
        if all(self.sources.values()):
            return False
        else:
            return True


def parse(input):
    modules = {}
    for module, targets in delimited(input, " -> "):
        targets = targets.split(", ")
        match module[0]:
            case "%":
                op = FlipFlop(module, targets)
                module = module[1:]
            case "&":
                op = Conjunction(module, targets)
                module = module[1:]
            case _:
                op = Broadcast(module, targets)
        existing = modules.get(module)
        modules[module] = op
        if existing:
            op.sources = existing.sources

        for t in targets:
            m = modules.get(t)
            if not m:
                m = Module("XX", [])
                modules[t] = m
            m.add_input(module)
    return modules


def press_button(signal, modules):
    high = 0
    low = 0
    queue = [signal]
    while queue:
        (src, pulse, dest) = queue.pop(0)
        if pulse:
            high += 1
        else:
            low += 1

        module = modules[dest]
        v = module(pulse, src)
        if v is not None:
            for t in module.targets:
                queue.append((dest, v, t))
    return low, high


def part1(input: str) -> dict[str, Module]:
    modules = parse(input)
    low = 0
    high = 0
    for i in range(22):
        (l, h) = press_button(("button", False, "broadcaster"), modules)
        low += l
        high += h
    return low * high


def visualize(modules):
    if os.getenv("DEBUG"):
        return

    channels = []
    reset = []
    for m in modules["broadcaster"].targets:
        node = modules[m]
        bits = [node]
        targets = [modules[t] for t in node.targets]
        reset.append(next(filter(lambda t: isinstance(t, Conjunction), targets)))
        while True:
            for n in node.targets:
                t = modules[n]
                if isinstance(t, FlipFlop):
                    bits.append(t)
                    node = t
                    break
            else:
                break
        channels.append(bits)

    print("               broadcaster")
    print("     ┌─────────┬────┴────┬─────────┐")
    # print("   ↓         ↓         ↓        ↓")
    for col in range(len(channels)):
        print(f" ┌─ {channels[col][0]} ←┐", end="")
    print()
    display = max([len(c) for c in channels])
    for row in range(1, display):
        for col in range(4):
            m = channels[col][row]
            x = reset[col].name[1:]
            l = "├─" if x in m.targets else "│ "
            r = "←┤" if x in m.sources else " │"
            print(f" {l} {m} {r}", end="")
        print()
    for r in reset:
        print(f" └→ {r} ─┘", end="")
    print(f"\n     └─────────┴────┬────┴─────────┘")
    collector = modules[next(iter(modules["rx"].sources.keys()))]
    print(f"                   {collector}")
    print(f"                    ↓")
    print(f"                   &rx")


def part2(input: str):
    modules = parse(input)
    visualize(modules)
    base = []
    for m in modules["broadcaster"].targets:
        bits = {m: 1}
        value = 1
        base_num = 0
        node = modules[m]
        while not isinstance(node, Conjunction):
            if len(node.targets) == 2:
                base_num += value
            for t in node.targets:
                instance = modules[t]
                if isinstance(instance, FlipFlop):
                    bits[t] = value
                    node = instance
                    break
            else:
                base_num += value
                node = instance
            value <<= 1

        base.append(base_num)
    return math.lcm(*base)


if __name__ == "__main__":
    main()
