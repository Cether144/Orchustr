// Package fsutil holds the small filesystem helpers shared by CLI features.
package fsutil

import (
	"os"
	"path/filepath"

	"github.com/Regent33/Orchustr/cli/shared/kernel/clierr"
)

// WriteFile writes contents to path, creating parent directories as needed.
func WriteFile(path string, contents []byte) error {
	if parent := filepath.Dir(path); parent != "." {
		if err := os.MkdirAll(parent, 0o755); err != nil {
			return clierr.Wrap(clierr.KindIO, err, "cannot create directory %s", parent)
		}
	}
	if err := os.WriteFile(path, contents, 0o644); err != nil {
		return clierr.Wrap(clierr.KindIO, err, "cannot write %s", path)
	}
	return nil
}

// FileExists reports whether path exists and is a regular file.
func FileExists(path string) bool {
	info, err := os.Stat(path)
	return err == nil && info.Mode().IsRegular()
}
