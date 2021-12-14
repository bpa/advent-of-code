import unittest
from aoc import Grid, Point


def cypher_grid():
    return Grid(list(map(lambda a: a.strip(), filter(lambda a: a, """
        abcde
        fghik
        lmnop
        qrstu
        vwxyz
        """.splitlines()))))


def maze():
    return Grid(list(map(lambda a: a.strip(), filter(lambda a: a, """
        ##########
        #1# #6  7#
        #   ## # #
        #   #  ###
        # ##4 # 2#
        # ## #  ##
        #5      3#
        ##########
        """.splitlines()))))


class TestGrid(unittest.TestCase):
    def test_index(self):
        g = cypher_grid()
        self.assertEqual(g.index(*list('aeiou')), {
            'a': Point(g, 0, 0),
            'e': Point(g, 4, 0),
            'i': Point(g, 3, 1),
            'o': Point(g, 3, 2),
            'u': Point(g, 4, 3),
        })

    def test_shortest_path(self):
        g = maze()
        p = g.index(*[str(i) for i in range(10)])
        open = ' 123456789'
        def path(a, b): return len(g.shortest_path(p[str(a)], p[str(b)], open))
        self.assertEqual(path(1, 5), 5)
        self.assertEqual(path(2, 4), 8)
        self.assertEqual(path(3, 7), 13)
        self.assertEqual(path(2, 6), 14)

    def test_distances(self):
        g = maze()
        nums = '0123456789'
        open = ' 0123456789'
        dist = g.distances(open, *list(nums))
        self.assertEqual(
            dist['1'], {'2': 14, '3': 12, '4': 10, '5': 5, '6': 16, '7': 17})
        self.assertEqual(
            dist['7'], {'1': 17, '2': 15, '3': 13, '4': 7, '5': 12, '6': 3})


if __name__ == '__main__':
    unittest.main()
