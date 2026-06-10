package domain

import (
	"os"
	"path/filepath"
	"testing"
)

const validGraph = `name: t
version: "0.1.0"
entry: a
exits: [a]
nodes:
  - id: a
    handler: nodes::a
    metadata: {}
edges: []
`

func writeFile(t *testing.T, dir, name, contents string) string {
	t.Helper()
	path := filepath.Join(dir, name)
	if err := os.WriteFile(path, []byte(contents), 0o644); err != nil {
		t.Fatal(err)
	}
	return path
}

func TestLintSingleValidFile(t *testing.T) {
	path := writeFile(t, t.TempDir(), "graph.yaml", validGraph)
	validated, err := NewLintUseCase().Execute(path)
	if err != nil {
		t.Fatalf("lint failed: %v", err)
	}
	if len(validated) != 1 {
		t.Fatalf("expected 1 validated path, got %d", len(validated))
	}
}

func TestLintProjectFollowsGraphRef(t *testing.T) {
	dir := t.TempDir()
	writeFile(t, dir, "orchustr.yaml",
		"orchustr_version: \"0.1.3\"\nproject:\n  name: p\n  language: rust\n  provider: anthropic\ngraph:\n  $ref: ./graph.yaml\n")
	writeFile(t, dir, "graph.yaml", validGraph)
	validated, err := NewLintUseCase().Execute(dir)
	if err != nil {
		t.Fatalf("lint failed: %v", err)
	}
	if len(validated) != 1 {
		t.Fatalf("expected 1 validated path, got %d", len(validated))
	}
}

func TestLintInvalidGraphFails(t *testing.T) {
	path := writeFile(t, t.TempDir(), "graph.yaml", "name: bad\nversion: \"0\"\nentry: ghost\nexits: []\nnodes: []\nedges: []\n")
	if _, err := NewLintUseCase().Execute(path); err == nil {
		t.Fatal("expected invalid graph to fail lint")
	}
}

func TestLintEmptyDirectoryFails(t *testing.T) {
	if _, err := NewLintUseCase().Execute(t.TempDir()); err == nil {
		t.Fatal("expected directory without graphs to fail lint")
	}
}
