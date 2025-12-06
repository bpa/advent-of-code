package main

import (
	"fmt"
	"os"
	"os/exec"
	"os/signal"
	"path/filepath"
	"strings"
	"sync"
	"syscall"
	"time"

	"github.com/bpa/aoc-runner/language"
	"github.com/fsnotify/fsnotify"
)

func RunCommand(name string, args, env []string) error {
	cmd := exec.Command(name, args...)
	cmd.Env = env
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Stdin = os.Stdin
	return cmd.Run()
}

func StartWatcher(lang language.Language, year int, day int) {
	workDir := lang.WorkDir()
	if err := os.Chdir(workDir); err != nil {
		fmt.Fprintf(os.Stderr, "error changing to working directory %s: %v\n", workDir, err)
		os.Exit(1)
	}

	var paths []string
	realInput := filepath.Join("..", "input", fmt.Sprintf("day%d.txt", day))
	testInput := filepath.Join("..", "input", fmt.Sprintf("day%d.test", day))
	paths = append(paths, realInput, testInput)
	paths = append(paths, lang.WatchPaths(day)...)

	stop, err := StartWatch(paths, func() {
		clearScreen()

		if info, err := os.Stat(testInput); err == nil && info.Size() > 0 {
			RunCommand(lang.Cmd(), lang.TestArgs(day, testInput), lang.TestEnv())
			fmt.Println("-----")
		}
		RunCommand(lang.Cmd(), lang.RunArgs(day, realInput), lang.RunEnv())
	})
	if err != nil {
		fmt.Fprintf(os.Stderr, "watcher start error: %v\n", err)
		os.Exit(1)
	}

	// Block until interrupted (Ctrl-C)
	sig := make(chan os.Signal, 1)
	signal.Notify(sig, os.Interrupt, syscall.SIGTERM)
	<-sig
	stop()
}

// StartWatch watches the provided paths (files or directories) and calls onChange(useTest)
// after a short debounce when modifications are detected. If testInputPath is non-empty
// and its file size is >0 when the debounce fires, onChange is called with useTest=true.
// Returns a stop function to terminate the watcher.
func StartWatch(paths []string, onChange func()) (func(), error) {
	watcher, err := fsnotify.NewWatcher()
	if err != nil {
		return nil, err
	}

	var debounceMu sync.Mutex
	var debounceTimer *time.Timer

	resetDebounce := func() {
		debounceMu.Lock()
		defer debounceMu.Unlock()
		if debounceTimer != nil {
			debounceTimer.Stop()
		}
		debounceTimer = time.AfterFunc(600*time.Millisecond, func() {
			onChange()
		})
	}

	for _, p := range paths {
		if p == "" {
			continue
		}
		fmt.Println(p)
		if err := watcher.Add(p); err != nil {
			parent := filepath.Dir(p)
			if parent != "" {
				_ = watcher.Add(parent)
			}
		}
	}

	stop := make(chan struct{})

	go func() {
		defer watcher.Close()
		for {
			select {
			case ev, ok := <-watcher.Events:
				fmt.Printf("%v\n", ev)
				if !ok {
					return
				}
				if ev.Op&(fsnotify.Write|fsnotify.Create|fsnotify.Remove|fsnotify.Rename) == 0 {
					continue
				}
				if ignorePath(ev.Name) {
					continue
				}
				resetDebounce()
			case err, ok := <-watcher.Errors:
				if !ok {
					return
				}
				fmt.Println("watcher error:", err)
			case <-stop:
				return
			}
		}
	}()

	return func() { close(stop) }, nil
}

// ignorePath filters temporary files and hidden files
func ignorePath(p string) bool {
	base := filepath.Base(p)
	if strings.HasPrefix(base, ".") {
		return true
	}
	if strings.HasSuffix(base, "~") || strings.HasSuffix(base, ".swp") || strings.HasSuffix(base, ".tmp") {
		return true
	}
	return false
}
