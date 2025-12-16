package aoc

import (
	"errors"
	"iter"
)

func (p *Point[T]) Get() (*T, error) {
	if !p.Grid.IsValid(p.X, p.Y) {
		return nil, errors.New("point out of bounds")
	}
	return &p.Grid.Data[p.Y][p.X], nil
}

func (p *Point[T]) Neighbor(x, y int) (*Point[T], error) {
	nx := p.X + x
	if nx < 0 || nx >= p.Grid.Width {
		return nil, errors.New("point out of bounds")
	}

	ny := p.Y + y
	if ny < 0 || ny >= p.Grid.Height {
		return nil, errors.New("point out of bounds")
	}

	return &Point[T]{nx, ny, p.Grid}, nil
}

func (p *Point[T]) N() (*Point[T], error) {
	return p.Neighbor(0, -1)
}

func (p *Point[T]) NE() (*Point[T], error) {
	return p.Neighbor(1, -1)
}

func (p *Point[T]) E() (*Point[T], error) {
	return p.Neighbor(1, 0)
}

func (p *Point[T]) SE() (*Point[T], error) {
	return p.Neighbor(1, 1)
}

func (p *Point[T]) S() (*Point[T], error) {
	return p.Neighbor(0, 1)
}

func (p *Point[T]) SW() (*Point[T], error) {
	return p.Neighbor(-1, 1)
}

func (p *Point[T]) W() (*Point[T], error) {
	return p.Neighbor(-1, 0)
}

func (p *Point[T]) NW() (*Point[T], error) {
	return p.Neighbor(-1, -1)
}

func (p *Point[T]) On4(dir int) (*Point[T], error) {
	switch dir {
	case 0:
		return p.N()
	case 1:
		return p.E()
	case 2:
		return p.S()
	case 3:
		return p.W()
	default:
		return nil, errors.New("invalid direction")
	}
}

// Get the n, e, s, w adjacent points
func (p *Point[T]) Adjacent4() iter.Seq[*Point[T]] {
	return func(yield func(*Point[T]) bool) {
		if p, err := p.Neighbor(0, -1); err == nil {
			if !yield(p) {
				return
			}
		}
		if p, err := p.Neighbor(1, 0); err == nil {
			if !yield(p) {
				return
			}
		}
		if p, err := p.Neighbor(0, 1); err == nil {
			if !yield(p) {
				return
			}
		}
		if p, err := p.Neighbor(-1, 0); err == nil {
			yield(p)
		}
	}
}

// Get the n, e, s, w adjacent points
func (p *Point[T]) Adjacent8() iter.Seq[*Point[T]] {
	return func(yield func(*Point[T]) bool) {
		if p, err := p.Neighbor(0, -1); err == nil {
			if !yield(p) {
				return
			}
		}
		if p, err := p.Neighbor(1, -1); err == nil {
			if !yield(p) {
				return
			}
		}
		if p, err := p.Neighbor(1, 0); err == nil {
			if !yield(p) {
				return
			}
		}
		if p, err := p.Neighbor(1, 1); err == nil {
			if !yield(p) {
				return
			}
		}
		if p, err := p.Neighbor(0, 1); err == nil {
			if !yield(p) {
				return
			}
		}
		if p, err := p.Neighbor(-1, 1); err == nil {
			if !yield(p) {
				return
			}
		}
		if p, err := p.Neighbor(-1, 0); err == nil {
			if !yield(p) {
				return
			}
		}
		if p, err := p.Neighbor(-1, -1); err == nil {
			yield(p)
		}
	}
}

func (p *Point[T]) ManhattanDistance(to *Point[T]) int {
	var dist int
	if p.X > to.X {
		dist = p.X - to.X
	} else {
		dist = to.X - p.X
	}
	if p.Y > to.Y {
		dist += p.Y - to.Y
	} else {
		dist += to.Y - p.Y
	}
	return dist
}
