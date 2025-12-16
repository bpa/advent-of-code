package aoc

import (
	"errors"
	"iter"
	"strings"
)

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

func ParseGridFunc[T interface{}](input string, parseCell func(rune) T) (*Grid[T], error) {
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

	cells := make([][]T, 0, 1024)
	for len(line) > 0 {
		row := make([]T, width)
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
	return &Grid[T]{
		Data:   cells,
		Width:  width,
		Height: len(cells),
	}, nil
}

func (g *Grid[T]) Points() iter.Seq[*Point[T]] {
	return func(yield func(*Point[T]) bool) {
		for y := 0; y < g.Height; y++ {
			for x := 0; x < g.Width; x++ {
				if !yield(&Point[T]{x, y, g}) {
					return
				}
			}
		}
	}
}

func (g *Grid[T]) Values() iter.Seq[*T] {
	return func(yield func(*T) bool) {
		for y := 0; y < g.Height; y++ {
			for x := 0; x < g.Width; x++ {
				if !yield(&g.Data[y][x]) {
					return
				}
			}
		}
	}
}

// Find all points given, return a map of item: point
func (g *Grid[T]) IsValid(x, y int) bool {
	return x >= 0 && x < g.Width && y >= 0 && y < g.Height
}

// Get the Point at x, y
func (g *Grid[T]) At(x, y int) (Point[T], error) {
	if !g.IsValid(x, y) {
		return Point[T]{}, errors.New("point out of bounds")
	}
	return Point[T]{x, y, g}, nil
}

// Get the element at the point
func (g *Grid[T]) Get(x, y int) (*T, error) {
	if !g.IsValid(x, y) {
		return nil, errors.New("point out of bounds")
	}
	return &g.Data[y][x], nil
}

// Set the element at the point
func (g *Grid[T]) Set(x, y int, value T) {
	g.Data[y][x] = value
}

func (g *Grid[T]) Range(x0, y0, x1, y1 int) iter.Seq2[Coord, *T] {
	if x0 < 0 {
		x0 = 0
	}
	if x1 > g.Width {
		x1 = g.Width
	}
	if y0 < 0 {
		y0 = 0
	}
	if y1 > g.Height {
		y1 = g.Height
	}
	return func(yield func(Coord, *T) bool) {
		for y := y0; y < y1; y++ {
			for x := 0; x < x1; x++ {
				if !yield(Coord{x, y}, &g.Data[y][x]) {
					return
				}
			}
		}
	}
}

// Get the n, e, s, w adjacent points
func (g *Grid[T]) Adjacent4(x, y int) iter.Seq[*Point[T]] {
	return (&Point[T]{x, y, g}).Adjacent4()
}

// Get the n, e, s, w adjacent points
func (g *Grid[T]) Adjacent8(x, y int) iter.Seq[*Point[T]] {
	return (&Point[T]{x, y, g}).Adjacent8()
}
