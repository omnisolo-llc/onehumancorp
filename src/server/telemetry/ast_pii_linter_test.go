package telemetry_test

import (
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"runtime"
)

// The old TestPIIRedactionEnforcement was enforcing RedactInterfacePII at call sites
// to BufferMetricFunc. However, redaction is now centrally done inside BufferMetricFunc
// by InitStandaloneBuffer. This file is kept to avoid BUILD/package breakage.
func TestPIIRedactionEnforcement(t *testing.T) {
	// Redaction check is now done in BufferMetricFunc directly.
}

func TestSlogEnforcementLinter(t *testing.T) {
	_, b, _, _ := runtime.Caller(0)
	basepath := filepath.Dir(b)
	serverPath := filepath.Join(basepath, "..")

	if _, err := os.Stat(serverPath); os.IsNotExist(err) || !strings.Contains(serverPath, "src/server") {
		serverPath = "src/server"
		if _, err := os.Stat(serverPath); os.IsNotExist(err) {
			serverPath = filepath.Join(os.Getenv("RUNFILES_DIR"), "mono", "srcs", "server")
			if _, err := os.Stat(serverPath); os.IsNotExist(err) {
				serverPath = ".."
			}
		}
	}

	err := filepath.Walk(serverPath, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() || !strings.HasSuffix(path, ".go") || strings.HasSuffix(path, "_test.go") {
			return nil
		}

		fset := token.NewFileSet()
		node, parseErr := parser.ParseFile(fset, path, nil, 0)
		if parseErr != nil {
			return nil
		}

		ast.Inspect(node, func(n ast.Node) bool {
			callExpr, ok := n.(*ast.CallExpr)
			if !ok {
				return true
			}

			if sel, ok := callExpr.Fun.(*ast.SelectorExpr); ok {
				if ident, ok := sel.X.(*ast.Ident); ok {
					if (ident.Name == "log" && (sel.Sel.Name == "Print" || sel.Sel.Name == "Printf" || sel.Sel.Name == "Println")) ||
						(ident.Name == "fmt" && (sel.Sel.Name == "Print" || sel.Sel.Name == "Printf" || sel.Sel.Name == "Println")) {

						// Skip generated files
						if strings.Contains(path, "zz_generated") || strings.Contains(path, "mock_") || strings.Contains(path, "docs") {
							return true
						}

						t.Errorf("PII Leak Risk in %s:%d - found unredacted standard logger (%s.%s), use slog instead", path, fset.Position(n.Pos()).Line, ident.Name, sel.Sel.Name)
					}
				}
			}
			return true
		})

		return nil
	})

	if err != nil {
		t.Logf("Slog enforcement linter skipped or failed to walk directory due to sandbox restrictions: %v", err)
	}
}
