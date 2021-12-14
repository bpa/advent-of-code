class Point:
    """A point in 2D space"""

    def __init__(self, grid, x, y):
        self.grid = grid
        self.x = x
        self.y = y

    def get(self):
        return self.grid.data[self.y][self.x]

    def set(self, value):
        self.grid.data[self.y][self.x] = value

    def neighbor(self, x, y):
        nx = self.x + x
        ny = self.y + y
        if self.grid.is_valid(nx, ny):
            return Point(self.grid, nx, ny)
        else:
            return None

    def n(self): return self.neighbor(0, -1)
    def ne(self): return self.neighbor(1, -1)
    def e(self): return self.neighbor(1, 0)
    def se(self): return self.neighbor(1, 1)
    def s(self): return self.neighbor(0, 1)
    def sw(self): return self.neighbor(-1, 1)
    def w(self): return self.neighbor(-1, 0)
    def nw(self): return self.neighbor(-1, -1)

    def adjacent_4(self):
        """Get the n, e, s, w adjacent points"""
        from .func import nop1
        return filter(nop1, [self.n(), self.e(), self.s(), self.w()])

    def adjacent_8(self):
        """Get the n, e, s, w adjacent points"""
        from .func import nop1
        return filter(nop1, [self.n(), self.ne(), self.e(), self.se(), self.s(), self.sw(), self.w(), self.nw()])

    def distance(self, to):
        return abs(self.x - to.x) + abs(self.y - to.y)

    def __iter__(self):
        return iter([self.x, self.y])

    def __repr__(self):
        return f'({self.x}, {self.y})'

    def __set__(self, instance, value):
        print(self, instance, value)

    def __lt__(self, o):
        try:
            o = o.get()
        except:
            pass

        return self.get() < o

    def __le__(self, o):
        try:
            o = o.get()
        except:
            pass

        return self.get() <= o

    def __eq__(self, o):
        try:
            o = o.get()
        except:
            pass

        return self.get() == o

    def __ne__(self, o):
        try:
            o = o.get()
        except:
            pass

        return self.get() != o

    def __gt__(self, o):
        try:
            o = o.get()
        except:
            pass

        return self.get() > o

    def __ge__(self, o):
        try:
            o = o.get()
        except:
            pass

        return self.get() >= o

    def __hash__(self):
        return self.x * 181 + self.y

    def __bool__(self):
        return bool(self.grid.data[self.y][self.x])

    def __add__(self, o):
        try:
            o = o.get()
        except:
            pass

        return self.get() + o

    def __sub__(self, o):
        try:
            o = o.get()
        except:
            pass

        return self.get() - o

    def __mul__(self, o):
        try:
            o = o.get()
        except:
            pass

        return self.get() * o

    def __truediv__(self, o):
        try:
            o = o.get()
        except:
            pass

        return self.get() / o

    def __floordiv__(self, o):
        try:
            o = o.get()
        except:
            pass

        return self.get() // o

    def __mod__(self, o):
        try:
            o = o.get()
        except:
            pass

        return self.get() % o

    def __divmod__(self, o):
        try:
            o = o.get()
        except:
            pass

        return divmod(self.get(), o)

    def __pow__(self, o):
        try:
            o = o.get()
        except:
            pass

        return pow(self.get(), o)

    def __lshift__(self, o):
        try:
            o = o.get()
        except:
            pass

        return self.get() << o

    def __rshift__(self, o):
        try:
            o = o.get()
        except:
            pass

        return self.get() >> o

    def __and__(self, o):
        try:
            o = o.get()
        except:
            pass

        return self.get() & o

    def __xor__(self, o):
        try:
            o = o.get()
        except:
            pass

        return self.get() ^ o

    def __or__(self, o):
        try:
            o = o.get()
        except:
            pass

        return self.get() | o

    def __iadd__(self, o):
        try:
            o = o.get()
        except:
            pass

        self.grid.data[self.y][self.x] += o
        return self

    def __isub__(self, o):
        try:
            o = o.get()
        except:
            pass

        self.grid.data[self.y][self.x] -= o
        return self

    def __imul__(self, o):
        try:
            o = o.get()
        except:
            pass

        self.grid.data[self.y][self.x] *= o
        return self

    def __itruediv__(self, o):
        try:
            o = o.get()
        except:
            pass

        self.grid.data[self.y][self.x] /= o
        return self

    def __ifloordiv__(self, o):
        try:
            o = o.get()
        except:
            pass

        self.grid.data[self.y][self.x] //= o
        return self

    def __imod__(self, o):
        try:
            o = o.get()
        except:
            pass

        self.grid.data[self.y][self.x] %= o
        return self

    def __ipow__(self, o):
        try:
            o = o.get()
        except:
            pass

        self.grid.data[self.y][self.x] += self.grid.data[self.y][self.x].pow(o)
        return self

    def __ilshift__(self, o):
        try:
            o = o.get()
        except:
            pass

        self.grid.data[self.y][self.x] <<= o
        return self

    def __irshift__(self, o):
        try:
            o = o.get()
        except:
            pass

        self.grid.data[self.y][self.x] >>= o
        return self

    def __iand__(self, o):
        try:
            o = o.get()
        except:
            pass

        self.grid.data[self.y][self.x] &= o
        return self

    def __ixor__(self, o):
        try:
            o = o.get()
        except:
            pass

        self.grid.data[self.y][self.x] += o
        return self

    def __ior__(self, o):
        try:
            o = o.get()
        except:
            pass

        self.grid.data[self.y][self.x] |= o
        return self

    def __neg__(self):
        -self.grid.data[self.y][self.x]

    def __pos__(self):
        +self.grid.data[self.y][self.x]

    def __abs__(self):
        abs(self.grid.data[self.y][self.x])

    def __invert__(self):
        ~self.grid.data[self.y][self.x]

    def __complex__(self):
        complex(self.grid.data[self.y][self.x])

    def __int__(self):
        int(self.grid.data[self.y][self.x])

    def __float__(self):
        float(self.grid.data[self.y][self.x])

    def __round__(self, digits):
        round(self.grid.data[self.y][self.x], digits)

    def __trunc__(self, digits):
        from math import trunc
        trunc(self.grid.data[self.y][self.x], digits)

    def __floor__(self, digits):
        from math import floor
        floor(self.grid.data[self.y][self.x], digits)

    def __ceil__(self, digits):
        from math import ceil
        ceil(self.grid.data[self.y][self.x], digits)
