package main

import (
	"github.com/bpa/aoc"
)

func day7Part1(input string) int {
	upper := make([]bool, 256)
	lower := make([]bool, 256)
	total := 0
	i := -1
	for _, c := range input {
		i++
		lower[i+1] = false
		if c == '.' {
			lower[i] = lower[i] || upper[i]
		} else if c == '^' && upper[i] {
			lower[i-1] = true
			lower[i+1] = true
			total += 1
		} else if c == '\n' {
			i = -1
			upper, lower = lower, upper
			lower[0] = false
		} else if c == 'S' {
			lower[i] = true
		}
	}
	return total
}

func day7Part2(input string) int {
	upper := make([]int, 256)
	lower := make([]int, 256)
	i := -1
	for _, c := range input {
		i++
		lower[i+1] = 0
		if c == '.' {
			lower[i] += upper[i]
		} else if c == '^' && upper[i] > 0 {
			lower[i-1] += upper[i]
			lower[i+1] = upper[i]
		} else if c == '\n' {
			i = -1
			upper, lower = lower, upper
			lower[0] = 0
		} else if c == 'S' {
			lower[i] = 1
		}
	}
	// lower is the active total
	if i == -1 {
		// We finished the row and switched, but there was no more input
		lower = upper
	}
	total := 0
	for _, v := range lower {
		total += v
	}
	return total
}

func main() {
	aoc.Main(day7Part1, day7Part2)
}
