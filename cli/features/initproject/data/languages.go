package data

import (
	"fmt"
	"strings"

	"github.com/Regent33/Orchustr/cli/features/initproject/domain"
)

func (r *EmbeddedRenderer) languageFiles(options *domain.Options) ([]domain.RenderedFile, error) {
	switch options.Language {
	case domain.LangPython:
		return r.pythonFiles(options)
	case domain.LangTypescript:
		return r.singleEntrypoint(options, map[string]string{
			"templates/typescript/package.json.tmpl":  "package.json",
			"templates/typescript/tsconfig.json.tmpl": "tsconfig.json",
			"templates/typescript/index.ts.tmpl":      "src/index.ts",
		}, tsHandler)
	case domain.LangDart:
		// `dart run` with no arguments executes bin/<package_name>.dart.
		return r.singleEntrypoint(options, map[string]string{
			"templates/dart/pubspec.yaml.tmpl": "pubspec.yaml",
			"templates/dart/main.dart.tmpl":    "bin/{{project_name_snake}}.dart",
		}, dartHandler)
	default:
		return r.rustFiles(options)
	}
}

func (r *EmbeddedRenderer) rustFiles(options *domain.Options) ([]domain.RenderedFile, error) {
	out := make([]domain.RenderedFile, 0, 8)
	for path, target := range map[string]string{
		"templates/rust/Cargo.toml.tmpl": "Cargo.toml",
		"templates/rust/main.rs.tmpl":    "src/main.rs",
	} {
		rendered, err := r.renderFile(path, options)
		if err != nil {
			return nil, err
		}
		out = append(out, domain.RenderedFile{RelativePath: target, Contents: []byte(rendered)})
	}
	nodeTemplate, err := r.read("templates/rust/node.rs.tmpl")
	if err != nil {
		return nil, err
	}
	var mods strings.Builder
	for _, node := range topologyNodes(options.Topology) {
		fmt.Fprintf(&mods, "pub mod %s;\n", node)
		out = append(out, domain.RenderedFile{
			RelativePath: "src/nodes/" + node + ".rs",
			Contents:     []byte(strings.ReplaceAll(nodeTemplate, "{{node_name}}", node)),
		})
	}
	out = append(out,
		domain.RenderedFile{RelativePath: "src/nodes/mod.rs", Contents: []byte(mods.String())},
		domain.RenderedFile{
			RelativePath: "tests/integration_test.rs",
			Contents:     []byte("#[test]\nfn generated_project_builds() {\n    assert!(true);\n}\n"),
		})
	return out, nil
}

func (r *EmbeddedRenderer) pythonFiles(options *domain.Options) ([]domain.RenderedFile, error) {
	out := make([]domain.RenderedFile, 0, 8)
	for path, target := range map[string]string{
		"templates/python/requirements.txt.tmpl": "requirements.txt",
		"templates/python/main.py.tmpl":          "main.py",
		"templates/python/test_agent.py.tmpl":    "tests/test_agent.py",
	} {
		rendered, err := r.renderFile(path, options)
		if err != nil {
			return nil, err
		}
		out = append(out, domain.RenderedFile{RelativePath: target, Contents: []byte(rendered)})
	}
	nodeTemplate, err := r.read("templates/python/node.py.tmpl")
	if err != nil {
		return nil, err
	}
	nodes := topologyNodes(options.Topology)
	var initPy strings.Builder
	initPy.WriteString("\"\"\"Node handlers for the generated agent.\"\"\"\n\n")
	for _, node := range nodes {
		fmt.Fprintf(&initPy, "from .%s import %s\n", node, node)
		out = append(out, domain.RenderedFile{
			RelativePath: "nodes/" + node + ".py",
			Contents:     []byte(strings.ReplaceAll(nodeTemplate, "{{node_name}}", node)),
		})
	}
	fmt.Fprintf(&initPy, "\nPIPELINE = [%s]\n", strings.Join(nodes, ", "))
	out = append(out, domain.RenderedFile{RelativePath: "nodes/__init__.py", Contents: []byte(initPy.String())})
	return out, nil
}

// singleEntrypoint renders languages whose nodes live inline in one entry file.
func (r *EmbeddedRenderer) singleEntrypoint(
	options *domain.Options,
	files map[string]string,
	handler func(node string) string,
) ([]domain.RenderedFile, error) {
	nodes := topologyNodes(options.Topology)
	handlers := make([]string, len(nodes))
	for i, node := range nodes {
		handlers[i] = handler(node)
	}
	out := make([]domain.RenderedFile, 0, len(files))
	for path, target := range files {
		rendered, err := r.renderFile(path, options)
		if err != nil {
			return nil, err
		}
		rendered = strings.ReplaceAll(rendered, "{{node_handlers}}", strings.Join(handlers, "\n"))
		rendered = strings.ReplaceAll(rendered, "{{node_list}}", strings.Join(nodes, ", "))
		out = append(out, domain.RenderedFile{
			RelativePath: substitute(target, options),
			Contents:     []byte(rendered),
		})
	}
	return out, nil
}

func tsHandler(node string) string {
	return fmt.Sprintf(
		"const %s: NodeHandler = async (state) => ({ ...state, %s: \"ok\" });", node, node)
}

func dartHandler(node string) string {
	return fmt.Sprintf(
		"Future<State> %s(State state) async => {...state, '%s': 'ok'};", node, node)
}
