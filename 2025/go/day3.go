package main

import (
	"fmt"
	"strings"
	"time"

	"github.com/bpa/aoc"
)

func solveDay3(input string, banks int) int {
	total := 0
	for _, line := range strings.Split(input, "\n") {
		ints := make([]int, 0, len(line))
		for _, c := range []byte(line) {
			ints = append(ints, int(c-'0'))
		}
		joltage := 0
		start := 0
		end := len(line) - banks + 1
		for i := 0; i < banks; i++ {
			max := 0
			ind := 0
			for o := start; o < end; o++ {
				if ints[o] > max {
					max = ints[o]
					ind = o
				}
			}
			joltage *= 10
			joltage += max
			start = ind + 1
			end += 1
		}
		total += joltage
	}
	return total
}

func main() {
	input := aoc.Input()
	s := time.Now()
	fmt.Printf("Part 1: %v (%v)\n", solveDay3(input, 2), time.Since(s))
	s = time.Now()
	fmt.Printf("Part 2: %v (%v)\n", solveDay3(input, 12), time.Since(s))
}
