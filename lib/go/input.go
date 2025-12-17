package aoc

import (
	"fmt"
	"io"
	"os"
	"path/filepath"
	"regexp"
	"runtime"
	"strconv"
	"strings"
)

func ToInt(a string) int {
	if v, err := strconv.Atoi(strings.TrimSpace(a)); err == nil {
		return v
	}
	return 0
}

func Input() string {
	var filename string
	if len(os.Args) > 1 {
		filename = os.Args[1]
	} else {
		var exe string
		exe, err := os.Executable()
		if err != nil || strings.HasPrefix(exe, "/tmp") {
			exe, err = os.Getwd()
			if err != nil {
				fmt.Println("All attempts to detect input failed")
				os.Exit(1)
			}
		}

		_, caller, _, ok := runtime.Caller(1)
		if !ok {
			fmt.Println("Couldn't figure out day, try specifying the input file")
			os.Exit(1)
		}
		base := strings.TrimSuffix(caller, filepath.Ext(caller))
		base = filepath.Base(base)
		re := regexp.MustCompile(`^.*/(\d{4})/`)
		parts := re.FindStringSubmatch(exe)
		filename = parts[0] + "input/" + parts[1] + "/" + base + ".txt"
	}

	if filename == "-" {
		file, err := io.ReadAll(os.Stdin)
		if err != nil {
			fmt.Printf("Error reading from stdin: %s\n", err.Error())
			os.Exit(1)
		}
		return string(file)
	}

	file, err := os.ReadFile(filename)
	if err != nil {
		fmt.Printf("Could not open `%s`: %s\n", filename, err.Error())
		os.Exit(1)
	}

	return string(file)
}
