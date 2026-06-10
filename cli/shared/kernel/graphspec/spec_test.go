package graphspec

import "testing"

const validGraph = `
name: test-agent
version: "0.1.0"
entry: think
exits: [done]
nodes:
  - id: think
    handler: nodes::think
    metadata: {}
  - id: done
    handler: nodes::done
    metadata: {}
edges:
  - from: think
    to: done
`

func TestValidGraphPasses(t *testing.T) {
	spec, err := Parse([]byte(validGraph), false)
	if err != nil {
		t.Fatalf("parse failed: %v", err)
	}
	if err := spec.Validate(); err != nil {
		t.Fatalf("expected valid graph, got: %v", err)
	}
}

func TestUndeclaredEntryFails(t *testing.T) {
	spec, err := Parse([]byte(validGraph), false)
	if err != nil {
		t.Fatalf("parse failed: %v", err)
	}
	spec.Entry = "missing"
	if err := spec.Validate(); err == nil {
		t.Fatal("expected undeclared entry to fail validation")
	}
}

func TestMissingExitsFails(t *testing.T) {
	spec, _ := Parse([]byte(validGraph), false)
	spec.Exits = nil
	if err := spec.Validate(); err == nil {
		t.Fatal("expected graph without exits to fail validation")
	}
}

func TestUnknownEdgeNodeFails(t *testing.T) {
	spec, _ := Parse([]byte(validGraph), false)
	spec.Edges[0].To = "ghost"
	if err := spec.Validate(); err == nil {
		t.Fatal("expected edge to unknown node to fail validation")
	}
}

func TestInvalidYamlFails(t *testing.T) {
	if _, err := Parse([]byte("nodes: ["), false); err == nil {
		t.Fatal("expected invalid YAML to fail parsing")
	}
}

func TestJsonGraphParses(t *testing.T) {
	raw := `{"name":"j","version":"0.1.0","entry":"a","exits":["a"],` +
		`"nodes":[{"id":"a","handler":"nodes::a","metadata":{}}],"edges":[]}`
	spec, err := Parse([]byte(raw), true)
	if err != nil {
		t.Fatalf("parse failed: %v", err)
	}
	if err := spec.Validate(); err != nil {
		t.Fatalf("expected valid JSON graph, got: %v", err)
	}
}
