// Package data executes launch plans as child processes with inherited stdio.
package data

import (
	"os"
	"os/exec"

	"github.com/Regent33/Orchustr/cli/features/runproject/domain"
	"github.com/Regent33/Orchustr/cli/shared/kernel/clierr"
)

// ProcessExecutor runs a LaunchPlan in a project directory.
type ProcessExecutor struct{}

// NewProcessExecutor constructs the default executor.
func NewProcessExecutor() *ProcessExecutor { return &ProcessExecutor{} }

// Execute runs the plan, streaming the child's stdio to the user, and maps
// launch and exit failures to typed CLI errors.
func (e *ProcessExecutor) Execute(projectDir string, plan *domain.LaunchPlan) error {
	cmd := exec.Command(plan.Program, plan.Args...)
	cmd.Dir = projectDir
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	if err := cmd.Run(); err != nil {
		if exit, ok := err.(*exec.ExitError); ok {
			return clierr.New(clierr.KindProject, "`%s` exited with status %d",
				plan.Program, exit.ExitCode())
		}
		return clierr.Wrap(clierr.KindToolchain, err,
			"failed to launch `%s` — is the toolchain installed and on PATH?", plan.Program)
	}
	return nil
}
