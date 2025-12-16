package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strconv"
)

type Language struct {
	Cmd        string
	Ext        string
	TestArgs   []string
	TestEnv    []string
	TestFile   string
	RunArgs    []string
	RunEnv     []string
	RunFile    string
	WatchPaths []string
	WorkDir    string
}

var Languages = map[string]func(int, int) Language{
	"go":     GoLang,
	"python": Python,
	"rust":   Rust,
}

func GoLang(year, day int) Language {
	yy := strconv.Itoa(year)
	testFile := filepath.Join("..", "input", yy, fmt.Sprintf("day%d.test", day))
	runFile := filepath.Join("..", "input", yy, fmt.Sprintf("day%d.txt", day))
	return Language{
		Cmd:      "go",
		Ext:      "go",
		TestArgs: []string{"run", "-tags=debug", fmt.Sprintf("day%d.go", day), testFile},
		TestEnv: []string{
			fmt.Sprintf("GOPATH=%s", os.Getenv("GOPATH")),
			fmt.Sprintf("TMP=%s", os.Getenv("TMP")),
			fmt.Sprintf("TEMP=%s", os.Getenv("TEMP")),
			fmt.Sprintf("TERM=%s", os.Getenv("TERM")),
			fmt.Sprintf("LOCALAPPDATA=%s", os.Getenv("LOCALAPPDATA")),
		},
		TestFile: testFile,
		RunArgs:  []string{"run", fmt.Sprintf("day%d.go", day), runFile},
		RunEnv: []string{
			fmt.Sprintf("GOPATH=%s", os.Getenv("GOPATH")),
			fmt.Sprintf("TMP=%s", os.Getenv("TMP")),
			fmt.Sprintf("TEMP=%s", os.Getenv("TEMP")),
			fmt.Sprintf("TERM=%s", os.Getenv("TERM")),
			fmt.Sprintf("LOCALAPPDATA=%s", os.Getenv("LOCALAPPDATA")),
		},
		RunFile: runFile,
		WatchPaths: []string{
			fmt.Sprintf("day%d.go", day),
			filepath.Join("..", "..", "lib", "go"),
		},
		WorkDir: "go",
	}
}

func Python(year, day int) Language {
	yy := strconv.Itoa(year)
	testFile := filepath.Join("..", "input", yy, fmt.Sprintf("day%d.test", day))
	runFile := filepath.Join("..", "input", yy, fmt.Sprintf("day%d.txt", day))
	return Language{
		Cmd:      "python3",
		Ext:      "py",
		TestArgs: []string{fmt.Sprintf("day%d.py", day), testFile},
		TestEnv: []string{
			"PYTHONPATH=../../lib/python", "DEBUG=true",
			fmt.Sprintf("TERM=%s", os.Getenv("TERM")),
		},
		TestFile: testFile,
		RunArgs:  []string{fmt.Sprintf("day%d.py", day), runFile},
		RunEnv: []string{
			"PYTHONPATH=../../lib/python",
			fmt.Sprintf("TERM=%s", os.Getenv("TERM")),
		},
		RunFile: runFile,
		WatchPaths: []string{
			fmt.Sprintf("day%d.py", day),
			filepath.Join("..", "..", "lib", "python", "aoc"),
		},
		WorkDir: "python",
	}
}

func Rust(year, day int) Language {
	yy := strconv.Itoa(year)
	testFile := filepath.Join("input", yy, fmt.Sprintf("day%d.test", day))
	runFile := filepath.Join("input", yy, fmt.Sprintf("day%d.txt", day))
	return Language{
		Cmd:      "cargo",
		Ext:      "rs",
		TestArgs: []string{"aoc", "-d", strconv.Itoa(day), "-input", filepath.Join("input", yy, fmt.Sprintf("day%d.test", day))},
		TestEnv: []string{
			fmt.Sprintf("TERM=%s", os.Getenv("TERM")),
		},
		TestFile: testFile,
		RunArgs:  []string{"aoc", "-d", strconv.Itoa(day), "-input", filepath.Join("input", yy, fmt.Sprintf("day%d.txt", day))},
		RunEnv: []string{
			fmt.Sprintf("TERM=%s", os.Getenv("TERM")),
		},
		RunFile: runFile,
		WatchPaths: []string{
			".",
			filepath.Join("..", "lib", "rust"),
		},
		WorkDir: ".",
	}
}
