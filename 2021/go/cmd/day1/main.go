package main

import (
	"aoc"
	"fmt"
	"strconv"
	"strings"
)

func day1Part1(input string) int {
	var count = 0
	var prior = -1
	for _, depth := range strings.Split(input, "\n") {
		if d, err := strconv.Atoi(depth); err == nil {
			if d > prior {
				count += 1
			}
			prior = d
		}
	}
	return count - 1
}

func toInt(a string) int {
	if v, err := strconv.Atoi(a); err == nil {
		return v
	}
	return 0
}

func day1Part2(input string) int {
	var count = 0
	var prior = 0
	numbers := strings.Split(input, "\n")
	for i := 0; i < 3; i++ {
		prior += toInt(numbers[i])
	}
	for i := 0; i < len(numbers)-3; i++ {

		var next = prior - toInt(numbers[i]) + toInt(numbers[i+3])
		if next > prior {
			count += 1
		}
		prior = next
	}
	return count
}

func main1() {
	input := util.Input()
	fmt.Printf("Part 1: %v\n", day1Part1(input))
	fmt.Printf("Part 2: %v\n", day1Part2(input))
}
