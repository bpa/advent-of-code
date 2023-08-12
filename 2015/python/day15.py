#!/usr/bin/env python3

from aoc import *


def calculate(state, ingredients, care_about_calories):
    if care_about_calories:
        total = 0
        for j, ing in enumerate(ingredients):
            total += state[j] * ing[4]
        if total != 500:
            return 0

    total = 1
    for i in range(4):
        amount = 0
        for j, ing in enumerate(ingredients):
            amount += state[j] * ing[i]
        if amount < 0:
            total = 0
        else:
            total *= amount
    return total


def solve(input, care_about_calories):
    teaspoons = 100
    ingredients = list(comb(input, r"(-?\d+)", int))
    best = 0
    state0 = [0] * len(ingredients)
    state1 = [0] * len(ingredients)
    state0[0] = teaspoons
    i = 0
    j = len(ingredients) - 1
    while True:
        state1[i] = state0[i]
        score = calculate(state1, ingredients, care_about_calories)
        if score > best:
            best = score
        if i < j:
            state1[i] -= 1
            state0[i + 1] = state0[i] - state1[i]
            i += 1
            continue

        while True:
            state1[i] = 0
            i -= 1
            if state1[i]:
                state1[i] -= 1
                state0[i + 1] = state0[i] - state1[i]
                i += 1
                break
            elif i == 0:
                return best


if __name__ == "__main__":
    main(False, True)
