package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"

	"github.com/bpa/aoc-runner/language"
)

func main() {
	lang := flag.String("lang", "", "language to run (python,go,rust)")
	day := flag.Int("day", 0, "day to run (required)")
	flag.Parse()

	cwd, err := os.Getwd()
	if err != nil {
		fmt.Fprintf(os.Stderr, "error getting current working directory: %v\n", err)
		os.Exit(1)
	}
	dirName := filepath.Base(cwd)

	if len(dirName) != 4 || !strings.HasPrefix(dirName, "20") {
		fmt.Fprintf(os.Stderr, "error: directory must be a year starting with 20 (e.g., 2025)\n")
		fmt.Fprintf(os.Stderr, "current directory: %s\n", dirName)
		os.Exit(1)
	}

	year, err := strconv.Atoi(dirName)
	if err != nil {
		fmt.Fprintf(os.Stderr, "error: could not parse year from directory name: %v\n", err)
		os.Exit(1)
	}

	if len(os.Args) == 1 {
		fmt.Fprintf(os.Stderr, "Usage: %s <lang> <day>\n", os.Args[0])
		fmt.Fprintf(os.Stderr, "   or: %s -lang <language> [-day <day>]\n\n", os.Args[0])
		fmt.Fprintf(os.Stderr, "Running from year directory: %d\n\n", year)
		fmt.Println("Available languages:")
		for name := range language.Languages {
			fmt.Printf(" - %s\n", name)
		}
		return
	}

	args := flag.Args()
	if len(args) >= 1 && strings.TrimSpace(*lang) == "" {
		*lang = args[0]
	}
	if len(args) >= 2 {
		if d, err := strconv.Atoi(args[1]); err == nil {
			*day = d
		}
	}

	if strings.TrimSpace(*lang) == "" {
		fmt.Println("Available languages:")
		for name := range language.Languages {
			fmt.Printf(" - %s\n", name)
		}
		return
	}

	if *day <= 0 {
		fmt.Fprintf(os.Stderr, "error: day is required\n")
		fmt.Fprintf(os.Stderr, "Usage: %s <lang> <day>\n   or: %s -lang <language> -day <day>\n", os.Args[0], os.Args[0])
		os.Exit(2)
	}

	var chosen language.Language
	want := strings.ToLower(*lang)
	if r, ok := language.Languages[want]; ok {
		chosen = r
	}
	if chosen == nil {
		fmt.Fprintf(os.Stderr, "unknown language: %s\n", *lang)
		fmt.Println("Available languages:")
		for name := range language.Languages {
			fmt.Printf(" - %s\n", name)
		}
		return
	}

	if err := WriteDayTemplate(chosen, year, *day); err != nil {
		fmt.Fprintf(os.Stderr, "error preparing template: %v\n", err)
		os.Exit(1)
	}

	if err := FetchInput(year, *day); err != nil {
		fmt.Fprintf(os.Stderr, "error fetching input: %v\n", err)
		os.Exit(1)
	}

	StartWatcher(chosen, year, *day)
}
