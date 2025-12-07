package main

import (
	"errors"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strconv"
)

func FetchInput(year int, day int) error {
	inputDir := filepath.Join("input", strconv.Itoa(year))
	if err := os.MkdirAll(inputDir, 0o755); err != nil {
		return err
	}

	testPath := filepath.Join(inputDir, fmt.Sprintf("day%d.test", day))
	if _, err := os.Stat(testPath); os.IsNotExist(err) {
		if f, err := os.Create(testPath); err == nil {
			f.Close()
		}
	}

	outPath := filepath.Join(inputDir, fmt.Sprintf("day%d.txt", day))
	if _, err := os.Stat(outPath); err == nil {
		return nil // already have input
	}

	sessionPath := filepath.Join("..", ".session")
	b, err := os.ReadFile(sessionPath)
	if err != nil {
		return errors.New(".session file not found in repository root; create it with your AoC session cookie")
	}
	session := string(b)

	url := fmt.Sprintf("https://adventofcode.com/%d/day/%d/input", year, day)
	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		return err
	}
	req.Header.Set("Cookie", fmt.Sprintf("session=%s", session))
	req.Header.Set("User-Agent", "aoc-runner (github.com/bpa/advent-of-code)")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()
	if resp.StatusCode != 200 {
		return fmt.Errorf("failed to fetch input: status %d", resp.StatusCode)
	}

	out, err := os.Create(outPath)
	if err != nil {
		return err
	}
	defer out.Close()

	if _, err := io.Copy(out, resp.Body); err != nil {
		return err
	}
	return nil
}
