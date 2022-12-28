#!/usr/bin/env python3

from aoc import *


def is_visible(tree, dir):
    height = tree.get()
    tree = dir(tree)
    while tree is not None:
        if tree >= height:
            return False
        tree = dir(tree)
    return True


def part1(input):
    visible = 0
    forest = grid(input, map=int)
    for tree in forest:
        if is_visible(tree, Point.n) or is_visible(tree, Point.w) or is_visible(tree, Point.s) or is_visible(tree, Point.e):
            visible += 1
    return visible


def view(tree: Point, dir):
    max_height = tree.get()
    seen = 0
    tree = dir(tree)
    while tree is not None:
        seen += 1
        if tree.get() < max_height:
            tree = dir(tree)
        else:
            break
    return seen


def part2(input):
    high_score = 0
    forest = grid(input, map=int)
    for tree in forest:
        score = 1
        score *= view(tree, Point.n)
        score *= view(tree, Point.e)
        score *= view(tree, Point.s)
        score *= view(tree, Point.w)
        high_score = max(high_score, score)
    return high_score


if __name__ == '__main__':
    main()
