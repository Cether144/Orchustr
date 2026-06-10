// Package presentation exposes `orchustr run` as a cobra command.
package presentation

import (
	"fmt"

	"github.com/Regent33/Orchustr/cli/features/runproject/domain"
	"github.com/Regent33/Orchustr/cli/shared/fsutil"
	"github.com/Regent33/Orchustr/cli/shared/kernel/projectcfg"
	"github.com/spf13/cobra"
)

// Executor is the data-layer port that runs a launch plan.
type Executor interface {
	Execute(projectDir string, plan *domain.LaunchPlan) error
}

// NewCommand builds the run command around an injected executor.
func NewCommand(executor Executor) *cobra.Command {
	return &cobra.Command{
		Use:   "run [project-dir]",
		Short: "Run an Orchustr project with its language toolchain",
		Long: "Read orchustr.yaml, detect the project language, and launch the\n" +
			"canonical entrypoint (cargo run / python main.py / npm start / dart run).",
		Example: "  orchustr run\n  orchustr run ./hermes-agent",
		Args:    cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			projectDir := "."
			if len(args) == 1 {
				projectDir = args[0]
			}
			cfg, err := projectcfg.Load(projectDir)
			if err != nil {
				return err
			}
			plan, err := domain.BuildLaunchPlan(projectDir, cfg, fsutil.FileExists)
			if err != nil {
				return err
			}
			fmt.Fprintf(cmd.OutOrStdout(), "running %s (%s)\n", cfg.Project.Name, cfg.Project.Language)
			return executor.Execute(projectDir, plan)
		},
	}
}
