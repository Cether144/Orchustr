// Command orchustr is the official Orchustr CLI: scaffold, lint, run, and
// trace Orchustr agent projects in Rust, Python, TypeScript, or Dart.
package main

import (
	"os"

	"github.com/Regent33/Orchustr/cli/app/di"
)

func main() {
	if err := di.NewRootCommand().Execute(); err != nil {
		// Cobra already prints the error with the configured prefix.
		os.Exit(1)
	}
}
