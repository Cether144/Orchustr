// Package domain decides how `orchustr run` launches a project, mirroring the
// Rust DefaultProjectRunner: detect the declared language and shell out to its
// canonical entrypoint.
package domain

import (
	"path/filepath"
	"runtime"

	"github.com/Regent33/Orchustr/cli/shared/kernel/clierr"
	"github.com/Regent33/Orchustr/cli/shared/kernel/projectcfg"
)

// FileExists is the filesystem probe injected from the data layer.
type FileExists func(path string) bool

// LaunchPlan names the program and arguments used to run a project.
type LaunchPlan struct {
	Program string
	Args    []string
}

// BuildLaunchPlan inspects the project layout and returns the launch command.
func BuildLaunchPlan(projectDir string, cfg *projectcfg.Config, exists FileExists) (*LaunchPlan, error) {
	join := func(name string) string { return filepath.Join(projectDir, name) }
	switch cfg.Project.Language {
	case "rust":
		if exists(join("Cargo.toml")) {
			return &LaunchPlan{Program: "cargo", Args: []string{"run"}}, nil
		}
		return nil, clierr.New(clierr.KindProject, "no Cargo.toml found in %s", projectDir)
	case "python":
		for _, candidate := range []string{"main.py", "agent.py", "app.py"} {
			if exists(join(candidate)) {
				return &LaunchPlan{Program: pythonProgram(), Args: []string{candidate}}, nil
			}
		}
		return nil, clierr.New(clierr.KindProject,
			"no Python entrypoint (main.py / agent.py / app.py) in %s", projectDir)
	case "typescript":
		if exists(join("package.json")) {
			return &LaunchPlan{Program: npmProgram(), Args: []string{"start"}}, nil
		}
		if exists(join(filepath.FromSlash("src/index.ts"))) {
			return &LaunchPlan{Program: npxProgram(), Args: []string{"tsx", "src/index.ts"}}, nil
		}
		return nil, clierr.New(clierr.KindProject, "no package.json or src/index.ts in %s", projectDir)
	case "dart":
		if exists(join("pubspec.yaml")) {
			return &LaunchPlan{Program: "dart", Args: []string{"run"}}, nil
		}
		return nil, clierr.New(clierr.KindProject, "no pubspec.yaml in %s", projectDir)
	default:
		return nil, clierr.New(clierr.KindConfig,
			"unsupported project.language %q in orchustr.yaml", cfg.Project.Language)
	}
}

func pythonProgram() string {
	if runtime.GOOS == "windows" {
		return "python"
	}
	return "python3"
}

func npmProgram() string {
	if runtime.GOOS == "windows" {
		return "npm.cmd"
	}
	return "npm"
}

func npxProgram() string {
	if runtime.GOOS == "windows" {
		return "npx.cmd"
	}
	return "npx"
}
