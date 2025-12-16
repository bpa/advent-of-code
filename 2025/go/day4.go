package main

import (
	"fmt"

	"github.com/bpa/aoc"
	"github.com/gammazero/deque"
)

type Cell struct {
	IsRoll    bool
	Neighbors int
}

func day4Part1(input string) int {
	grid, _ := aoc.ParseGridFunc(input, func(r rune) Cell {
		if r == '@' {
			return Cell{IsRoll: true, Neighbors: 0}
		}
		return Cell{IsRoll: false, Neighbors: 0}
	})
	for p := range grid.Points() {
		if c, _ := p.Get(); c.IsRoll {
			for n := range p.Adjacent8() {
				v, _ := n.Get()
				v.Neighbors++
			}
		}
	}
	total := 0
	for p := range grid.Values() {
		if p.IsRoll && p.Neighbors < 4 {
			total++
		}
	}
	return total
}

func day4Part2(input string) int {
	grid, _ := aoc.ParseGridFunc(input, func(r rune) Cell {
		if r == '@' {
			return Cell{IsRoll: true, Neighbors: 0}
		}
		return Cell{IsRoll: false, Neighbors: 0}
	})
	for p := range grid.Points() {
		if c, _ := p.Get(); c.IsRoll {
			for n := range p.Adjacent8() {
				v, _ := n.Get()
				v.Neighbors++
			}
		}
	}
	total := 0
	var q deque.Deque[*aoc.Point[Cell]]
	for p := range grid.Points() {
		if c, _ := p.Get(); c.IsRoll && c.Neighbors < 4 {
			q.PushBack(p)
			total++
		}
	}
	for q.Len() > 0 {
		p := q.PopFront()
		for n := range p.Adjacent8() {
			if v, _ := n.Get(); v.IsRoll {
				v.Neighbors--
				if v.Neighbors == 3 {
					q.PushBack(n)
					total++
				}
			}
		}
	}
	return total
}

func main() {
	input := aoc.Input()
	fmt.Printf("Part 1: %v\n", day4Part1(input))
	fmt.Printf("Part 2: %v\n", day4Part2(input))
}
