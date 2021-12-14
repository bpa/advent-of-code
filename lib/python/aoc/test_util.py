import unittest
from .util import chunks


class TestUtil(unittest.TestCase):
    def test_chunks(self):
        self.assertEqual(list(chunks('abcd', 2)), [['a', 'b'], ['c', 'd']])
        self.assertEqual(list(chunks('abcde', 2)), [
                         ['a', 'b'], ['c', 'd'], ['e']])
        self.assertEqual(list(chunks('abcde', 2, pad='f')), [
                         ['a', 'b'], ['c', 'd'], ['e', 'f']])
