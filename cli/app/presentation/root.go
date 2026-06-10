// Package presentation defines the root orchustr command shell.
package presentation

import (
	"github.com/Regent33/Orchustr/cli/app/config"
	"github.com/spf13/cobra"
)

// NewRootCommand builds the bare root command; features are attached in di.
func NewRootCommand() *cobra.Command {
	root := &cobra.Command{
		Use:     config.BinaryName,
		Version: config.Version,
		Short:   "Orchustr — build, lint, run, and trace AI agent projects",
		Long: "Orchustr is a Rust-first AI orchestration framework.\n" +
			"This CLI scaffolds agent projects in Rust, Python, TypeScript, or Dart,\n" +
			"validates their graphs offline, and runs them with the right toolchain.",
		Example: "  orchustr init hermes-agent --lang rust\n" +
			"  orchustr lint .\n" +
			"  orchustr run .",
		SilenceUsage: true,
	}
	root.SetErrPrefix(config.BinaryName + ":")
	return root
}
