package data

import (
	"strings"
	"testing"

	"github.com/Regent33/Orchustr/cli/features/initproject/domain"
)

func render(t *testing.T, language domain.Language, topology domain.Topology) map[string]string {
	t.Helper()
	files, err := NewEmbeddedRenderer().Render(&domain.Options{
		ProjectName: "hermes-agent",
		Language:    language,
		Topology:    topology,
		Provider:    domain.ProviderAnthropic,
	})
	if err != nil {
		t.Fatalf("render failed: %v", err)
	}
	out := map[string]string{}
	for _, file := range files {
		out[file.RelativePath] = string(file.Contents)
	}
	return out
}

func TestEveryLanguageRendersRunnableEntrypoint(t *testing.T) {
	entrypoints := map[domain.Language]string{
		domain.LangRust:       "Cargo.toml",
		domain.LangPython:     "main.py",
		domain.LangTypescript: "package.json",
		domain.LangDart:       "pubspec.yaml",
	}
	for language, entry := range entrypoints {
		files := render(t, language, domain.TopologyReact)
		if _, ok := files[entry]; !ok {
			t.Errorf("%s scaffold is missing %s — `orchustr run` would fail", language, entry)
		}
		for _, required := range []string{"orchustr.yaml", "graph.yaml", ".gitignore", ".env.example", "README.md"} {
			if _, ok := files[required]; !ok {
				t.Errorf("%s scaffold is missing common file %s", language, required)
			}
		}
	}
}

func TestNoUnreplacedPlaceholdersRemain(t *testing.T) {
	for _, language := range domain.Languages() {
		for _, topology := range domain.Topologies() {
			for path, contents := range render(t, language, topology) {
				if strings.Contains(contents, "{{") {
					t.Errorf("%s/%s: %s still contains template placeholders", language, topology, path)
				}
			}
		}
	}
}

func TestTopologyNodesMatchGraph(t *testing.T) {
	files := render(t, domain.LangPython, domain.TopologyPlanExecute)
	for _, node := range []string{"plan", "execute_step", "done"} {
		if _, ok := files["nodes/"+node+".py"]; !ok {
			t.Errorf("plan-execute python scaffold is missing nodes/%s.py", node)
		}
		if !strings.Contains(files["graph.yaml"], "id: "+node) {
			t.Errorf("plan-execute graph.yaml does not declare node %s", node)
		}
	}
}

func TestRustTemplateHasNoPathDependency(t *testing.T) {
	files := render(t, domain.LangRust, domain.TopologyReact)
	cargo := files["Cargo.toml"]
	if strings.Contains(cargo, "path = ") {
		t.Fatal("Rust template must not use path dependencies — they break outside the monorepo")
	}
	if !strings.Contains(cargo, "git = ") {
		t.Fatal("Rust template must pull or-schema from the monorepo git URL")
	}
}

func TestDartPackageNameIsSnakeCase(t *testing.T) {
	files := render(t, domain.LangDart, domain.TopologyReact)
	if !strings.Contains(files["pubspec.yaml"], "name: hermes_agent") {
		t.Fatal("dart package name must convert hyphens to underscores")
	}
}
