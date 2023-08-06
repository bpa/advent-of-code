package main

import (
	"aoc"
	"fmt"
	"strings"
)

func day2Part1(input string) int {
	var x = 0
	var y = 0
	for _, instruction := range strings.Split(input, "\n") {
		tokens := strings.Split(instruction, " ")
		if tokens[0] == "forward" {
			x += util.ToInt(tokens[1])
		} else if tokens[0] == "up" {
			y -= util.ToInt(tokens[1])
		} else if tokens[0] == "down" {
			y += util.ToInt(tokens[1])
		}
	}
	return x * y
}

func day2Part2(input string) int {
	var x = 0
	var y = 0
	var aim = 0
	for _, instruction := range strings.Split(input, "\n") {
		tokens := strings.Split(instruction, " ")
		if len(tokens) != 2 {
			break
		}
		amount := util.ToInt(tokens[1])
		if tokens[0] == "forward" {
			x += amount
			y += aim * amount
		} else if tokens[0] == "up" {
			aim -= amount
		} else if tokens[0] == "down" {
			aim += amount
		}
	}
	return x * y
}

func main2() {
	input := util.Input()
	fmt.Printf("Part 1: %v\n", day2Part1(input))
	fmt.Printf("Part 2: %v\n", day2Part2(input))
}
