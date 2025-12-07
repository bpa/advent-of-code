package main

import (
	"fmt"
	"os"
	"path/filepath"
	"text/template"
)

func WriteDayTemplate(lang Language, year int, day int) error {
	tmplDir := filepath.Join("..", "templates")
	tmplFile := filepath.Join(tmplDir, fmt.Sprintf("template.%s", lang.Ext))
	if _, err := os.Stat(tmplFile); os.IsNotExist(err) {
		return fmt.Errorf("template not found: %s", tmplFile)
	}

	if err := os.MkdirAll(lang.WorkDir, 0o755); err != nil {
		return err
	}
	dest := filepath.Join(lang.WorkDir, fmt.Sprintf("day%d.%s", day, lang.Ext))
	if _, err := os.Stat(dest); err == nil {
		return nil // already exists
	}

	b, err := os.ReadFile(tmplFile)
	if err != nil {
		return err
	}

	t, err := template.New("tmpl").Parse(string(b))
	if err != nil {
		return err
	}

	out, err := os.OpenFile(dest, os.O_CREATE|os.O_WRONLY, 0o644)
	if err != nil {
		return err
	}
	defer out.Close()

	data := struct {
		Day  int
		Year int
	}{Day: day, Year: year}
	if err := t.Execute(out, data); err != nil {
		return err
	}
	return nil
}
