def nop1(o):
    return o


def switch(*args, **kwargs):
    import re
    from collections import Mapping
    from .puzzle import lines

    rere = re.compile(r'^/(.*)/$')
    loop = []
    lookup = {}

    if args and isinstance(args[0], Mapping):
        for (k, f) in args[0].items():
            try:
                match = rere.match(k)
                regex = match.groups(1)
                c = re.compile(regex[0])
                loop.append((c, f))
            except:
                lookup[k] = f

    for (k, f) in kwargs.items():
        lookup[k] = f

    def run(input=None):
        if input == None:
            input = lines()

        for row in input:
            if isinstance(row, str):
                key = row
                args = []
            else:
                key = row[0]
                args = row[1:]

            f = lookup.get(key, None)
            if f:
                f(*args)
                continue

            for (k, f) in loop:
                match = k.match(key)
                if match:
                    groups = match.groups()
                    if groups:
                        f(*groups, *args)
                    else:
                        f(key, *args)
                    break

    return run


def windows(iterable, size):
    arr = list(iterable)
    for i in range(len(arr) - size + 1):
        yield arr[i:i+size]


def chunks(iterable, size, pad=None):
    arr = list(iterable)
    i = 0
    complete = int(len(arr) / size)
    partial = len(arr) % size
    if partial and not pad:
        complete += 1
        partial = 0

    for _ in range(complete):
        yield arr[i:i+size]
        i += size

    if partial:
        yield arr[i:] + [pad] * partial


def manhattan_distance(x0, y0, x1, y1):
    return abs(x0 - x1) + abs(y0 - y1)
