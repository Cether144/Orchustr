package domain

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

type memWriter struct{ written map[string][]byte }

func (m *memWriter) Write(path string, contents []byte) error {
	m.written[path] = contents
	return nil
}

func projectDir(t *testing.T, language string) string {
	t.Helper()
	dir := t.TempDir()
	config := "orchustr_version: \"0.1.3\"\nproject:\n  name: p\n  language: " + language +
		"\n  provider: anthropic\ngraph:\n  $ref: ./graph.yaml\n"
	if err := os.WriteFile(filepath.Join(dir, "orchustr.yaml"), []byte(config), 0o644); err != nil {
		t.Fatal(err)
	}
	return dir
}

func TestNodeUsesProjectLanguageExtension(t *testing.T) {
	writer := &memWriter{written: map[string][]byte{}}
	dir := projectDir(t, "python")

	path, err := NewScaffoldUseCase(writer).Node(dir, "summarize")
	if err != nil {
		t.Fatalf("scaffold failed: %v", err)
	}
	if !strings.HasSuffix(path, "summarize.py") {
		t.Fatalf("expected .py stub, got %s", path)
	}
	if !strings.Contains(string(writer.written[path]), "async def summarize") {
		t.Fatal("python stub body missing")
	}
}

func TestNodeRejectsUnsafeName(t *testing.T) {
	writer := &memWriter{written: map[string][]byte{}}
	if _, err := NewScaffoldUseCase(writer).Node(projectDir(t, "rust"), "../evil"); err == nil {
		t.Fatal("expected unsafe node name to be rejected")
	}
}

func TestTopologyRequiresProject(t *testing.T) {
	writer := &memWriter{written: map[string][]byte{}}
	if _, err := NewScaffoldUseCase(writer).Topology(t.TempDir(), "debate"); err == nil {
		t.Fatal("expected missing orchustr.yaml to fail")
	}
}

func TestTopologyWritesRustStub(t *testing.T) {
	writer := &memWriter{written: map[string][]byte{}}
	path, err := NewScaffoldUseCase(writer).Topology(projectDir(t, "rust"), "debate")
	if err != nil {
		t.Fatalf("scaffold failed: %v", err)
	}
	if !strings.Contains(string(writer.written[path]), "impl LoopTopology for debateTopology") {
		t.Fatal("topology stub body missing")
	}
}
