// Package presentation exposes `orchustr lint` as a cobra command.
package presentation

import (
	"fmt"

	"github.com/Regent33/Orchustr/cli/features/lint/domain"
	"github.com/spf13/cobra"
)

// NewCommand builds the lint command around an injected use case.
func NewCommand(useCase *domain.LintUseCase) *cobra.Command {
	return &cobra.Command{
		Use:   "lint [path]",
		Short: "Validate graph files offline (defaults to the current project)",
		Long: "Validate an Orchustr project's graph, a single graph file, or every\n" +
			"YAML graph in a directory. No network or API keys required.",
		Example: "  orchustr lint .\n  orchustr lint graph.yaml\n  orchustr lint ./examples",
		Args:    cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			path := "."
			if len(args) == 1 {
				path = args[0]
			}
			validated, err := useCase.Execute(path)
			if err != nil {
				return err
			}
			for _, graph := range validated {
				fmt.Fprintf(cmd.OutOrStdout(), "ok: %s\n", graph)
			}
			return nil
		},
	}
}
