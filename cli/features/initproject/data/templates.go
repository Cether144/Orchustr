// Package data implements the template-rendering port for `orchustr init`
// using templates embedded at compile time.
package data

import (
	"embed"
	"strings"

	"github.com/Regent33/Orchustr/cli/app/config"
	"github.com/Regent33/Orchustr/cli/features/initproject/domain"
	"github.com/Regent33/Orchustr/cli/shared/kernel/clierr"
)

//go:embed templates
var templateFS embed.FS

// EmbeddedRenderer renders project files from the embedded template tree.
type EmbeddedRenderer struct{}

// NewEmbeddedRenderer constructs the default renderer.
func NewEmbeddedRenderer() *EmbeddedRenderer { return &EmbeddedRenderer{} }

// Render produces every file for the requested language and topology.
func (r *EmbeddedRenderer) Render(options *domain.Options) ([]domain.RenderedFile, error) {
	files, err := r.commonFiles(options)
	if err != nil {
		return nil, err
	}
	languageFiles, err := r.languageFiles(options)
	if err != nil {
		return nil, err
	}
	return append(files, languageFiles...), nil
}

func (r *EmbeddedRenderer) commonFiles(options *domain.Options) ([]domain.RenderedFile, error) {
	graph, err := r.read("templates/graphs/" + string(options.Topology) + ".yaml")
	if err != nil {
		return nil, err
	}
	out := []domain.RenderedFile{{RelativePath: "graph.yaml", Contents: []byte(graph)}}
	for path, target := range map[string]string{
		"templates/common/orchustr.yaml.tmpl": "orchustr.yaml",
		"templates/common/env.example.tmpl":   ".env.example",
		"templates/common/gitignore.tmpl":     ".gitignore",
		"templates/common/README.md.tmpl":     "README.md",
	} {
		rendered, err := r.renderFile(path, options)
		if err != nil {
			return nil, err
		}
		out = append(out, domain.RenderedFile{RelativePath: target, Contents: []byte(rendered)})
	}
	return out, nil
}

func (r *EmbeddedRenderer) read(path string) (string, error) {
	raw, err := templateFS.ReadFile(path)
	if err != nil {
		return "", clierr.Wrap(clierr.KindIO, err, "embedded template missing: %s", path)
	}
	return string(raw), nil
}

func (r *EmbeddedRenderer) renderFile(path string, options *domain.Options) (string, error) {
	raw, err := r.read(path)
	if err != nil {
		return "", err
	}
	return substitute(raw, options), nil
}

// substitute applies the same flat placeholder scheme the Rust CLI used.
func substitute(template string, options *domain.Options) string {
	replacer := strings.NewReplacer(
		"{{project_name_snake}}", strings.ReplaceAll(options.ProjectName, "-", "_"),
		"{{project_name}}", options.ProjectName,
		"{{language}}", string(options.Language),
		"{{topology}}", string(options.Topology),
		"{{provider_env}}", options.Provider.EnvKey(),
		"{{provider}}", string(options.Provider),
		"{{repo_url}}", config.RepoURL,
	)
	return replacer.Replace(template)
}

// topologyNodes lists the node ids declared by each starter graph, in order.
func topologyNodes(topology domain.Topology) []string {
	switch topology {
	case domain.TopologyPlanExecute:
		return []string{"plan", "execute_step", "done"}
	case domain.TopologyReflection:
		return []string{"draft", "critique", "accept"}
	default:
		return []string{"think", "act", "done"}
	}
}
