// Package presentation exposes `orchustr new` as a cobra command group.
package presentation

import (
	"fmt"

	"github.com/Regent33/Orchustr/cli/features/scaffold/domain"
	"github.com/spf13/cobra"
)

// NewCommand builds the `new` command group around an injected use case.
func NewCommand(useCase *domain.ScaffoldUseCase) *cobra.Command {
	cmd := &cobra.Command{
		Use:   "new",
		Short: "Add a node or topology to an existing project",
	}
	cmd.AddCommand(newNodeCommand(useCase), newTopologyCommand(useCase))
	return cmd
}

func newNodeCommand(useCase *domain.ScaffoldUseCase) *cobra.Command {
	var projectDir string
	cmd := &cobra.Command{
		Use:     "node <name>",
		Short:   "Scaffold a node handler stub in the project's language",
		Example: "  orchustr new node summarize\n  orchustr new node act --project ./hermes-agent",
		Args:    cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			path, err := useCase.Node(projectDir, args[0])
			if err != nil {
				return err
			}
			fmt.Fprintf(cmd.OutOrStdout(), "created %s\n", path)
			return nil
		},
	}
	cmd.Flags().StringVarP(&projectDir, "project", "C", ".", "project directory")
	return cmd
}

func newTopologyCommand(useCase *domain.ScaffoldUseCase) *cobra.Command {
	var projectDir string
	cmd := &cobra.Command{
		Use:     "topology <name>",
		Short:   "Scaffold a custom Rust LoopTopology stub",
		Example: "  orchustr new topology debate",
		Args:    cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			path, err := useCase.Topology(projectDir, args[0])
			if err != nil {
				return err
			}
			fmt.Fprintf(cmd.OutOrStdout(), "created %s\n", path)
			return nil
		},
	}
	cmd.Flags().StringVarP(&projectDir, "project", "C", ".", "project directory")
	return cmd
}
