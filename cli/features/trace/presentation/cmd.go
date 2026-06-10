// Package presentation exposes `orchustr trace` by delegating to the Rust
// orchustr binary, which hosts the or-lens dashboard in-process. The Go CLI
// deliberately does not reimplement the dashboard.
package presentation

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"

	"github.com/Regent33/Orchustr/cli/app/config"
	"github.com/Regent33/Orchustr/cli/shared/kernel/clierr"
	"github.com/spf13/cobra"
)

// NewCommand builds the trace command.
func NewCommand() *cobra.Command {
	return &cobra.Command{
		Use:   "trace [project-dir]",
		Short: "Open the local or-lens trace dashboard (requires the Rust CLI)",
		Long: "Start the local execution dashboard for a project. The dashboard is\n" +
			"hosted by the Rust orchustr binary; set " + config.RustBinaryEnv + " or install it\n" +
			"with `cargo install --path crates/or-cli` from the Orchustr repo.",
		Example: "  orchustr trace .\n  ORCHUSTR_RUST_BIN=~/bin/orchustr-rs orchustr trace .",
		Args:    cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			projectDir := "."
			if len(args) == 1 {
				projectDir = args[0]
			}
			binary, err := findRustBinary()
			if err != nil {
				return err
			}
			fmt.Fprintf(cmd.OutOrStdout(), "delegating to %s\n", binary)
			child := exec.Command(binary, "trace", projectDir)
			child.Stdin = os.Stdin
			child.Stdout = os.Stdout
			child.Stderr = os.Stderr
			return child.Run()
		},
	}
}

// findRustBinary locates the Rust orchustr binary, refusing to return the
// currently running Go executable (both install under the same name).
func findRustBinary() (string, error) {
	if override := os.Getenv(config.RustBinaryEnv); override != "" {
		return override, nil
	}
	self, _ := os.Executable()
	selfReal, _ := filepath.EvalSymlinks(self)
	candidate, err := exec.LookPath("orchustr")
	if err == nil {
		candidateReal, _ := filepath.EvalSymlinks(candidate)
		if candidateReal != selfReal && candidateReal != "" {
			return candidate, nil
		}
	}
	return "", clierr.New(clierr.KindToolchain,
		"the trace dashboard needs the Rust orchustr binary; install it with\n"+
			"  cargo install --path crates/or-cli   (from %s)\n"+
			"or point %s at it", config.RepoURL, config.RustBinaryEnv)
}
