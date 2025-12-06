package language

import (
	"fmt"
	"path/filepath"
)

type GoRunner struct{}

func (g *GoRunner) Cmd() string {
	return "go"
}

func (g *GoRunner) Ext() string {
	return "go"
}

func (g *GoRunner) TestArgs(day int, input string) []string {
	return []string{"run", fmt.Sprintf("day%d.go", day), "-tags=debug", input}
}

func (g *GoRunner) TestEnv() []string {
	return []string{}
}

func (g *GoRunner) RunArgs(day int, input string) []string {
	return []string{"run", fmt.Sprintf("day%d.go", day), input}
}

func (g *GoRunner) RunEnv() []string {
	return []string{}
}

func (g *GoRunner) WatchPaths(day int) []string {
	return []string{
		fmt.Sprintf("day%d.go", day),
		filepath.Join("..", "..", "lib", "go"),
	}
}

func (g *GoRunner) WorkDir() string {
	return "go"
}

func init() {
	Register("go", &GoRunner{})
}
