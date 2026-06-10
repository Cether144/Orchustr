// Package domain implements the environment checks behind `orchustr doctor`.
package domain

import (
	"os"
	"path/filepath"

	"github.com/Regent33/Orchustr/cli/shared/kernel/projectcfg"
)

// LookPath is the PATH probe injected from the data layer (exec.LookPath).
type LookPath func(program string) (string, error)

// CheckResult is one doctor finding.
type CheckResult struct {
	Name   string
	OK     bool
	Detail string
}

// DoctorUseCase inspects the local toolchain and the current project.
type DoctorUseCase struct {
	lookPath LookPath
}

// NewDoctorUseCase wires the use case with its PATH probe.
func NewDoctorUseCase(lookPath LookPath) *DoctorUseCase {
	return &DoctorUseCase{lookPath: lookPath}
}

// toolchains lists each probed program and the language it unlocks.
var toolchains = []struct{ program, purpose string }{
	{"cargo", "Rust projects (orchustr run, --lang rust)"},
	{"python", "Python projects (--lang python)"},
	{"npm", "TypeScript projects (--lang typescript)"},
	{"dart", "Dart projects (--lang dart)"},
	{"git", "git-based template dependencies"},
}

// Execute probes toolchains and, when dir holds an Orchustr project, its config.
func (uc *DoctorUseCase) Execute(dir string) []CheckResult {
	results := make([]CheckResult, 0, len(toolchains)+1)
	for _, tool := range toolchains {
		path, err := uc.lookPath(tool.program)
		if err != nil {
			results = append(results, CheckResult{tool.program, false, "not on PATH — needed for " + tool.purpose})
			continue
		}
		results = append(results, CheckResult{tool.program, true, path})
	}
	results = append(results, uc.checkProject(dir))
	return results
}

func (uc *DoctorUseCase) checkProject(dir string) CheckResult {
	if _, err := os.Stat(filepath.Join(dir, projectcfg.FileName)); err != nil {
		return CheckResult{projectcfg.FileName, true, "not in a project directory (skipped)"}
	}
	cfg, err := projectcfg.Load(dir)
	if err != nil {
		return CheckResult{projectcfg.FileName, false, err.Error()}
	}
	return CheckResult{projectcfg.FileName, true,
		"project '" + cfg.Project.Name + "' (" + cfg.Project.Language + ")"}
}
