// Package projectcfg loads the root orchustr.yaml project file, mirroring the
// OrchustrConfig schema parsed by the Rust or-cli crate.
package projectcfg

import (
	"os"
	"path/filepath"
	"strings"

	"github.com/Regent33/Orchustr/cli/shared/kernel/clierr"
	"gopkg.in/yaml.v3"
)

// FileName is the canonical project config file name.
const FileName = "orchustr.yaml"

// Config is the root orchustr.yaml document.
type Config struct {
	OrchustrVersion string         `yaml:"orchustr_version"`
	Project         Metadata       `yaml:"project"`
	Graph           GraphRef       `yaml:"graph"`
	Observability   *Observability `yaml:"observability"`
	McpServers      []any          `yaml:"mcp_servers"`
}

// Metadata names the project and declares its language and provider.
type Metadata struct {
	Name     string `yaml:"name"`
	Language string `yaml:"language"`
	Provider string `yaml:"provider"`
}

// GraphRef points at the graph file relative to the project root.
type GraphRef struct {
	Ref string `yaml:"$ref"`
}

// Observability mirrors the optional observability block (defaults: enabled,
// port 7700 — kept in sync with or-lens).
type Observability struct {
	Enabled       *bool `yaml:"enabled"`
	DashboardPort *int  `yaml:"dashboard_port"`
}

// DashboardPort returns the configured port, or 0 when observability is disabled.
func (c *Config) DashboardPort(defaultPort int) int {
	if c.Observability == nil {
		return defaultPort
	}
	if c.Observability.Enabled != nil && !*c.Observability.Enabled {
		return 0
	}
	if c.Observability.DashboardPort != nil {
		return *c.Observability.DashboardPort
	}
	return defaultPort
}

// GraphPath resolves the referenced graph file against the project root.
func (c *Config) GraphPath(projectDir string) string {
	return filepath.Join(projectDir, filepath.FromSlash(c.Graph.Ref))
}

// Load reads and validates orchustr.yaml from projectDir.
func Load(projectDir string) (*Config, error) {
	raw, err := os.ReadFile(filepath.Join(projectDir, FileName))
	if err != nil {
		return nil, clierr.Wrap(clierr.KindProject, err,
			"no readable %s in %s — is this an Orchustr project?", FileName, projectDir)
	}
	var cfg Config
	if err := yaml.Unmarshal(raw, &cfg); err != nil {
		return nil, clierr.Wrap(clierr.KindConfig, err, "%s is not valid YAML", FileName)
	}
	if strings.TrimSpace(cfg.Project.Name) == "" {
		return nil, clierr.New(clierr.KindConfig, "%s must set project.name", FileName)
	}
	if strings.TrimSpace(cfg.Graph.Ref) == "" {
		return nil, clierr.New(clierr.KindConfig, "%s must set graph.$ref", FileName)
	}
	return &cfg, nil
}
