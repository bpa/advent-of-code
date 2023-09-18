class WordMap:
    def __init__(self):
        self.root = {}

    def add(self, word, value):
        node = self.root
        for c in word:
            next = node.get(c)
            if not next:
                next = {}
                node[c] = next
            node = next
        node["_value"] = value
