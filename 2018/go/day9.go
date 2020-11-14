package main

import (
	"fmt"
	"time"
)

func main() {
	start := time.Now()
	// score := high_score(459, 71320)
	score := high_score(465, 71940)
	elapsed := time.Since(start)
	fmt.Println("Part 1", score)
	fmt.Println("Time: ", elapsed)

	start = time.Now()
	score = high_score(465, 7194000)
	elapsed = time.Since(start)
	fmt.Println("\nPart 2", score)
	fmt.Println("Time: ", elapsed)
}

type Marble struct {
	value       uint
	left, right *Marble
}

func NewMarble(value uint) *Marble {
	m := new(Marble)
	m.left = m
	m.right = m
	m.value = value
	return m
}

func high_score(players uint, last uint) uint {
	scores := make([]uint, players)
	current := NewMarble(0)
	for marble := uint(1); marble <= last; marble++ {
		if marble%23 == 0 {
			player := marble % players
			scores[player] += marble
			for i := 0; i < 7; i++ {
				current = current.left
			}
			scores[player] += current.value
			current.left.right = current.right
			current.right.left = current.left
			current = current.right
		} else {
			current = current.right
			new_marble := NewMarble(marble)
			new_marble.right = current.right
			current.right = new_marble
			new_marble.left = current
			new_marble.right.left = new_marble
			current = new_marble
		}
	}

	high := uint(0)
	for player := uint(0); player < players; player++ {
		if scores[player] > high {
			high = scores[player]
		}
	}
	return high
}
