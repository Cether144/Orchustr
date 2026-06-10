package domain

import (
	"path/filepath"
	"testing"
)

type fakeRenderer struct{ files []RenderedFile }

func (f *fakeRenderer) Render(*Options) ([]RenderedFile, error) { return f.files, nil }

type memWriter struct{ written map[string][]byte }

func (m *memWriter) Write(path string, contents []byte) error {
	m.written[path] = contents
	return nil
}

func options(t *testing.T) *Options {
	t.Helper()
	return &Options{
		ProjectName: "hermes-agent",
		Language:    LangRust,
		Topology:    TopologyReact,
		Provider:    ProviderAnthropic,
		TargetDir:   t.TempDir(),
	}
}

func TestInitWritesRenderedFiles(t *testing.T) {
	renderer := &fakeRenderer{files: []RenderedFile{
		{RelativePath: "orchustr.yaml", Contents: []byte("x")},
		{RelativePath: "src/main.rs", Contents: []byte("y")},
	}}
	writer := &memWriter{written: map[string][]byte{}}
	opts := options(t)

	root, err := NewInitUseCase(renderer, writer).Execute(opts)
	if err != nil {
		t.Fatalf("init failed: %v", err)
	}
	if root != filepath.Join(opts.TargetDir, "hermes-agent") {
		t.Fatalf("unexpected root: %s", root)
	}
	if len(writer.written) != 2 {
		t.Fatalf("expected 2 files, wrote %d", len(writer.written))
	}
}

func TestInitRejectsUnsafeProjectName(t *testing.T) {
	writer := &memWriter{written: map[string][]byte{}}
	opts := options(t)
	opts.ProjectName = "../escape"

	if _, err := NewInitUseCase(&fakeRenderer{}, writer).Execute(opts); err == nil {
		t.Fatal("expected path-traversal name to be rejected")
	}
	if len(writer.written) != 0 {
		t.Fatal("no files may be written on validation failure")
	}
}

func TestInitRejectsUnsupportedLanguage(t *testing.T) {
	opts := options(t)
	opts.Language = Language("cobol")
	if _, err := NewInitUseCase(&fakeRenderer{}, &memWriter{written: map[string][]byte{}}).Execute(opts); err == nil {
		t.Fatal("expected unsupported language to be rejected")
	}
}
