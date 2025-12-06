# aoc-runner

Small runner to execute Advent of Code solutions in multiple languages from the repository root.

## Usage

```powershell
cd aoc-runner
go run . -lang python -year 2015 -day 1
```

Or build a binary:

```powershell
go build -o aoc-runner.exe .
.\aoc-runner.exe -lang go -year 2017 -day 15
```

## How it works

- `python` runner expects scripts at `../<year>/python/day<N>.py` relative to `aoc-runner/`.
- `go` runner expects Go source at `../<year>/go/day<N>.go` and uses `go run`.
- `rust` runner is a stub; many Rust projects have differing workspace layouts — implement per-year handling as needed.

The runner automatically:
1. Writes a template for the day if it doesn't exist
2. Fetches the puzzle input (requires `.session` cookie file at repo root)
3. Runs the solution for the specified language and day
