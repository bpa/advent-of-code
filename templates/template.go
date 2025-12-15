package main

import (
	"fmt"

	"github.com/bpa/aoc"
)

func day{{ .Day }}Part1(input string) int {
	return 0
}

func day{{ .Day }}Part2(input string) int {
	return 0
}

func main() {
	input := aoc.Input()
	fmt.Printf("Part 1: %v\n", day{{ .Day }}Part1(input))
	fmt.Printf("Part 2: %v\n", day{{ .Day }}Part2(input))
}
