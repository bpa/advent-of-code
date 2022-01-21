package main

import (
	"fmt"
	"github.com/bpa/aoc/util"
	"os"
	"strconv"
	"strings"
	"time"
)

type Registers []int

func (r *Registers) get(i string) int {
	if v, err := strconv.Atoi(i); err == nil {
		return v
	}
	return (*r)[int(i[0])-int('a')]
}

func (r *Registers) set(i string, v int) {
	(*r)[int(i[0])-int('a')] = v
}

func day18Part1(input string) int {
	assembly := parseDay18Assembly(input)
	i := 0
	answer := 0
	registers := Registers(make([]int, 26))
	for {
		inst := assembly[i]
		switch inst[0] {
		case "snd":
			answer = registers.get(inst[1])
		case "set":
			registers.set(inst[1], registers.get(inst[2]))
		case "add":
			registers.set(inst[1], registers.get(inst[1])+registers.get(inst[2]))
		case "mul":
			registers.set(inst[1], registers.get(inst[1])*registers.get(inst[2]))
		case "mod":
			registers.set(inst[1], registers.get(inst[1])%registers.get(inst[2]))
		case "rcv":
			if registers.get(inst[1]) != 0 {
				return answer
			}
		case "jgz":
			if registers.get(inst[1]) > 0 {
				i += registers.get(inst[2]) - 1
			}
		default:
			fmt.Print("Unknown instruction: ", inst[0])
			os.Exit(1)
		}
		i++
	}
}

func day18Part2(input string) int {
	assembly := parseDay18Assembly(input)
	a := make(chan int, 1000)
	b := make(chan int, 1000)
	answer := make(chan int)
	go runDay18(assembly, 0, a, b, answer)
	go runDay18(assembly, 1, b, a, answer)
	return <-answer
}

func runDay18(assembly [][]string, id int, in, out, ans chan int) {
	i := 0
	sent := 0
	registers := Registers(make([]int, 26))
	registers.set("p", id)
	for {
		inst := assembly[i]
		switch inst[0] {
		case "snd":
			sent++
			out <- registers.get(inst[1])
		case "set":
			registers.set(inst[1], registers.get(inst[2]))
		case "add":
			registers.set(inst[1], registers.get(inst[1])+registers.get(inst[2]))
		case "mul":
			registers.set(inst[1], registers.get(inst[1])*registers.get(inst[2]))
		case "mod":
			registers.set(inst[1], registers.get(inst[1])%registers.get(inst[2]))
		case "rcv":
			select {
			case v := <-in:
				registers.set(inst[1], v)
			case <-time.After(1 * time.Millisecond):
				if id == 1 {
					ans <- sent
				}
				return
			}
		case "jgz":
			if registers.get(inst[1]) > 0 {
				i += registers.get(inst[2]) - 1
			}
		default:
			fmt.Print("Unknown instruction: ", inst[0])
			os.Exit(1)
		}
		i++
	}
}

func parseDay18Assembly(input string) [][]string {
	lines := strings.Split(input, "\n")
	assembly := make([][]string, len(lines))
	for i, line := range lines {
		assembly[i] = strings.Fields(line)
	}
	return assembly
}

func main() {
	input := util.Input()
	fmt.Printf("Part 1: %v\n", day18Part1(input))
	fmt.Printf("Part 2: %v\n", day18Part2(input))
}
