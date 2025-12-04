//go:build !debug
// +build !debug

package aoc

func Debug(a ...any) (n int, err error) {
	return 0, nil
}
