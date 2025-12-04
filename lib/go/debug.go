//go:build debug
// +build debug

package aoc

import "fmt"

func Debug(f string, a ...any) (n int, err error) {
	return fmt.Printf(f, a[:]...)
}
