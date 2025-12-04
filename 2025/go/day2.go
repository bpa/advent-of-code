package main

import (
	"fmt"
	"math"
	"strings"

	"github.com/bpa/aoc"
)

func invalid(a, b string) int {
	sum := 0
	left := int(aoc.ToInt(a))
	right := int(aoc.ToInt(b))
	l := len(a)
	start := 0
	max := 0
	if l%2 == 0 {
		start = int(aoc.ToInt(a[:l/2]))
		max = int(math.Pow10(l / 2))
	} else {
		start = int(math.Pow10(l / 2))
		max = start * 10
	}
	for {
		for i := start; i < max; i++ {
			v := i*max + i
			if v < left {
				continue
			}
			if right < v {
				return sum
			}
			sum += v
		}
		start = max * 10
		max = start * 10
	}
}

func day2Part1(input string) int {
	sum := 0
	for _, r := range strings.Split(strings.TrimSpace(input), ",") {
		nums := strings.Split(r, "-")
		sum += invalid(nums[0], nums[1])
	}
	return sum
}

func invalid_n(digits, repeats, start, min, max int, invalid map[int]bool) {
	end := int(math.Pow10(digits))
	for i := start; i < end; i++ {
		v := 0
		for j := 0; j < repeats; j++ {
			v *= end
			v += i
		}
		if v < min {
			continue
		}
		if v > max {
			return
		}
		invalid[v] = true
	}
}

func invalid_part2(a, b string) int {
	invalid := make(map[int]bool)
	left := aoc.ToInt(a)
	right := aoc.ToInt(b)
	l := len(a)
	r := len(b)
	for i := 1; i <= r/2; i++ {
		if l > 1 && l%i == 0 {
			start := aoc.ToInt(a[:i])
			invalid_n(i, l/i, start, left, right, invalid)
		}
		if l != r && r%i == 0 {
			start := int(math.Pow10(i - 1))
			invalid_n(i, r/i, start, left, right, invalid)
		}
	}
	sum := 0
	for i, _ := range invalid {
		sum += i
	}
	return sum
}

func day2Part2(input string) int {
	sum := 0
	for _, r := range strings.Split(strings.TrimSpace(input), ",") {
		nums := strings.Split(r, "-")
		sum += invalid_part2(nums[0], nums[1])
	}
	return sum
}

func main() {
	input := aoc.Input()
	fmt.Printf("Part 1: %v\n", day2Part1(input))
	fmt.Printf("Part 2: %v\n", day2Part2(input))
}
