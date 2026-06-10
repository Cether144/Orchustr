// Package di is the global composition root: every dependency is constructed
// here and injected via constructors, never resolved from inside a feature.
package di

import (
	"os/exec"

	apppresentation "github.com/Regent33/Orchustr/cli/app/presentation"
	doctordomain "github.com/Regent33/Orchustr/cli/features/doctor/domain"
	doctorpresentation "github.com/Regent33/Orchustr/cli/features/doctor/presentation"
	initdata "github.com/Regent33/Orchustr/cli/features/initproject/data"
	initdomain "github.com/Regent33/Orchustr/cli/features/initproject/domain"
	initpresentation "github.com/Regent33/Orchustr/cli/features/initproject/presentation"
	lintdomain "github.com/Regent33/Orchustr/cli/features/lint/domain"
	lintpresentation "github.com/Regent33/Orchustr/cli/features/lint/presentation"
	rundata "github.com/Regent33/Orchustr/cli/features/runproject/data"
	runpresentation "github.com/Regent33/Orchustr/cli/features/runproject/presentation"
	scaffolddomain "github.com/Regent33/Orchustr/cli/features/scaffold/domain"
	scaffoldpresentation "github.com/Regent33/Orchustr/cli/features/scaffold/presentation"
	tracepresentation "github.com/Regent33/Orchustr/cli/features/trace/presentation"
	"github.com/Regent33/Orchustr/cli/shared/fsutil"
	"github.com/spf13/cobra"
)

// diskWriter adapts fsutil to the FileWriter ports of init and scaffold.
type diskWriter struct{}

func (diskWriter) Write(path string, contents []byte) error {
	return fsutil.WriteFile(path, contents)
}

// NewRootCommand wires every feature and returns the executable root command.
func NewRootCommand() *cobra.Command {
	writer := diskWriter{}
	root := apppresentation.NewRootCommand()
	root.AddCommand(
		initpresentation.NewCommand(initdomain.NewInitUseCase(initdata.NewEmbeddedRenderer(), writer)),
		scaffoldpresentation.NewCommand(scaffolddomain.NewScaffoldUseCase(writer)),
		lintpresentation.NewCommand(lintdomain.NewLintUseCase()),
		runpresentation.NewCommand(rundata.NewProcessExecutor()),
		tracepresentation.NewCommand(),
		doctorpresentation.NewCommand(doctordomain.NewDoctorUseCase(exec.LookPath)),
	)
	return root
}
