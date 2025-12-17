package main

import (
	"iter"
	"sort"
	"strings"

	"github.com/bpa/aoc"
)

type Range struct {
	Start int
	End   int
}

func day5Part1(input string) int {
	next, _ := iter.Pull(strings.Lines(input))
	fresh := make([]Range, 0, 1024)
	for line, end := next(); end; line, end = next() {
		parts := strings.Split(strings.TrimRight(line, "\r\n"), "-")
		if len(parts) != 2 {
			break
		}
		fresh = append(fresh, Range{
			Start: aoc.ToInt(parts[0]),
			End:   aoc.ToInt(parts[1]),
		})
	}
	total := 0
	for line, end := next(); end; line, end = next() {
		num := aoc.ToInt(line)
		for _, f := range fresh {
			if num >= f.Start && num <= f.End {
				total++
				break
			}
		}
	}
	return total
}

func day5Part2(input string) int {
	next, stop := iter.Pull(strings.Lines(input))
	fresh := make([]Range, 0, 1024)
	for line, end := next(); end; line, end = next() {
		parts := strings.Split(strings.TrimRight(line, "\r\n"), "-")
		if len(parts) != 2 {
			stop()
			break
		}
		fresh = append(fresh, Range{
			Start: aoc.ToInt(parts[0]),
			End:   aoc.ToInt(parts[1]),
		})
	}

	sort.Slice(fresh, func(i, j int) bool {
		return fresh[i].Start < fresh[j].Start
	})

	total := 0
	end := -1
	for _, f := range fresh {
		if f.Start <= end {
			if f.End > end {
				total += f.End - end
				end = f.End
			}
		} else {
			total += f.End - f.Start + 1
			end = f.End
		}
	}

	return total
}

func main() {
	aoc.Main(day5Part1, day5Part2)
}
