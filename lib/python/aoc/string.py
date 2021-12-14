PAIRS = {}
if True:
    from .util import chunks
    for (a, b) in chunks('(){}[]<>', 2):
        PAIRS[a] = b
        PAIRS[b] = a


def r(regex):
    import re
    return re.compile(regex)


def delimited(delimiter=None, map=None):
    pass
