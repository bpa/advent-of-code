package aoc

type Coord struct {
	X, Y int
}

type Point[T interface{}] struct {
	X, Y int
	Grid *Grid[T]
}

type Grid[T interface{}] struct {
	Data          [][]T
	Width, Height int
}
