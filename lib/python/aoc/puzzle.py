def lines(map=None):
    import sys
    if map == None:
        with open(sys.argv[1]) as file:
            for line in file.read().splitlines():
                yield line
    else:
        with open(sys.argv[1]) as file:
            for line in file.read().splitlines():
                yield map(line)


def delimited(delimiter, map=None):
    import sys
    if map == None:
        with open(sys.argv[1]) as file:
            for line in file.read().splitlines():
                yield line.split(delimiter)
    else:
        with open(sys.argv[1]) as file:
            for line in file.read().splitlines():
                yield [map(i) for i in line.split(delimiter)]


def regex(r, map=None):
    import re
    import sys
    compiled = re.compile(r)
    with open(sys.argv[1]) as file:
        line_no = 0
        for line in file.readlines():
            line_no += 1
            result = compiled.search(line)
            if result:
                if map == None:
                    yield result.groups()
                else:
                    yield map(result.groups())
            else:
                raise Exception(
                    f"puzzle.regex() failed to match\nregex: {r}\nline {line_no}: {line}")


def grid(separator=None, map=lambda a: a):
    import sys
    from .grid import Grid
    if separator:
        def split(s): return s.split(separator)
    else:
        def split(a): return a
    grid = []
    with open(sys.argv[1]) as file:
        for line in file.readlines():
            line = line.rstrip()
            row = [map(i) for i in split(line)]
            grid.append(row)
    return Grid(grid)


def pandas(separator=None, map=lambda a: a):
    import sys
    import pandas
    return pandas.read_csv(sys.argv[1], sep=separator)


def pandas_line(separator=None, map=lambda a: a):
    import sys
    import pandas
    with open(sys.argv[1]) as file:
        data = [map(p) for p in file.read().split(separator)]
    return pandas.DataFrame({'input': data})['input']
