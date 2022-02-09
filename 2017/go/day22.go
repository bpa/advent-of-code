package main

import (
	"fmt"
	"strings"

	"github.com/bpa/aoc/util"
)

const (
	weakened = iota
	infected
	flagged
)

func day22Part1(input string) int {
	nodes, center := parse22(input)
	count := 0
	v := virus{0, [2]int{center, center}}
	for i := 0; i < 10_000; i++ {
		if _, exists := nodes[v.loc]; exists {
			v.turnRight()
			delete(nodes, v.loc)
		} else {
			nodes[v.loc] = infected
			v.turnLeft()
			count++
		}
		v.move()
	}
	return count
}

func day22Part2(input string) int {
	nodes, center := parse22(input)
	count := 0
	v := virus{0, [2]int{center, center}}
	for i := 0; i < 10_000_000; i++ {
		if state, exists := nodes[v.loc]; exists {
			switch state {
			case weakened:
				nodes[v.loc] = infected
				count++
			case infected:
				nodes[v.loc] = flagged
				v.turnRight()
			case flagged:
				delete(nodes, v.loc)
				v.turnAround()
			}
		} else {
			nodes[v.loc] = weakened
			v.turnLeft()
		}
		v.move()
	}
	return count
}

func parse22(input string) (map[[2]int]int, int) {
	nodes := make(map[[2]int]int)
	rows := strings.Split(input, "\n")
	for y, row := range rows {
		for x, node := range row {
			if node == '#' {
				k := [2]int{x, y}
				nodes[k] = infected
			}
		}
	}
	return nodes, len(rows) / 2
}

type virus struct {
	dir int
	loc [2]int
}

func (p *virus) turnRight() {
	p.dir = (p.dir + 1) % 4
}

func (p *virus) turnLeft() {
	p.dir = (p.dir + 3) % 4
}

func (p *virus) turnAround() {
	p.dir = (p.dir + 2) % 4
}

func (p *virus) move() {
	switch p.dir {
	case 0:
		p.loc[1]--
	case 1:
		p.loc[0]++
	case 2:
		p.loc[1]++
	case 3:
		p.loc[0]--
	}
}

func main() {
	input := util.Input()
	fmt.Printf("Part 1: %v\n", day22Part1(input))
	fmt.Printf("Part 2: %v\n", day22Part2(input))
}
