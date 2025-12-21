package main

import (
	"slices"
	"strings"

	"github.com/bpa/aoc"
)

type Point struct {
	x, y, z int
}

func (p *Point) distance(o *Point) int {
	x := p.x - o.x
	y := p.y - o.y
	z := p.z - o.z
	return x*x + y*y + z*z
}

type Distance struct {
	dist int
	a, b Point
}

func NewPoint(point string) Point {
	n := strings.Split(point, ",")
	return Point{aoc.ToInt(n[0]), aoc.ToInt(n[1]), aoc.ToInt(n[2])}
}

func parse(input string) ([]Distance, int) {
	junctions := make([]Point, 0, 1024)
	distances := make([]Distance, 0, 500_000)
	for _, s := range strings.Fields(input) {
		p := NewPoint(s)
		for _, o := range junctions {
			distances = append(distances, Distance{p.distance(&o), p, o})
		}
		junctions = append(junctions, p)
	}

	slices.SortFunc(distances, func(i, j Distance) int {
		return i.dist - j.dist
	})
	return distances, len(junctions)
}

func merge(d Distance, circuit *[]map[Point]bool, cluster *map[Point]int) {
	if a, ok := (*cluster)[d.a]; ok {
		if b, ok := (*cluster)[d.b]; ok {
			if a == b {
				return
			}
			for c, _ := range (*circuit)[b] {
				(*cluster)[c] = a
				(*circuit)[a][c] = true
			}
			clear((*circuit)[b])
		} else {
			(*cluster)[d.b] = a
			(*circuit)[a][d.b] = true
		}
	} else if b, ok := (*cluster)[d.b]; ok {
		(*cluster)[d.a] = b
		(*circuit)[b][d.a] = true
	} else {
		id := len(*circuit)
		(*cluster)[d.a] = id
		(*cluster)[d.b] = id
		*circuit = append(*circuit, map[Point]bool{d.a: true, d.b: true})
	}
}

func day8Part1(input string) int {
	distances, _ := parse(input)

	circuit := make([]map[Point]bool, 0, 1024)
	cluster := make(map[Point]int)
	rounds := 1000
	if len(distances) < 1000 {
		rounds = 10
	}
	for i := 0; i < rounds; i++ {
		d := distances[i]
		merge(d, &circuit, &cluster)
	}
	slices.SortFunc(circuit, func(a, b map[Point]bool) int {
		return len(b) - len(a)
	})

	total := 1
	for _, c := range circuit[:3] {
		total *= len(c)
	}
	return total
}

func day8Part2(input string) int {
	distances, junctions := parse(input)

	circuit := make([]map[Point]bool, 0, 1024)
	cluster := make(map[Point]int)
	for _, d := range distances {
		merge(d, &circuit, &cluster)
		if len(circuit[cluster[d.a]]) == junctions {
			return d.a.x * d.b.x
		}
	}

	return 0
}

func main() {
	aoc.Main(day8Part1, day8Part2)
}
