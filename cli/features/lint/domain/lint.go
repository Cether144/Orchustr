// Package domain implements graph validation for `orchustr lint`, mirroring
// the Rust or-cli lint_path behavior: lint a single file, a project (via
// orchustr.yaml's graph $ref), or every YAML graph in a directory.
package domain

import (
	"os"
	"path/filepath"
	"strings"

	"github.com/Regent33/Orchustr/cli/shared/kernel/clierr"
	"github.com/Regent33/Orchustr/cli/shared/kernel/graphspec"
	"github.com/Regent33/Orchustr/cli/shared/kernel/projectcfg"
)

// LintUseCase validates graph descriptor files.
type LintUseCase struct{}

// NewLintUseCase constructs the use case.
func NewLintUseCase() *LintUseCase { return &LintUseCase{} }

// Execute returns every validated graph path, or the first failure.
func (uc *LintUseCase) Execute(path string) ([]string, error) {
	info, err := os.Stat(path)
	if err != nil {
		return nil, clierr.Wrap(clierr.KindProject, err, "cannot access %s", path)
	}
	if info.Mode().IsRegular() {
		return uc.lintFile(path)
	}
	if _, err := os.Stat(filepath.Join(path, projectcfg.FileName)); err == nil {
		cfg, err := projectcfg.Load(path)
		if err != nil {
			return nil, err
		}
		return uc.lintFile(cfg.GraphPath(path))
	}
	return uc.lintDirectory(path)
}

func (uc *LintUseCase) lintFile(path string) ([]string, error) {
	raw, err := os.ReadFile(path)
	if err != nil {
		return nil, clierr.Wrap(clierr.KindIO, err, "cannot read graph %s", path)
	}
	isJSON := strings.EqualFold(filepath.Ext(path), ".json")
	spec, err := graphspec.Parse(raw, isJSON)
	if err != nil {
		return nil, clierr.Wrap(clierr.KindValidation, err, "%s", path)
	}
	if err := spec.Validate(); err != nil {
		return nil, clierr.Wrap(clierr.KindValidation, err, "%s", path)
	}
	return []string{path}, nil
}

func (uc *LintUseCase) lintDirectory(dir string) ([]string, error) {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return nil, clierr.Wrap(clierr.KindIO, err, "cannot read directory %s", dir)
	}
	var validated []string
	for _, entry := range entries {
		ext := strings.ToLower(filepath.Ext(entry.Name()))
		if entry.Type().IsRegular() && (ext == ".yaml" || ext == ".yml") {
			paths, err := uc.lintFile(filepath.Join(dir, entry.Name()))
			if err != nil {
				return nil, err
			}
			validated = append(validated, paths...)
		}
	}
	if len(validated) == 0 {
		return nil, clierr.New(clierr.KindProject, "no graph yaml files found in %s", dir)
	}
	return validated, nil
}
