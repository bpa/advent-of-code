//go:build !windows
// +build !windows

package main

func clearScreen() {
	RunCommand("clear", []string{}, []string{})
}
