package main

import (
	"strings"

	"github.com/bpa/aoc"
)

func day12Part1(input string) int {
	total := 0
	for line := range strings.Lines(input) {
		shapes := strings.Split(line, " ")
		if len(shapes) != 7 {
			continue
		}
		dims := strings.Split(shapes[0], "x")
		avail := (aoc.ToInt(dims[0]) / 3) * (aoc.ToInt(dims[1][:len(dims[1])-1]) / 3)
		for _, p := range shapes[1:] {
			avail -= aoc.ToInt(p)
		}
		if avail >= 0 {
			total += 1
		}
	}
	return total
}

func main() {
	input := aoc.Input()
	aoc.TimeSolution(1, input, day12Part1)
}
