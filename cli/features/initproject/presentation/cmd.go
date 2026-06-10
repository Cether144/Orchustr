// Package presentation exposes `orchustr init` as a cobra command.
package presentation

import (
	"fmt"
	"os"

	"github.com/Regent33/Orchustr/cli/features/initproject/domain"
	"github.com/spf13/cobra"
)

// NewCommand builds the init command around an injected use case.
func NewCommand(useCase *domain.InitUseCase) *cobra.Command {
	var lang, topology, provider string
	cmd := &cobra.Command{
		Use:   "init <project-name>",
		Short: "Create a new Orchustr agent project",
		Long: "Scaffold a ready-to-run Orchustr agent project with orchustr.yaml,\n" +
			"a starter graph, and language-specific sources.",
		Example: "  orchustr init hermes-agent --lang rust\n" +
			"  orchustr init hermes-agent --lang typescript --topology plan-execute",
		Args: cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			cwd, err := os.Getwd()
			if err != nil {
				return err
			}
			root, err := useCase.Execute(&domain.Options{
				ProjectName: args[0],
				Language:    domain.Language(lang),
				Topology:    domain.Topology(topology),
				Provider:    domain.Provider(provider),
				TargetDir:   cwd,
			})
			if err != nil {
				return err
			}
			fmt.Fprintf(cmd.OutOrStdout(),
				"created %s\n\nnext steps:\n  cd %s\n  orchustr lint .\n  orchustr run .\n",
				root, args[0])
			return nil
		},
	}
	cmd.Flags().StringVarP(&lang, "lang", "l", string(domain.LangRust),
		fmt.Sprintf("project language %v", domain.Languages()))
	cmd.Flags().StringVarP(&topology, "topology", "t", string(domain.TopologyReact),
		fmt.Sprintf("starter agent loop %v", domain.Topologies()))
	cmd.Flags().StringVarP(&provider, "provider", "p", string(domain.ProviderAnthropic),
		fmt.Sprintf("LLM provider preset %v", domain.Providers()))
	return cmd
}
