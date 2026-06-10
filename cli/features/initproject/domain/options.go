// Package domain holds the pure business rules for `orchustr init`.
package domain

import (
	"regexp"

	"github.com/Regent33/Orchustr/cli/shared/kernel/clierr"
)

// Language is a scaffold target language.
type Language string

// Topology is a starter agent loop shape.
type Topology string

// Provider is an LLM provider preset.
type Provider string

// Supported enum values, kept identical to the Rust or-cli flags.
const (
	LangRust       Language = "rust"
	LangPython     Language = "python"
	LangTypescript Language = "typescript"
	LangDart       Language = "dart"

	TopologyReact       Topology = "react"
	TopologyPlanExecute Topology = "plan-execute"
	TopologyReflection  Topology = "reflection"

	ProviderAnthropic Provider = "anthropic"
	ProviderOpenai    Provider = "openai"
	ProviderOllama    Provider = "ollama"
)

// Languages lists every supported language in flag-help order.
func Languages() []Language { return []Language{LangRust, LangPython, LangTypescript, LangDart} }

// Topologies lists every supported topology in flag-help order.
func Topologies() []Topology {
	return []Topology{TopologyReact, TopologyPlanExecute, TopologyReflection}
}

// Providers lists every supported provider in flag-help order.
func Providers() []Provider { return []Provider{ProviderAnthropic, ProviderOpenai, ProviderOllama} }

// EnvKey returns the .env.example key for the provider.
func (p Provider) EnvKey() string {
	switch p {
	case ProviderOpenai:
		return "OPENAI_API_KEY"
	case ProviderOllama:
		return "OLLAMA_BASE_URL"
	default:
		return "ANTHROPIC_API_KEY"
	}
}

// FileExtension returns the node source extension for the language.
func (l Language) FileExtension() string {
	switch l {
	case LangPython:
		return "py"
	case LangTypescript:
		return "ts"
	case LangDart:
		return "dart"
	default:
		return "rs"
	}
}

// Options carries the validated inputs for one init run.
type Options struct {
	ProjectName string
	Language    Language
	Topology    Topology
	Provider    Provider
	TargetDir   string
}

// projectNamePattern keeps generated names safe for directories, Cargo
// package names, npm names, and Dart packages alike.
var projectNamePattern = regexp.MustCompile(`^[a-z][a-z0-9_-]{0,63}$`)

// Validate rejects unsafe or unsupported option combinations before any
// filesystem writes happen.
func (o *Options) Validate() error {
	if !projectNamePattern.MatchString(o.ProjectName) {
		return clierr.New(clierr.KindValidation,
			"project name %q must start with a lowercase letter and use only [a-z0-9_-] (max 64 chars)",
			o.ProjectName)
	}
	if !contains(Languages(), o.Language) {
		return clierr.New(clierr.KindValidation, "unsupported language %q", o.Language)
	}
	if !contains(Topologies(), o.Topology) {
		return clierr.New(clierr.KindValidation, "unsupported topology %q", o.Topology)
	}
	if !contains(Providers(), o.Provider) {
		return clierr.New(clierr.KindValidation, "unsupported provider %q", o.Provider)
	}
	return nil
}

func contains[T comparable](haystack []T, needle T) bool {
	for _, candidate := range haystack {
		if candidate == needle {
			return true
		}
	}
	return false
}
