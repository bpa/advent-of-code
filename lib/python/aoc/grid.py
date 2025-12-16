from .point import Point


class Grid:
    """A 2D array of elements"""

    def __init__(self, data, format=None, true=None, false=None):
        self.data = data
        self.format = format
        self.true = true
        self.false = false
        self.height = len(data)
        self.width = len(data[0])

    def clone(self):
        copy = [r.copy() for r in self.data]
        return Grid(copy, self.format, self.true, self.false)

    @classmethod
    def of(cls, w, h, default_value=None):
        return cls([[default_value for _ in range(w)] for _ in range(h)])

    def is_valid(self, x, y):
        """Are the x, y coordinates on the grid?"""
        return x >= 0 and x < self.width and y >= 0 and y < self.height

    def at(self, x, y):
        """Get the Point at x, y"""
        return Point(self, x, y)

    def get(self, x, y):
        """Get the element at the point, or return None"""
        return self.data[y][x]

    def set(self, x, y, value):
        """Set the element at the point, or return None"""
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
        for y, row in enumerate(self.data):
            for x, value in enumerate(row):
                if value in keys:
                    locations[value] = Point(self, x, y)
        return locations

    def distances(self, wall, *items, neighbors=4) -> dict[str, dict[str, int]]:
        # TODO: optimize with floodfill followed by lookups
        points = self.index(*items)
        dist = {key: {} for key in points.keys()}
        for i, start in enumerate(items):
            if start not in points:
                continue
            for j in range(i + 1, len(items)):
                end = items[j]
                if end in points:
                    d = len(
                        self.shortest_path(
                            points[start], points[end], wall=wall, neighbors=neighbors
                        )
                    )
                    dist[start][end] = d
                    dist[end][start] = d
        return dist

    def shortest_path(
        self, a, b, wall="", neighbors=4, vertex_cost=lambda a, b: 1, distance=None
    ):
        if distance == None:
            from .util import manhattan_distance as distance
        if neighbors == 4:
            adj = [(0, -1), (1, 0), (0, 1), (-1, 0)]
        elif neighbors == 8:
            adj = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)]
        else:
            raise Exception("Invalid number of neighbors")

        import sys

        (G, CLOSED, PARENT_X, PARENT_Y) = range(4)
        nodes = []
        for y in range(self.width):
            row = []
            for x in range(self.height):
                row.append([sys.maxsize, False, None, None])
            nodes.append(row)

        start = nodes[a.y][a.x]
        start[G] = 0

        from heapq import heappop, heappush

        queue = [(0, a.x, a.y)]
        while queue:
            (_, x, y) = heappop(queue)
            node = nodes[y][x]
            node[CLOSED] = True
            if x == b.x and y == b.y:
                break

            for dx, dy in adj:
                nx = x + dx
                if nx < 0 or nx >= self.width:
                    continue

                ny = y + dy
                if ny < 0 or ny >= self.height:
                    continue

                if self.data[ny][nx] in wall:
                    continue

                n2 = nodes[ny][nx]
                if n2[CLOSED]:
                    continue

                g = node[G] + vertex_cost(self.data[y][x], self.data[ny][nx])
                if g >= n2[G]:
                    continue

                n2[G] = g
                n2[PARENT_X] = x
                n2[PARENT_Y] = y
                h = distance(nx, ny, b.x, b.y)
                # TODO: Figure out how to remove the existing node from the open list
                # if not n2[OPEN]:
                heappush(queue, (g + h, nx, ny))
                # n2[OPEN] = True

        path = []
        x = b.x
        y = b.y
        point = nodes[y][x]
        while point[PARENT_X] != None:
            path.append(Point(self, x, y))
            x = point[PARENT_X]
            y = point[PARENT_Y]
            point = nodes[y][x]
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

    def count(self, f=lambda a: a, x1=None, y1=None, x0=None, y0=None):
        (x1, y1, x0, y0) = self._bounds(x1, y1, x0, y0)
        cnt = 0
        for y in range(y0, y1):
            for x in range(x0, x1):
                if f(self.data[y][x]):
                    cnt += 1
        return cnt

    def sum(self, x1=None, y1=None, x0=None, y0=None):
        (x1, y1, x0, y0) = self._bounds(x1, y1, x0, y0)
        total = 0
        for y in range(y0, y1):
            for x in range(x0, x1):
                total += self.data[y][x]
        return total

    def to_string(self, x1=None, y1=None, x0=None, y0=None):
        (x1, y1, x0, y0) = self._bounds(x1, y1, x0, y0)

        true = None
        false = None
        if isinstance(self.data[0][0], bool):
            if self.true == None:
                true = "#"
            if self.false == None:
                false = "."

        if self.format == None:
            to_string = str
        else:
            to_string = self.format

        from io import StringIO

        repr = StringIO()
        repr.write("\n")
        for y in range(y0, y1):
            for x in range(x0, x1):
                value = self.data[y][x]
                if value:
                    if true:
                        repr.write(true)
                    else:
                        repr.write(to_string(value))
                else:
                    if false:
                        repr.write(false)
                    else:
                        repr.write(to_string(value))
            repr.write("\n")
        return repr.getvalue()

    def iter_reverse(self):
        for y in range(self.height - 1, -1, -1):
            for x in range(self.width - 1, -1, -1):
                yield Point(self, x, y)

    def fill(self, x, y, value=1):
        queue = [(x, y)]
        while queue:
            x, y = queue.pop(0)
            if self.is_valid(x, y) and self.data[y][x] == 0:
                self.data[y][x] = value
                queue.append((x+1, y))
                queue.append((x-1, y))
                queue.append((x, y+1))
                queue.append((x, y-1))

    def sum_table(self):
        table = Grid.of(self.width, self.height, 0)
        x_sum = self.data[0][0]
        table.data[0][0] = x_sum
        for x in range(1, table.width):
            x_sum += self.data[0][x]
            table.data[0][x] = x_sum
        y_sum = self.data[0][0]
        for y in range(1, table.height):
            y_sum += self.data[y][0]
            table.data[y][0] = y_sum
        for y in range(1, table.height):
            for x in range(1, table.width):
                table.data[y][x] = (
                    self.data[y][x]
                    + table.data[y - 1][x]
                    + table.data[y][x - 1]
                    - table.data[y - 1][x - 1]
                )
        return table

    def __str__(self):
        return self.to_string()

    def __repr__(self):
        return "\n" + "\n".join(map(lambda row: str(row), self.data))

    def __iter__(self):
        for y in range(self.height):
            for x in range(self.width):
                yield Point(self, x, y)
