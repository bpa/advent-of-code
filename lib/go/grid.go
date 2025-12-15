package aoc

import (
	"errors"
	"iter"
	"strings"
)

type Cell interface{}

type Grid[C Cell] struct {
	Data          [][]C
	Width, Height int
}

func ParseGrid(input string) (*Grid[rune], error) {
	return ParseGridFunc(input, func(r rune) rune {
		return r
	})
}

func ParseGridBool(input string) (*Grid[bool], error) {
	return ParseGridFunc(input, func(r rune) bool {
		return r != '.' && r != ' '
	})
}

func ParseGridFunc[C Cell](input string, parseCell func(rune) C) (*Grid[C], error) {
	if len(input) == 0 {
		return nil, errors.New("empty input")
	}

	s := input
	separator := 0
	var line string
	if i := strings.IndexByte(s, '\n'); i >= 0 {
		line, s = s[:i], s[i+1:]
		l := len(line)
		if line[l-1] == '\r' {
			separator = 1
			line = line[:l-1]
		}
	} else {
		line, s = s, ""
	}
	width := len(line)

	cells := make([][]C, 0, 1024)
	for len(line) > 0 {
		row := make([]C, width)
		for x, ch := range line {
			row[x] = parseCell(ch)
		}
		cells = append(cells, row)
		if i := strings.IndexByte(s, '\n'); i >= 0 {
			line, s = s[:i-separator], s[i+1:]
		} else {
			line, s = s, ""
		}
	}
	return &Grid[C]{
		Data:   cells,
		Width:  width,
		Height: len(cells),
	}, nil
}

func (g *Grid[C]) Cells() iter.Seq[C] {
	return func(yield func(C) bool) {
		for y := 0; y < g.Height; y++ {
			for x := 0; x < g.Width; x++ {
				if !yield(g.Data[y][x]) {
					return
				}
			}
		}
	}
}
