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

	"github.com/fsnotify/fsnotify"
)

func RunCommand(name string, args, env []string) error {
	fmt.Printf("%s %v\n", name, args)
	cmd := exec.Command(name, args...)
	cmd.Env = env
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Stdin = os.Stdin
	return cmd.Run()
}

func StartWatcher(lang Language, year int, day int) {
	if err := os.Chdir(lang.WorkDir); err != nil {
		fmt.Fprintf(os.Stderr, "error changing to working directory %s: %v\n", lang.WorkDir, err)
		os.Exit(1)
	}

	var paths []string
	paths = append(paths, lang.TestFile, lang.RunFile)
	paths = append(paths, lang.WatchPaths...)

	stop, err := StartWatch(paths, func() {
		clearScreen()

		if info, err := os.Stat(lang.TestFile); err == nil && info.Size() > 0 {
			RunCommand(lang.Cmd, lang.TestArgs, lang.TestEnv)
			fmt.Println("-----")
		}
		RunCommand(lang.Cmd, lang.RunArgs, lang.RunEnv)
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

// StartWatch watches the provided paths (files or directories) and calls onChange()
// immediately when modifications are detected. After onChange() fires, a quiet period
// of 600ms begins where subsequent changes are ignored. Returns a stop function to
// terminate the watcher.
func StartWatch(paths []string, onChange func()) (func(), error) {
	watcher, err := fsnotify.NewWatcher()
	if err != nil {
		return nil, err
	}

	var debounceMu sync.Mutex
	var quietUntil time.Time

	shouldIgnore := func() bool {
		debounceMu.Lock()
		defer debounceMu.Unlock()
		return time.Now().Before(quietUntil)
	}

	markQuiet := func() {
		debounceMu.Lock()
		defer debounceMu.Unlock()
		quietUntil = time.Now().Add(600 * time.Millisecond)
	}

	for _, p := range paths {
		if p == "" {
			continue
		}
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
				if !ok {
					return
				}
				if ev.Op&(fsnotify.Write|fsnotify.Create|fsnotify.Remove|fsnotify.Rename) == 0 {
					continue
				}
				if ignorePath(ev.Name) {
					continue
				}
				// Fire immediately if not in quiet period
				if !shouldIgnore() {
					onChange()
					markQuiet()
				}
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
