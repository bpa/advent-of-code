package language

import (
	"fmt"
	"path/filepath"
)

type PythonRunner struct{}

func (g *PythonRunner) Cmd() string {
	return "python3"
}

func (g *PythonRunner) Ext() string {
	return "py"
}

func (g *PythonRunner) TestArgs(day int, input string) []string {
	return g.RunArgs(day, input)
}

func (g *PythonRunner) TestEnv() []string {
	return []string{"PYTHONPATH=../../lib/python", "DEBUG=true"}
}

func (g *PythonRunner) RunArgs(day int, input string) []string {
	return []string{fmt.Sprintf("day%d.py", day), input}
}

func (g *PythonRunner) RunEnv() []string {
	return []string{"PYTHONPATH=../../lib/python"}
}

func (g *PythonRunner) WatchPaths(day int) []string {
	return []string{
		fmt.Sprintf("day%d.py", day),
		filepath.Join("..", "..", "lib", "python"),
	}
}

func (g *PythonRunner) WorkDir() string {
	return "python"
}

func init() {
	Register("python", &PythonRunner{})
}
