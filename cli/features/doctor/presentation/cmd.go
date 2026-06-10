// Package presentation exposes `orchustr doctor` as a cobra command.
package presentation

import (
	"fmt"

	"github.com/Regent33/Orchustr/cli/features/doctor/domain"
	"github.com/spf13/cobra"
)

// NewCommand builds the doctor command around an injected use case.
func NewCommand(useCase *domain.DoctorUseCase) *cobra.Command {
	return &cobra.Command{
		Use:     "doctor",
		Short:   "Check that your environment is ready for Orchustr projects",
		Example: "  orchustr doctor",
		Args:    cobra.NoArgs,
		RunE: func(cmd *cobra.Command, args []string) error {
			failures := 0
			for _, result := range useCase.Execute(".") {
				mark := "ok"
				if !result.OK {
					mark = "!!"
					failures++
				}
				fmt.Fprintf(cmd.OutOrStdout(), "[%s] %-14s %s\n", mark, result.Name, result.Detail)
			}
			if failures > 0 {
				fmt.Fprintf(cmd.OutOrStdout(),
					"\n%d check(s) failed — only the languages you use need their toolchain.\n", failures)
			}
			return nil
		},
	}
}
