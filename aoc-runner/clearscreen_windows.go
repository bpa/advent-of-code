//go:build windows
// +build windows

package main

func clearScreen() {
	RunCommand("cmd", []string{"/c", "cls"}, []string{})
}
