//go:build windows
// +build windows

package main

const newline = "\r\n"

func clearScreen() {
	RunCommand("cmd", []string{"/c", "cls"}, []string{})
}
