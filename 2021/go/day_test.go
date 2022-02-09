package main


import (
	"github.com/bpa/aoc/util"
	"testing"
)

var x int

func BenchmarkDay3Part1(b *testing.B) {
	input := util.Input("../input/2021/day3.txt")
	for i:=0; i< b.N; i++ {
		x = day3Part1(input)
	}
}
func BenchmarkDay3Part2(b *testing.B) {
	input := util.Input("../input/2021/day3.txt")
	for i:=0; i< b.N; i++ {
		x = day3Part2(input)
	}
}

