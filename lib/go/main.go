package aoc

import (
	"fmt"
	"time"
)

func Main[T interface{}](part1 func(string) T, part2 func(string) T) {
	input := Input()
	timeSolution(1, input, part1)
	timeSolution(2, input, part2)
}

func timeSolution[T interface{}](part int, input string, fn func(string) T) {
	s := time.Now()
	answer := fn(input)
	d1 := time.Since(s)
	fmt.Printf("Part %d: %v (%s)\n", part, answer, d1)
}
