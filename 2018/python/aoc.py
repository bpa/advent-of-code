from functools import wraps
import time


def aoc(day, part):
    def timeit(f):
        @wraps(f)
        def timed(*args, **kwargs):
            start = time.time_ns()
            answer = f(*args, **kwargs)
            end = time.time_ns()
            elapsed = end - start
            print(f"Part {part}: {answer}")
            if elapsed > 1_000_000_000:
                print(f"Elapsed: {elapsed/1_000_000_000:.2f}s")
            elif elapsed > 1_000_000:
                print(f"Elapsed: {elapsed/1_000_000:.2f}ms")
            elif elapsed > 1_000:
                print(f"Elapsed: {elapsed/1_000:.2f}µs")
            else:
                print(f"Elapsed: {elapsed}ns")
            print()
        return timed
    return timeit
