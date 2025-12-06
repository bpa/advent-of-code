package language

// import (
// 	"fmt"
// 	"os"
// 	"path/filepath"
// 	"strconv"
// )

// type RustRunner struct {
// 	year int
// 	day  int
// }

// func (r *RustRunner) WorkDir(year int) string {
// 	// Prefer current directory if it has Cargo.toml (user is already in year dir)
// 	if _, err := os.Stat("Cargo.toml"); err == nil {
// 		return "."
// 	}
// 	return filepath.Join("..", strconv.Itoa(year))
// }

// // Run tries common Cargo layouts for a given year and day:
// // - If src/bin/dayN.rs exists, run `cargo run --bin dayN --release -- input/dayN.txt`
// // - If src/dayN.rs exists, run `cargo run --release -- input/dayN.txt`
// // - Otherwise, attempt a generic `cargo run --release -- input/dayN.txt`
// func (r *RustRunner) Run(year int, day int, inputPath string) error {
// 	if inputPath == "" {
// 		inputPath = filepath.Join("input", fmt.Sprintf("day%d.txt", day))
// 	}

// 	// Check if test file exists and has content
// 	testPath := filepath.Join("input", fmt.Sprintf("day%d.test", day))
// 	if info, err := os.Stat(testPath); err == nil && info.Size() > 0 {
// 		// Attempt 1: src/bin/dayN.rs -> cargo run --bin dayN --release -- test input
// 		cmd := RunCommand("cargo", "run", "--bin", fmt.Sprintf("day%d", day), "--release", "--", testPath)
// 		if err := runCmd(cmd); err == nil {
// 			goto normalRun
// 		}

// 		// Attempt 2: src/dayN.rs -> cargo run --release -- test input
// 		cmd = RunCommand("cargo", "run", "--release", "--", testPath)
// 		if err := runCmd(cmd); err == nil {
// 			goto normalRun
// 		}
// 	}

// normalRun:
// 	// Attempt 1: src/bin/dayN.rs -> cargo run --bin dayN --release -- input/dayN.txt
// 	cmd := RunCommand("cargo", "run", "--bin", fmt.Sprintf("day%d", day), "--release", "--", inputPath)
// 	if err := runCmd(cmd); err == nil {
// 		return nil
// 	}

// 	// Attempt 2: src/dayN.rs -> cargo run --release -- input/dayN.txt
// 	cmd = RunCommand("cargo", "run", "--release", "--", inputPath)
// 	return runCmd(cmd)
// }

// func init() {
// 	Register("rust", func(year int, day int) Language {
// 		return &RustRunner{year: year, day: day}
// 	})
// }

// var _ Language = (*RustRunner)(nil)
