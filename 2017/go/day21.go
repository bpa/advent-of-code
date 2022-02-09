package main

import (
	"fmt"
	"strings"

	"github.com/bpa/aoc/util"
)

func main() {
	input := util.Input()
	two, three := parseDay21(input)
	fmt.Printf("Part 1: %v\n", solveDay21(two, three, 5))
	fmt.Printf("Part 2: %v\n", solveDay21(two, three, 18))
}

func parseDay21(input string) (map[int][]bool, map[int][]bool) {
	two := make(map[int][]bool)
	three := make(map[int][]bool)
	for _, line := range strings.Split(input, "\n") {
		enhancement := strings.Split(line, " => ")
		if len(enhancement[0]) == 5 {
			addVariations(two, enhancement, 2)
		} else {
			addVariations(three, enhancement, 3)
		}
	}
	return two, three
}

func addVariations(mapping map[int][]bool, rule []string, size int) {
	from := toArr21(rule[0])
	to := toArr21(rule[1])
	for i := 0; i < 2; i++ {
		flip21(&from, size)
		for j := 0; j < 4; j++ {
			rotate21(&from, size)
			mapping[toInt21(&from)] = to
		}
	}
}

func toInt21(img *[]bool) int {
	var mask = 0
	for _, c := range *img {
		mask *= 2
		if c {
			mask += 1
		}
	}
	return mask
}

func toArr21(img string) []bool {
	var mask = make([]bool, 0, len(img))
	for _, c := range img {
		switch c {
		case '#':
			mask = append(mask, true)
		case '.':
			mask = append(mask, false)
		}
	}
	return mask
}

func flip21(mask *[]bool, size int) {
	if size == 2 {
		(*mask)[0], (*mask)[1] = (*mask)[1], (*mask)[0]
		(*mask)[2], (*mask)[3] = (*mask)[3], (*mask)[2]
	} else {
		(*mask)[0], (*mask)[2] = (*mask)[2], (*mask)[0]
		(*mask)[3], (*mask)[5] = (*mask)[5], (*mask)[3]
		(*mask)[6], (*mask)[8] = (*mask)[8], (*mask)[6]
	}
}

func rotate21(maskPtr *[]bool, size int) {
	mask := *maskPtr
	if size == 2 {
		tmp := mask[0]
		mask[0] = mask[1]
		mask[1] = mask[3]
		mask[3] = mask[2]
		mask[2] = tmp
	} else {
		tmp := mask[0]
		mask[0] = mask[2]
		mask[2] = mask[8]
		mask[8] = mask[6]
		mask[6] = tmp
		tmp = mask[1]
		mask[1] = mask[5]
		mask[5] = mask[7]
		mask[7] = mask[3]
		mask[3] = tmp
	}
}

func solveDay21(enhance2, enhance3 map[int][]bool, iterations int) int {
	var img = [][]bool{
		{false, true, false},
		{false, false, true},
		{true, true, true},
	}

	for i := 0; i < iterations; i++ {
		if len(img)%2 == 0 {
			img = expand21(enhance2, img, 2)
		} else {
			img = expand21(enhance3, img, 3)
		}
	}
	return count(img)
}

func count(img [][]bool) int {
	var count = 0
	for _, y := range img {
		for _, x := range y {
			if x {
				count++
			}
		}
	}
	return count
}

func expand21(enhance map[int][]bool, img [][]bool, blk int) [][]bool {
	size := len(img)
	newSize := len(img) / blk * (blk + 1)
	newImg := make([][]bool, newSize)
	for i := 0; i < newSize; i++ {
		newImg[i] = make([]bool, newSize)
	}

	for i := 0; i*blk < size; i++ {
		for j := 0; j*blk < size; j++ {
			mask := 0
			for y := blk * i; y < blk*(i+1); y++ {
				for x := blk * j; x < blk*(j+1); x++ {
					mask *= 2
					if img[y][x] {
						mask++
					}
				}
			}

			replacement, ok := enhance[mask]
			if !ok {
				fmt.Println("Missing expansion")
				return img
			}
			z := 0
			for y := (blk + 1) * i; y < (blk+1)*(i+1); y++ {
				for x := (blk + 1) * j; x < (blk+1)*(j+1); x++ {
					newImg[y][x] = replacement[z]
					z++
				}
			}
		}
	}
	return newImg
}
