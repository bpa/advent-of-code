package main

import (
	"fmt"
	"regexp"
	"strconv"

	"github.com/bpa/aoc/util"
)

func day15Part1(input string) int {
	a, b := seeds(input)
	return solveDay15(a, 16807, 1, b, 48271, 1, 40_000_000)
}

func day15Part2(input string) int {
	a, b := seeds(input)
	return solveDay15(a, 16807, 4, b, 48271, 8, 5_000_000)
}

func seeds(input string) (int, int) {
	digits := regexp.MustCompile(`\d+`)
	found := digits.FindAllString(input, -1)
	nums := make([]int, 2)
	for i, d := range found {
		if v, err := strconv.Atoi(d); err == nil {
			nums[i] = v
		}
	}

	return nums[0], nums[1]
}

func solveDay15(seedA, factorA, mulA, seedB, factorB, mulB, iterations int) int {
	var a = make(chan int)
	var b = make(chan int)

	go generateDay15(seedA, factorA, iterations, mulA, a)
	go generateDay15(seedB, factorB, iterations, mulB, b)

	var total = 0
	for {
		x, ok := <-a
		if !ok {
			break
		}
		y := <-b
		if x&0xFFFF == y&0xFFFF {
			total++
		}
	}

	return total
}

func generateDay15(seed, factor, count, multiple int, out chan int) {
	for i := 0; i < count; i++ {
		for {
			seed = (seed * factor) % 2147483647
			if seed%multiple == 0 {
				break
			}
		}
		out <- seed
	}
	close(out)
}

func main() {
	input := util.Input()
	fmt.Printf("Part 1: %v\n", day15Part1(input))
	fmt.Printf("Part 2: %v\n", day15Part2(input))
}
