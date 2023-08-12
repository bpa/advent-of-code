class Distance:
    def __init__(self):
        from collections import defaultdict

        self.lookup = defaultdict(dict)
        self.nodes = set()
        self.names = []

    def one_way(self, a, b, dist):
        self.lookup[a][b] = dist
        self.nodes.add(a)
        self.nodes.add(b)

    def set_dist(self, a, b, dist):
        self.lookup[a][b] = dist
        self.lookup[b][a] = dist
        self.nodes.add(a)
        self.nodes.add(b)

    def as_table(self, default=None):
        table = [[None] * len(self.nodes) for i in range(len(self.nodes))]
        self.names = list(self.nodes)
        for i, a in enumerate(self.names):
            row = table[i]
            for j, b in enumerate(self.names):
                if i == j:
                    row[j] = 0
                else:
                    row[j] = self.lookup[a].get(b, default)
        return table
