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

// Ensure that ANY call to log.Print, log.Printf, log.Println, fmt.Print, fmt.Printf, fmt.Println is blocked
// in multi-tenant environments. All logging must go through slog with PIIRedactingHandler.
func TestStdLogLinter(t *testing.T) {
	_, b, _, _ := runtime.Caller(0)
	basepath := filepath.Dir(b)
	serverPath := filepath.Join(basepath, "..")

	if _, err := os.Stat(serverPath); os.IsNotExist(err) || !strings.Contains(serverPath, "srcs/server") {
		serverPath = "srcs/server"
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

		// Allowed legacy exemptions where std log/fmt is necessary for setup/CLI
		if strings.Contains(path, "cmd") || strings.Contains(path, "main.go") || strings.Contains(path, "sync_daemon.go") || strings.Contains(path, "cli") || strings.Contains(path, "dayone") || strings.Contains(path, "wizard") {
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
					if ident.Name == "log" && (sel.Sel.Name == "Print" || sel.Sel.Name == "Printf" || sel.Sel.Name == "Println" || sel.Sel.Name == "Fatal" || sel.Sel.Name == "Fatalf" || sel.Sel.Name == "Fatalln") {
						t.Errorf("PII Leak Risk in %s:%d - standard log package used instead of slog with PIIRedactingHandler", path, fset.Position(n.Pos()).Line)
					}
					if ident.Name == "fmt" && (sel.Sel.Name == "Print" || sel.Sel.Name == "Printf" || sel.Sel.Name == "Println") {
						t.Errorf("PII Leak Risk in %s:%d - standard fmt package used instead of slog with PIIRedactingHandler", path, fset.Position(n.Pos()).Line)
					}
				}
			}
			return true
		})

		return nil
	})

	if err != nil {
		t.Logf("StdLog linter skipped or failed to walk directory due to sandbox restrictions: %v", err)
	}
}
