package domain

import (
	"os"
	"path/filepath"

	"github.com/Regent33/Orchustr/cli/shared/kernel/clierr"
)

// RenderedFile is one project file produced by a template renderer.
type RenderedFile struct {
	RelativePath string
	Contents     []byte
}

// TemplateRenderer is the data-layer port that turns Options into files.
type TemplateRenderer interface {
	Render(options *Options) ([]RenderedFile, error)
}

// FileWriter is the data-layer port that persists rendered files.
type FileWriter interface {
	Write(path string, contents []byte) error
}

// InitUseCase scaffolds a new Orchustr project from validated options.
type InitUseCase struct {
	renderer TemplateRenderer
	writer   FileWriter
}

// NewInitUseCase wires the use case with its data-layer dependencies.
func NewInitUseCase(renderer TemplateRenderer, writer FileWriter) *InitUseCase {
	return &InitUseCase{renderer: renderer, writer: writer}
}

// Execute validates options, renders every template, and writes the project.
// It refuses to touch an existing directory so it can never clobber work.
func (uc *InitUseCase) Execute(options *Options) (string, error) {
	if err := options.Validate(); err != nil {
		return "", err
	}
	root := filepath.Join(options.TargetDir, options.ProjectName)
	if _, err := os.Stat(root); err == nil {
		return "", clierr.New(clierr.KindProject, "project directory already exists: %s", root)
	}
	files, err := uc.renderer.Render(options)
	if err != nil {
		return "", err
	}
	for _, file := range files {
		target := filepath.Join(root, filepath.FromSlash(file.RelativePath))
		if err := uc.writer.Write(target, file.Contents); err != nil {
			return "", err
		}
	}
	return root, nil
}
