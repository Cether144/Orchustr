package domain

import (
	"path/filepath"
	"testing"

	"github.com/Regent33/Orchustr/cli/shared/kernel/projectcfg"
)

func existsOnly(names ...string) FileExists {
	allowed := map[string]struct{}{}
	for _, name := range names {
		allowed[filepath.Join("proj", name)] = struct{}{}
	}
	return func(path string) bool {
		_, ok := allowed[path]
		return ok
	}
}

func configFor(language string) *projectcfg.Config {
	return &projectcfg.Config{Project: projectcfg.Metadata{Name: "p", Language: language}}
}

func TestRustPlanUsesCargo(t *testing.T) {
	plan, err := BuildLaunchPlan("proj", configFor("rust"), existsOnly("Cargo.toml"))
	if err != nil {
		t.Fatalf("plan failed: %v", err)
	}
	if plan.Program != "cargo" || plan.Args[0] != "run" {
		t.Fatalf("unexpected plan: %+v", plan)
	}
}

func TestPythonPlanPrefersMainPy(t *testing.T) {
	plan, err := BuildLaunchPlan("proj", configFor("python"), existsOnly("main.py", "app.py"))
	if err != nil {
		t.Fatalf("plan failed: %v", err)
	}
	if plan.Args[0] != "main.py" {
		t.Fatalf("expected main.py, got %+v", plan)
	}
}

func TestTypescriptFallsBackToTsx(t *testing.T) {
	plan, err := BuildLaunchPlan("proj", configFor("typescript"), existsOnly(filepath.FromSlash("src/index.ts")))
	if err != nil {
		t.Fatalf("plan failed: %v", err)
	}
	if plan.Args[0] != "tsx" {
		t.Fatalf("expected tsx fallback, got %+v", plan)
	}
}

func TestMissingEntrypointFails(t *testing.T) {
	if _, err := BuildLaunchPlan("proj", configFor("rust"), existsOnly()); err == nil {
		t.Fatal("expected missing Cargo.toml to fail")
	}
}

func TestUnsupportedLanguageFails(t *testing.T) {
	if _, err := BuildLaunchPlan("proj", configFor("cobol"), existsOnly()); err == nil {
		t.Fatal("expected unsupported language to fail")
	}
}
