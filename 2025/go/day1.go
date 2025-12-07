package main

import (
	"fmt"
	"strings"

	"github.com/bpa/aoc"
)

func day1Part1(input string) int {
	count := 0
	pos := 50
	for _, l := range strings.Split(input, "\n") {
		if len(l) > 1 {
			val := aoc.ToInt(l[1:])
			if l[0] == 'L' {
				pos -= val
			} else {
				pos += val
			}
			pos = (pos%100 + 100) % 100
			if pos == 0 {
				count += 1
			}
			pos = pos % 100
		}
	}
	return count
}

func day1Part2(input string) int {
	count := 0
	pos := 50
	for _, l := range strings.Split(input, "\n") {
		if len(l) > 1 {
			val := aoc.ToInt(l[1:])
			if l[0] == 'L' {
				extra := 0
				if pos > 0 {
					extra = 1
				}
				pos -= val
				if pos < 1 {
					count += pos/-100 + extra
				}
			} else {
				pos += val
				if pos > 99 {
					count += pos / 100
				}
			}
			pos = (pos%100 + 100) % 100
		}
	}
	return count
}

func main() {
	input := aoc.Input()
	fmt.Printf("Part 1: %v\n", day1Part1(input))
	fmt.Printf("Part 2: %v\n", day1Part2(input))
}
