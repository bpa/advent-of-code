//go:build !windows
// +build !windows

package main

const newline = "\n"

func clearScreen() {
	RunCommand("clear", []string{}, []string{})
}
