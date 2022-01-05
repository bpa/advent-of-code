from aoc.util import nop1


PAIRS = {}
if True:
    from .util import chunks
    for (a, b) in chunks('(){}[]<>', 2):
        PAIRS[a] = b
        PAIRS[b] = a


def r(regex):
    import re
    return re.compile(regex)


def delimit(line, delimiter, map=None):
    if map == None:
        return line.split(delimiter)
    else:
        return [map(i) for i in line.split(delimiter)]


def delimited(input, delimiter, map=None):
    if map == None:
        for line in input.splitlines():
            yield line.split(delimiter)
    else:
        for line in input.splitlines():
            yield [map(i) for i in line.split(delimiter)]


def grid(lines, separator=None, map=lambda a: a, format=None):
    from logging import debug
    from .grid import Grid
    import re
    if separator:
        if isinstance(separator, re.Pattern):
            split = separator.split
        else:
            def split(s): return s.split(separator)
    else:
        def split(a): return list(a)
    grid = []
    for line in lines.splitlines():
        line = line.strip()
        row = [map(i) for i in split(line)]
        grid.append(row)
    return Grid(grid, format)


def regex(iterable, r, map_each=None, map=None):
    import re
    compiled = re.compile(r)
    line_no = 1
    from logging import debug

    def match(compiled, line, line_no):
        result = compiled.search(line)
        if result:
            if map:
                return map(result.groups())
            if map_each:
                return [map(v) for v in result.groups()]
            return result.groups()
        else:
            raise Exception(
                f"puzzle.regex() failed to match\nregex: {r}\nline {line_no}: {line}")

    if isinstance(iterable, str):
        return match(compiled, iterable, line_no)
    for line in iterable:
        yield match(compiled, line, line_no)
        line_no += 1
