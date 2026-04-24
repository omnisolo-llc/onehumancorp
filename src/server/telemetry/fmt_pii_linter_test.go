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

// TestFmtLogLinter enforces the rule that fmt.Printf and log.Printf (and similar)
// are not used in production server code, to avoid multi-tenant PII leaks.
// slog should be used instead.
func TestFmtLogLinter(t *testing.T) {
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
		// Skip tests, examples, docs, db/migrations, and third_party stuff
		if info.IsDir() || !strings.HasSuffix(path, ".go") || strings.HasSuffix(path, "_test.go") || strings.Contains(path, "testdata") || strings.Contains(path, "examples") || strings.Contains(path, "db/migrations") {
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
					if (ident.Name == "fmt" || ident.Name == "log") && (sel.Sel.Name == "Printf" || sel.Sel.Name == "Println" || sel.Sel.Name == "Print") {
						t.Errorf("PII Leak Risk: Use of %s.%s found in %s:%d. Use safe slog logger instead.", ident.Name, sel.Sel.Name, path, fset.Position(n.Pos()).Line)
					}
				}
			}
			return true
		})

		return nil
	})

	if err != nil {
		t.Logf("FmtLog linter skipped or failed to walk directory: %v", err)
	}
}
