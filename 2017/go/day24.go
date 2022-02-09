package main

import (
	"fmt"
	"strings"

	"github.com/bpa/aoc/util"
)

func strongest24(start int, pieces *[]bridge, lookup map[int]*[]int) int {
	var best = 0
	if segments, exist := lookup[start]; exist {
		for _, s := range *segments {
			piece := &(*pieces)[s]
			if !piece.used {
				piece.used = true
				var span int
				if piece.a == start {
					span = strongest24(piece.b, pieces, lookup)
				} else {
					span = strongest24(piece.a, pieces, lookup)
				}
				if span+piece.strength > best {
					best = span + piece.strength
				}
				piece.used = false
			}
		}
	}
	return best
}

func longest24(start int, pieces *[]bridge, lookup map[int]*[]int) (int, int) {
	var longest = 0
	var strongest = 0
	if segments, exist := lookup[start]; exist {
		for _, s := range *segments {
			piece := &(*pieces)[s]
			if !piece.used {
				piece.used = true
				var span, str int
				if piece.a == start {
					span, str = longest24(piece.b, pieces, lookup)
				} else {
					span, str = longest24(piece.a, pieces, lookup)
				}
				span += 1
				str += piece.strength
				if span > longest {
					longest = span
					strongest = str
				} else if span == longest && str > strongest {
					strongest = str
				}
				piece.used = false
			}
		}
	}
	return longest, strongest
}

type bridge struct {
	a, b, strength int
	used           bool
}

func parse24(input string) ([]bridge, map[int]*[]int) {
	lines := strings.Split(input, "\n")
	pieces := make([]bridge, len(lines))
	lookup := make(map[int]*[]int)
	for i, piece := range lines {
		side := util.ListOfInt(piece, "/")
		pieces[i] = bridge{side[0], side[1], side[0] + side[1], false}
		for _, s := range side {
			list, exists := lookup[s]
			if exists {
				*list = append(*list, i)
			} else {
				lookup[s] = &[]int{i}
			}
		}
	}
	return pieces, lookup
}

func main() {
	input := util.Input()
	pieces, lookup := parse24(input)
	fmt.Printf("Part 1: %v\n", strongest24(0, &pieces, lookup))
	_, strength := longest24(0, &pieces, lookup)
	fmt.Printf("Part 2: %v\n", strength)
}
