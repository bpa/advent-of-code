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
        if nx < 0 or nx >= self.grid.width:
            return None

        ny = self.y + y
        if ny < 0 or ny >= self.grid.height:
            return None

        return Point(self.grid, nx, ny)

    def n(self):
        return self.neighbor(0, -1)

    def ne(self):
        return self.neighbor(1, -1)

    def e(self):
        return self.neighbor(1, 0)

    def se(self):
        return self.neighbor(1, 1)

    def s(self):
        return self.neighbor(0, 1)

    def sw(self):
        return self.neighbor(-1, 1)

    def w(self):
        return self.neighbor(-1, 0)

    def nw(self):
        return self.neighbor(-1, -1)

    def on_4(self, dir):
        match dir:
            case 0:
                return self.n()
            case 1:
                return self.e()
            case 2:
                return self.s()
            case 3:
                return self.w()
        return None

    def adjacent_4(self):
        """Get the n, e, s, w adjacent points"""
        from .func import nop1

        return filter(nop1, [self.n(), self.e(), self.s(), self.w()])

    def adjacent_8(self):
        """Get the n, e, s, w adjacent points"""
        from .func import nop1

        return filter(
            nop1,
            [
                self.n(),
                self.ne(),
                self.e(),
                self.se(),
                self.s(),
                self.sw(),
                self.w(),
                self.nw(),
            ],
        )

    def manhattan_distance(self, to):
        return abs(self.x - to.x) + abs(self.y - to.y)

    def __iter__(self):
        return iter([self.x, self.y])

    def __repr__(self):
        return f"({self.x}, {self.y})"

    def __set__(self, instance, value):
        print(self, instance, value)

    def __lt__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return self.get() < o

    def __le__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return self.get() <= o

    def __eq__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return self.get() == o

    def __ne__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return self.get() != o

    def __gt__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return self.get() > o

    def __ge__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return self.get() >= o

    def __hash__(self):
        return self.x * 199933 + self.y

    def __bool__(self):
        return bool(self.grid.data[self.y][self.x])

    def __add__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return self.get() + o

    __radd__ = __add__

    def __sub__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return self.get() - o

    def __rsub__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return o - self.get()

    def __mul__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return self.get() * o

    __rmul__ = __mul__

    def __truediv__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return self.get() / o

    def __rtruediv__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return o / self.get()

    def __floordiv__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return self.get() // o

    def __rfloordiv__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return o // self.get()

    def __mod__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return self.get() % o

    def __rmod__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return o % self.get()

    def __divmod__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return divmod(self.get(), o)

    def __rdivmod__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return divmod(o, self.get())

    def __pow__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return pow(self.get(), o)

    def __rpow__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return pow(o, self.get())

    def __lshift__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return self.get() << o

    def __rlshift__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return o << self.get()

    def __rshift__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return self.get() >> o

    def __rrshift__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return o >> self.get()

    def __and__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return self.get() & o

    __rand__ = __and__

    def __xor__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return self.get() ^ o

    __rxor__ = __xor__

    def __or__(self, o):
        if isinstance(o, Point):
            o = o.get()

        return self.get() | o

    __ror__ = __or__

    def __iadd__(self, o):
        if isinstance(o, Point):
            o = o.get()

        self.grid.data[self.y][self.x] += o
        return self

    def __isub__(self, o):
        if isinstance(o, Point):
            o = o.get()

        self.grid.data[self.y][self.x] -= o
        return self

    def __imul__(self, o):
        if isinstance(o, Point):
            o = o.get()

        self.grid.data[self.y][self.x] *= o
        return self

    def __itruediv__(self, o):
        if isinstance(o, Point):
            o = o.get()

        self.grid.data[self.y][self.x] /= o
        return self

    def __ifloordiv__(self, o):
        if isinstance(o, Point):
            o = o.get()

        self.grid.data[self.y][self.x] //= o
        return self

    def __imod__(self, o):
        if isinstance(o, Point):
            o = o.get()

        self.grid.data[self.y][self.x] %= o
        return self

    def __ipow__(self, o):
        if isinstance(o, Point):
            o = o.get()

        self.grid.data[self.y][self.x] += self.grid.data[self.y][self.x].pow(o)
        return self

    def __ilshift__(self, o):
        if isinstance(o, Point):
            o = o.get()

        self.grid.data[self.y][self.x] <<= o
        return self

    def __irshift__(self, o):
        if isinstance(o, Point):
            o = o.get()

        self.grid.data[self.y][self.x] >>= o
        return self

    def __iand__(self, o):
        if isinstance(o, Point):
            o = o.get()

        self.grid.data[self.y][self.x] &= o
        return self

    def __ixor__(self, o):
        if isinstance(o, Point):
            o = o.get()

        self.grid.data[self.y][self.x] += o
        return self

    def __ior__(self, o):
        if isinstance(o, Point):
            o = o.get()

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
