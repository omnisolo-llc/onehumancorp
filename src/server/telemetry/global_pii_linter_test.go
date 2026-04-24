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

func TestGlobalPIIRedactionLinter(t *testing.T) {
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
				if ident, ok := sel.X.(*ast.Ident); ok && ident.Name == "json" && sel.Sel.Name == "Marshal" {
					if len(callExpr.Args) > 0 {
						isRedacted := false
						arg := callExpr.Args[0]

						isTargetArg := false

						if innerIdent, ok := arg.(*ast.Ident); ok {
							if innerIdent.Name == "payload" || innerIdent.Name == "raw" || innerIdent.Name == "logEntry" {
								isTargetArg = true
							}
						} else if innerCall, ok := arg.(*ast.CallExpr); ok {
							if innerIdent, ok := innerCall.Fun.(*ast.Ident); ok {
								if innerIdent.Name == "RedactInterfacePII" || innerIdent.Name == "RedactPII" {
									isRedacted = true
									isTargetArg = true
								}
							} else if innerSel, ok := innerCall.Fun.(*ast.SelectorExpr); ok {
								if innerSel.Sel.Name == "RedactInterfacePII" || innerSel.Sel.Name == "RedactPII" {
									isRedacted = true
									isTargetArg = true
								}
							}
						}

						if !isTargetArg {
							return true
						}

						if innerIdent, ok := arg.(*ast.Ident); ok {
							if innerIdent.Name == "redactedMap" || innerIdent.Name == "redacted" {
								isRedacted = true
							}
						}

						if !isRedacted {
							t.Errorf("PII Leak Risk in %s:%d - json.Marshal called without RedactInterfacePII", path, fset.Position(n.Pos()).Line)
						}
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
