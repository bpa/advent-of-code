package main

import (
	"github.com/bpa/aoc"
	"github.com/gammazero/deque"
)

type Day11Node struct {
	name     string
	input    []int
	output   []int
	paths    int
	enqueued bool
}

func newDay11Node(name string) *Day11Node {
	return &Day11Node{
		name:   name,
		input:  make([]int, 0, 16),
		output: make([]int, 0, 16),
	}
}

func newDay11Graph(input string) aoc.IndexedArray[string, *Day11Node] {
	graph := aoc.NewIndexedArrayFunc(1024, newDay11Node)
	for _, line := range aoc.Delimited(input) {
		name := line[0]
		name = name[:len(name)-1]
		i, input := graph.GetOrCreate(name)
		for _, c := range line[1:] {
			o, output := graph.GetOrCreate(c)
			input.output = append(input.output, o)
			output.input = append(output.input, i)
		}
	}
	return graph
}

func day11Dist(graph *aoc.IndexedArray[string, *Day11Node], from, to string, clear bool) int {
	f, ok := graph.Ind[from]
	if !ok {
		return 0
	}

	if clear {
		for _, n := range graph.Data {
			n.paths = 0
		}
	}
	graph.Data[f].paths = 1

	var queue deque.Deque[int]
	queue.SetBaseCap(1024)
	queue.CopyInSlice(graph.Data[f].output)
	for queue.Len() > 0 {
		i := queue.PopFront()
		node := graph.Data[i]
		total := 0
		for _, j := range node.input {
			total += graph.Data[j].paths
		}
		node.paths = total
		for _, j := range node.output {
			out := graph.Data[j]
			if !(out.enqueued) {
				out.enqueued = true
				queue.PushBack(j)
			}
		}
		node.enqueued = false
	}
	_, t := graph.GetOrCreate(to)
	return t.paths
}

func day11Part1(input string) int {
	graph := newDay11Graph(input)
	return day11Dist(&graph, "you", "out", false)
}

func day11Part2(input string) int {
	graph := newDay11Graph(input)
	dac := day11Dist(&graph, "svr", "dac", false)
	_, fftNode := graph.GetOrCreate("fft")
	fft := fftNode.paths
	if dac < fft {
		middle := day11Dist(&graph, "dac", "fft", true)
		return dac * middle * day11Dist(&graph, "fft", "out", true)
	} else {
		middle := day11Dist(&graph, "fft", "dac", true)
		return fft * middle * day11Dist(&graph, "dac", "out", true)
	}
}

func main() {
	aoc.Main(day11Part1, day11Part2)
}
