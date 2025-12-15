package main

import (
	"fmt"

	"github.com/bpa/aoc"
)

type Cell struct {
	IsRoll    bool
	Neighbors int
}

func day4Part1(input string) int {
	grid, _ := aoc.ParseGridFunc(input, func(r rune) Cell {
		if r == '@' {
			return Cell{IsRoll: true, Neighbors: 0}
		}
		return Cell{IsRoll: false, Neighbors: 0}
	})
	x := 0
	for c := range grid.Cells() {
		if c.IsRoll {
			aoc.Debug("@")
		} else {
			aoc.Debug(".")
		}
		x++
		if x == grid.Width {
			aoc.Debug("\n")
			x = 0
		}
	}
	return 0
}

func day4Part2(input string) int {
	return 0
}

func main() {
	input := aoc.Input()
	fmt.Printf("Part 1: %v\n", day4Part1(input))
	fmt.Printf("Part 2: %v\n", day4Part2(input))
}
