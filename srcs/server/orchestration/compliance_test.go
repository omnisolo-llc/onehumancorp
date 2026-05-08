package orchestration

import (
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestNoPIILeakage(t *testing.T) {
	rootDir := ".."

	err := filepath.Walk(rootDir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		if info.IsDir() || !strings.HasSuffix(path, ".go") || strings.HasSuffix(path, "_test.go") {
			return nil
		}

		if strings.Contains(path, "sanitizer") {
			return nil
		}

		fset := token.NewFileSet()
		node, err := parser.ParseFile(fset, path, nil, 0)
		if err != nil {
			return nil // ignore unparseable
		}

		ast.Inspect(node, func(n ast.Node) bool {
			call, ok := n.(*ast.CallExpr)
			if !ok {
				return true
			}

			var isTargetCall bool
			var callName string

			if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
				if ident, ok := sel.X.(*ast.Ident); ok {
					if ident.Name == "log" && (sel.Sel.Name == "Printf" || sel.Sel.Name == "Println") {
						isTargetCall = true
						callName = "log." + sel.Sel.Name
					}
				}
				if sel.Sel.Name == "GenerateEmbedding" {
					isTargetCall = true
					callName = "GenerateEmbedding"
				}
			}

			if isTargetCall {
				for _, arg := range call.Args {
					// We strictly look for *ast.Ident (variables) named "payload" or "content".
					// This avoids false positives from string literals that contain the word "payload".
					ast.Inspect(arg, func(an ast.Node) bool {
						if ident, ok := an.(*ast.Ident); ok {
							if ident.Name == "payload" || ident.Name == "content" {
								t.Errorf("Potential PII Leakage found in %s: %s contains unsanitized variable '%s'", path, callName, ident.Name)
							}
						}
						return true
					})
				}
			}
			return true
		})

		return nil
	})

	if err != nil {
		t.Fatalf("Failed to walk directory: %v", err)
	}
}
