from os import sep


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
    from .string import delimited
    with open(sys.argv[1]) as file:
        return delimited(file.read().splitlines())


def blocks():
    import sys
    with open(sys.argv[1]) as file:
        return iter(file.read().split("\n\n"))


def regex(r, map=None):
    import sys
    from .string import regex
    with open(sys.argv[1]) as file:
        return regex(file.readlines())


def grid(separator=None, map=lambda a: a, format=None):
    import sys
    from .string import grid
    with open(sys.argv[1]) as file:
        return grid(file.read(), separator=separator, map=map, format=None)


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
