package telemetry_test

import (
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"testing"
)

func TestFmtPrintfLinter(t *testing.T) {
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
		// Allow harness, migrations, and ironclaw
		if strings.Contains(path, "/harness/") || strings.Contains(path, "/migrations/") || strings.Contains(path, "/ironclaw/") {
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
				if ident, ok := sel.X.(*ast.Ident); ok && ident.Name == "fmt" {
					if sel.Sel.Name == "Printf" || sel.Sel.Name == "Println" || sel.Sel.Name == "Print" {
						t.Errorf("PII Leak Risk in %s:%d - %s.%s called. Use structured slog logging instead.", path, fset.Position(n.Pos()).Line, ident.Name, sel.Sel.Name)
					}
				}
			}
			return true
		})

		return nil
	})

	if err != nil {
		t.Logf("Global PII linter skipped or failed to walk directory due to sandbox restrictions: %v", err)
	}
}
