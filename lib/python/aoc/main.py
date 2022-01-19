import sys


def main(*args):
    with open(sys.argv[1]) as file:
        input = file.read()
    m = sys.modules['__main__']
    for (i, n) in enumerate(['part1', 'part2']):
        if hasattr(m, n):
            r = getattr(m, n)(input)
            print(f'Part {i+1}: {r}')
    if hasattr(m, 'solve'):
        f = getattr(m, 'solve')
        if len(args) == 0:
            print("Missing args for solve")
        for (i, a) in enumerate(args):
            r = f(input, a)
            print(f'Part {i+1}: {r}')
