package main

import (
	"fmt"
	"math/big"
	"os"
	"regexp"
	"strconv"

	"github.com/bpa/aoc/util"
)

func day23Part1(input int64) int64 {
	return (input - 2) * (input - 2)
}

func day23Part2(input int64) int64 {
	var nonPrimesFound int64 = 0
	b := input*100 + 100_000
	for i := 0; i <= 1000; i++ {
		if !big.NewInt(b).ProbablyPrime(0) {
			nonPrimesFound++
		}
		b += 17
	}
	return nonPrimesFound
}

func main() {
	input := util.Input()
	re := regexp.MustCompile(`(\d+)`)
	start := re.Find([]byte(input))
	b, err := strconv.ParseInt(string(start), 10, 64)
	if err != nil {
		fmt.Printf("Bad input")
		os.Exit(1)
	}
	fmt.Printf("Part 1: %v\n", day23Part1(b))
	fmt.Printf("Part 2: %v\n", day23Part2(b))
}
