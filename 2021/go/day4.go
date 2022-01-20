package main

import (
	"fmt"
	"sort"
	"strings"
	"sync"

	"github.com/bpa/aoc/util"
)

func solveDay4(input string) int {
	blocks := strings.Split(input, "\n\n")
	numbers := util.ListOfInt(blocks[0], ",")
	players := make([]chan int, len(blocks)-1)
	complete := make(chan [2]int, len(blocks))

	var wg sync.WaitGroup
	for i, board := range blocks[1:] {
		players[i] = make(chan int)
		wg.Add(1)
		go func(i int, board *Board, numbers chan int) {
			defer wg.Done()
			var read = 0
			for {
				n := <-numbers
				read++
				if board.Call(n) {
					complete <- [2]int{read, board.Sum() * n}
					break
				}
			}
			for {
				if _, ok := <-numbers; !ok {
					break
				}
			}
		}(i, newBoard(board), players[i])
	}

	for _, n := range numbers {
		for _, p := range players {
			p <- n
		}
	}

	for _, p := range players {
		close(p)
	}

	wg.Wait()
	close(complete)
	done := make([][2]int, 0, len(blocks)-1)
	for {
		if board, ok := <-complete; ok {
			done = append(done, board)
		} else {
			break
		}
	}

	sort.Slice(done, func(i, j int) bool {
		return done[i][0] < done[j][0]
	})
	fmt.Printf("Part 1: %d\n", done[0][1])
	fmt.Printf("Part 2: %d\n", done[len(done)-1][1])

	return 0
}

type Board struct {
	numbers [][]int
	rows    []int
	cols    []int
}

func newBoard(input string) *Board {
	numbers := make([][]int, 5)
	rows := []int{5, 5, 5, 5, 5}
	cols := []int{5, 5, 5, 5, 5}
	for i, row := range strings.Split(input, "\n") {
		if row != "" {
			numbers[i] = util.ListOfInt(row)
		}
	}
	return &Board{numbers, rows, cols}
}

func (b *Board) Call(n int) bool {
	for y := 0; y < 5; y++ {
		for x := 0; x < 5; x++ {
			if b.numbers[y][x] == n {
				b.numbers[y][x] = -1
				b.rows[y]--
				b.cols[x]--
				if b.rows[y] == 0 || b.cols[x] == 0 {
					return true
				}
				return false
			}
		}
	}
	return false
}

func (b *Board) Sum() int {
	var sum = 0
	for y := 0; y < 5; y++ {
		for x := 0; x < 5; x++ {
			v := b.numbers[y][x]
			if v >= 0 {
				sum += v
			}
		}
	}
	return sum
}

func main() {
	solveDay4(util.Input())
}
