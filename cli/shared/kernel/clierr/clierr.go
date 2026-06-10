// Package clierr defines the typed error kinds shared by every CLI feature.
// It mirrors the Rust or-cli CliError variants so behavior stays consistent
// across both implementations.
package clierr

import (
	"errors"
	"fmt"
)

// Kind classifies a CLI failure for exit handling and message prefixes.
type Kind string

const (
	// KindConfig marks an unreadable or invalid orchustr.yaml.
	KindConfig Kind = "config"
	// KindValidation marks a graph spec that parsed but is semantically invalid.
	KindValidation Kind = "validation"
	// KindProject marks a missing or malformed project directory.
	KindProject Kind = "project"
	// KindIO marks an underlying filesystem failure.
	KindIO Kind = "io"
	// KindToolchain marks a missing external program (cargo, npm, ...).
	KindToolchain Kind = "toolchain"
)

// Error is the single error type surfaced by CLI use cases.
type Error struct {
	Kind    Kind
	Message string
	Cause   error
}

func (e *Error) Error() string {
	if e.Cause != nil {
		return fmt.Sprintf("%s: %s: %v", e.Kind, e.Message, e.Cause)
	}
	return fmt.Sprintf("%s: %s", e.Kind, e.Message)
}

// Unwrap exposes the cause for errors.Is / errors.As chains.
func (e *Error) Unwrap() error { return e.Cause }

// New creates an Error without an underlying cause.
func New(kind Kind, format string, args ...any) *Error {
	return &Error{Kind: kind, Message: fmt.Sprintf(format, args...)}
}

// Wrap creates an Error that preserves the underlying cause.
func Wrap(kind Kind, cause error, format string, args ...any) *Error {
	return &Error{Kind: kind, Message: fmt.Sprintf(format, args...), Cause: cause}
}

// KindOf reports the Kind of err, or empty string for foreign errors.
func KindOf(err error) Kind {
	var typed *Error
	if errors.As(err, &typed) {
		return typed.Kind
	}
	return ""
}
