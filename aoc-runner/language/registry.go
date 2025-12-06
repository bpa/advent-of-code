package language

type Language interface {
	Cmd() string
	Ext() string
	TestArgs(day int, input string) []string
	TestEnv() []string
	RunArgs(day int, input string) []string
	RunEnv() []string
	WatchPaths(day int) []string
	WorkDir() string
}

var Languages = map[string]Language{}

func Register(name string, r Language) {
	Languages[name] = r
}
