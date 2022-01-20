package main

import (
	"fmt"
	"strings"

	"github.com/bpa/aoc/util"
)

func day3Part1(input string) int {
	lines := strings.Split(input, "\n")
	counts := make([]int, len(lines[0]))
	for _, diag := range lines {
		for i, c := range diag {
			if c == '1' {
				counts[i] += 1
			}
		}
	}

	var gamma = 0
	for _, c := range counts {
		gamma *= 2
		if c >= len(lines)/2 {
			gamma += 1
		}
	}
	return gamma * (gamma ^ ((1 << len(lines[0])) - 1))
}

func fromBinary(counts []bool) int {
	var val = 0
	for _, b := range counts {
		val *= 2
		if b {
			val += 1
		}
	}
	return val
}

func day3Part2(input string) int {
	lines := strings.Split(input, "\n")
	oxygen := make([][]bool, len(lines))
	for i, line := range lines {
		oxygen[i] = make([]bool, len(lines[0]))
		for j, c := range line {
			oxygen[i][j] = c == '1'
		}
	}

	scrubber := oxygen

	for i := 0; i < len(oxygen[0]); i++ {
		oxygen = filter(oxygen, i, true)
	}

	for i := 0; i < len(scrubber[0]); i++ {
		scrubber = filter(scrubber, i, false)
	}

	return fromBinary(oxygen[0]) * fromBinary(scrubber[0])
}

func filter(counts [][]bool, i int, high bool) [][]bool {
	if len(counts) == 1 {
		return counts
	}

	var sum = 0
	for _, row := range counts {
		if row[i] {
			sum += 1
		}
	}

	var half = (len(counts) + 1) / 2
	var wanted = sum < half != high
	var size = sum
	if !wanted {
		size = len(counts) - sum
	}
	var filtered = make([][]bool, 0, size)
	for _, row := range counts {
		if row[i] == wanted {
			filtered = append(filtered, row)
		}
	}

	return filtered
}

func main() {
	input := util.Input()
	fmt.Printf("Part 1: %v\n", day3Part1(input))
	fmt.Printf("Part 2: %v\n", day3Part2(input))
}
