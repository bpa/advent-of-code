from .point import Point


class Grid:
    """A 2D array of elements"""

    def __init__(self, data):
        self.data = data
        self.height = len(data)
        self.width = len(data[0])

    def is_valid(self, x, y):
        """Are the x, y coordinates on the grid?"""
        return x >= 0 and x < self.width and y >= 0 and y < self.height

    def get(self, x, y=None):
        """Get the element at the point, or return None"""
        if isinstance(x, Point):
            if self.is_valid(x.x, x.y):
                return self.data[x.y][x.x]
        elif self.is_valid(x, y):
            return self.data[y][x]
        else:
            return None

    def set(self, x, y, value=None):
        """Get the element at the point, or return None"""
        if isinstance(x, Point):
            if self.is_valid(x.x, x.y):
                self.data[x.y][x.x] = y
        elif self.is_valid(x, y):
            self.data[y][x] = value

    def adjacent_4(self, x, y):
        """Get the n, e, s, w adjacent points"""
        return Point(self, x, y).adjacent_4()

    def adjacent_8(self, x, y):
        """Get the n, e, s, w adjacent points"""
        return Point(self, x, y).adjacent_8()

    def as_pandas(self):
        import pandas
        return pandas.DataFrame(self.data)

    def index(self, *items):
        """Find all points given, return a map of item: point"""
        keys = set(items)
        locations = {}
        for (y, row) in enumerate(self.data):
            for (x, value) in enumerate(row):
                if value in keys:
                    locations[value] = Point(self, x, y)
        return locations

    def distances(self, open, *items, neighbors=4):
        # TODO: optimize with floodfill followed by lookups
        points = self.index(*items)
        dist = {key: {} for key in points.keys()}
        for (i, start) in enumerate(items):
            if start not in points:
                continue
            for j in range(i+1, len(items)):
                end = items[j]
                if end in points:
                    d = len(self.shortest_path(
                        points[start], points[end], open=open, neighbors=neighbors))
                    dist[start][end] = d
                    dist[end][start] = d
        return dist

    def shortest_path(self, a, b, open=' ', neighbors=4):
        if neighbors == 4:
            adj = Point.adjacent_4
        elif neighbors == 8:
            adj = Point.adjacent_8
        else:
            raise Exception("Invalid number of neighbors")

        paths = {}
        import sys
        g_score = [[sys.maxsize] * self.width for i in range(self.height)]
        g_score[a.y][a.x] = 0

        from heapq import heapify, heappop, heappush
        queue = [(0, a)]
        point = a
        while queue:
            (distance, point) = heappop(queue)
            if point == b:
                break
            for n in adj(point):
                g = g_score[n.y][n.x]
                if n.get() in open and g > (distance + 1):
                    paths[n] = point
                    g_score[n.y][n.x] = distance + 1
                    heappush(queue, (n.distance(point) + distance + 1, n))

        path = []
        while point in paths:
            path.append(point)
            point = paths[point]
        return path

    def _bounds(self, x1=None, y1=None, x0=None, y0=None):
        if x0 == None:
            x0 = 0
        if y0 == None:
            y0 = 0
        if x1 == None:
            x1 = self.width
        if y1 == None:
            y1 = self.height
        return (x1, y1, x0, y0)

    def range(self, x1=None, y1=None, x0=None, y0=None):
        (x1, y1, x0, y0) = self._bounds(x1, y1, x0, y0)

        for y in range(y0, y1):
            for x in range(x0, x1):
                yield (x, y, self.data[y][x])

    def enumerate(self, x1=None, y1=None, x0=None, y0=None):
        (x1, y1, x0, y0) = self._bounds(x1, y1, x0, y0)

        for y in range(y0, y1):
            for x in range(x0, x1):
                yield (x, y, Point(self, x, y))

    def count(self, f, x1=None, y1=None, x0=None, y0=None):
        (x1, y1, x0, y0) = self._bounds(x1, y1, x0, y0)
        cnt = 0
        for y in range(y0, y1):
            for x in range(x0, x1):
                if f(self.data[y][x]):
                    cnt += 1
        return cnt

    def to_string(self, x1=None, y1=None, x0=None, y0=None, true=None, false=None):
        (x1, y1, x0, y0) = self._bounds(x1, y1, x0, y0)

        if isinstance(self.data[0][0], bool):
            if true == None:
                true = '#'
            if false == None:
                false = '.'

        from io import StringIO
        repr = StringIO()
        for y in range(y0, y1):
            for x in range(x0, x1):
                value = self.data[y][x]
                if value:
                    if true:
                        repr.write(true)
                    else:
                        repr.write(str(value))
                else:
                    if false:
                        repr.write(false)
                    else:
                        repr.write(str(value))
            repr.write("\n")
        return repr.getvalue()

    def __repr__(self):
        return "\n" + "\n".join(map(lambda row: ''.join(map(str, row)), self.data))

    def __iter__(self):
        for y in range(self.height):
            for x in range(self.width):
                yield Point(self, x, y)
