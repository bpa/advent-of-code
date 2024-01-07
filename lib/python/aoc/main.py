import sys
import time

SUFFIXES = ["ns", "μs", "ms"]


def scale(ns):
    for i in range(3):
        if ns < 1000:
            return ns, SUFFIXES[i]
        ns /= 1000
    if ns < 60:
        return ns, "s"
    return ns / 60, "m"


def print_for_humans(ns):
    t, suffix = scale(ns)
    if t < 10:
        return f"{t:.1f}{suffix}"
    else:
        return f"{int(t)}{suffix}"


def main(*args):
    with open(sys.argv[1]) as file:
        input = file.read()
    m = sys.modules["__main__"]
    for i, n in enumerate(["part1", "part2"]):
        if hasattr(m, n):
            start = time.perf_counter_ns()
            r = getattr(m, n)(input)
            end = time.perf_counter_ns()
            print(f"Part {i+1}: {r} ({print_for_humans(end-start)})")
    if hasattr(m, "solve"):
        f = getattr(m, "solve")
        if len(args) == 0:
            print("Missing args for solve")
        for i, a in enumerate(args):
            start = time.perf_counter_ns()
            r = f(input, a)
            end = time.perf_counter_ns()
            print(f"Part {i+1}: {r} ({print_for_humans(end-start)})")
    end = time.perf_counter_ns()
