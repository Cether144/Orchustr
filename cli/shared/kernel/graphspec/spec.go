// Package graphspec mirrors the Rust or-schema GraphSpec descriptor so the Go
// CLI lints exactly the same files the Rust runtimes load.
package graphspec

import (
	"encoding/json"
	"strings"

	"github.com/Regent33/Orchustr/cli/shared/kernel/clierr"
	"gopkg.in/yaml.v3"
)

// GraphSpec is a serializable Orchustr graph definition (see or-schema).
type GraphSpec struct {
	Name    string     `yaml:"name" json:"name"`
	Version string     `yaml:"version" json:"version"`
	Nodes   []NodeSpec `yaml:"nodes" json:"nodes"`
	Edges   []EdgeSpec `yaml:"edges" json:"edges"`
	Entry   string     `yaml:"entry" json:"entry"`
	Exits   []string   `yaml:"exits" json:"exits"`
}

// NodeSpec declares one node; the id must match a registered handler key.
type NodeSpec struct {
	ID       string `yaml:"id" json:"id"`
	Handler  string `yaml:"handler" json:"handler"`
	Metadata any    `yaml:"metadata" json:"metadata"`
}

// EdgeSpec connects two declared nodes with an optional routing predicate.
type EdgeSpec struct {
	From      string  `yaml:"from" json:"from"`
	To        string  `yaml:"to" json:"to"`
	Condition *string `yaml:"condition,omitempty" json:"condition,omitempty"`
}

// Parse decodes raw graph text. JSON is used when isJSON is true, YAML otherwise.
func Parse(raw []byte, isJSON bool) (*GraphSpec, error) {
	var spec GraphSpec
	if isJSON {
		if err := json.Unmarshal(raw, &spec); err != nil {
			return nil, clierr.Wrap(clierr.KindValidation, err, "graph is not valid JSON")
		}
	} else if err := yaml.Unmarshal(raw, &spec); err != nil {
		return nil, clierr.Wrap(clierr.KindValidation, err, "graph is not valid YAML")
	}
	return &spec, nil
}

// Validate enforces the same structural rules as the Rust or-cli linter:
// required fields, declared entry, at least one declared exit, and edges
// that reference declared nodes only.
func (g *GraphSpec) Validate() error {
	if strings.TrimSpace(g.Name) == "" {
		return clierr.New(clierr.KindValidation, "graph must declare a name")
	}
	if strings.TrimSpace(g.Entry) == "" {
		return clierr.New(clierr.KindValidation, "graph must declare an entry node")
	}
	nodes := make(map[string]struct{}, len(g.Nodes))
	for _, node := range g.Nodes {
		if strings.TrimSpace(node.ID) == "" || strings.TrimSpace(node.Handler) == "" {
			return clierr.New(clierr.KindValidation, "every node must declare an id and a handler")
		}
		nodes[node.ID] = struct{}{}
	}
	if _, ok := nodes[g.Entry]; !ok {
		return clierr.New(clierr.KindValidation, "entry node '%s' is not declared", g.Entry)
	}
	if len(g.Exits) == 0 {
		return clierr.New(clierr.KindValidation, "graph must declare at least one exit node")
	}
	for _, exit := range g.Exits {
		if _, ok := nodes[exit]; !ok {
			return clierr.New(clierr.KindValidation, "exit node '%s' is not declared", exit)
		}
	}
	for _, edge := range g.Edges {
		if _, ok := nodes[edge.From]; !ok {
			return clierr.New(clierr.KindValidation, "edge '%s' -> '%s' references an unknown node", edge.From, edge.To)
		}
		if _, ok := nodes[edge.To]; !ok {
			return clierr.New(clierr.KindValidation, "edge '%s' -> '%s' references an unknown node", edge.From, edge.To)
		}
	}
	return nil
}
