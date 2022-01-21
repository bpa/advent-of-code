package util

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