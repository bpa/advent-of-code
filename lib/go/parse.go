package aoc

import "strings"

func Delimited(input string, delimiter ...string) [][]string {
	lines := strings.Split(input, "\n")
	grid := make([][]string, 0, len(lines))
	if len(delimiter) == 0 {
		for _, line := range lines {
			if line != "" {
				grid = append(grid, strings.Fields(line))
			}
		}
	} else {
		for _, line := range lines {
			if line != "" {
				grid = append(grid, strings.Split(line, delimiter[0]))
			}
		}
	}
	return grid
}

func Lines(input string) []string {
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

	lines := make([]string, 0, 1024)
	for len(s) > 0 {
		lines = append(lines, line)
		if i := strings.IndexByte(s, '\n'); i >= 0 {
			line, s = s[:i-separator], s[i+1:]
		} else {
			line, s = s, ""
		}
	}
	return append(lines, line)
}
