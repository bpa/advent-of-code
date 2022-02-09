package main

import "C"
import (
	"fmt"
)

const (
	a = iota
	b
	c
	d
	e
	f
)

type state25 struct {
	write     int
	move      int
	nextState int
}

func solve25(steps int, states []state25) int {
	tape := make(map[int]struct{})
	var pos = 0
	var state = 0
	for i := 0; i < steps; i++ {
		var action state25
		if _, on := tape[pos]; on {
			action = states[state*2+1]
		} else {
			action = states[state*2]
		}
		if action.write == 1 {
			tape[pos] = struct{}{}
		} else {
			delete(tape, pos)
		}
		pos += action.move
		state = action.nextState
	}
	return len(tape)
}

func main() {
	//input := util.Input()
	fmt.Printf("Part 1: %v\n", solve25(6,
		[]state25{
			// A
			{1, 1, b},
			{0, -1, b},
			// B
			{1, -1, a},
			{1, 1, a},
		}))
	fmt.Printf("Part 2: %v\n", solve25(12964419,
		[]state25{
			// A
			{1, 1, b},
			{0, 1, f},
			// B
			{0, -1, b},
			{1, -1, c},
			// C
			{1, -1, d},
			{0, 1, c},
			// D
			{1, -1, e},
			{1, 1, a},
			// E
			{1, -1, f},
			{0, -1, d},
			// F
			{1, 1, a},
			{0, -1, e},
		}))
}
