package main

import (
	"github.com/bpa/aoc"
)

func add(a, b int) int {
	return a + b
}

func mul(a, b int) int {
	return a * b
}

func day6Part1(input string) int {
	lines := aoc.Delimited(input)
	total := 0
	end := len(lines) - 1
	for i, op := range lines[end] {
		var f func(int, int) int
		if op == "*" {
			f = mul
		} else {
			f = add
		}
		subtotal := aoc.ToInt(lines[0][i])
		for _, n := range lines[1:end] {
			subtotal = f(subtotal, aoc.ToInt(n[i]))
		}
		total += subtotal
	}
	return total
}

func day6Part2(input string) int {
	lines := aoc.Lines(input)
	end := len(lines) - 1
	total := 0
	subtotal := 0
	var f func(int, int) int
	for i, c := range lines[end] {
		value := 0
		isNumber := false
		for _, n := range lines[:end] {
			x := n[i]
			if x != ' ' {
				isNumber = true
				value *= 10
				value += int(x) - '0'
			}
		}
		if !isNumber {
			total += subtotal
		} else if c == '*' {
			f = mul
			subtotal = value
		} else if c == '+' {
			f = add
			subtotal = value
		} else {
			subtotal = f(subtotal, value)
		}
	}
	return total + subtotal
}

func main() {
	aoc.Main(day6Part1, day6Part2)
}
