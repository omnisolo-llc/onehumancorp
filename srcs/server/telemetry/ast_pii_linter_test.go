package telemetry_test

import (
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestPIIRedactionEnforcement(t *testing.T) {
	fset := token.NewFileSet()
	baseDir, _ := filepath.Abs("../..") // srcs directory

	err := filepath.Walk(baseDir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() || !strings.HasSuffix(path, ".go") || strings.HasSuffix(path, "_test.go") {
			return nil
		}

		node, err := parser.ParseFile(fset, path, nil, 0)
		if err != nil {
			return nil // ignore unparseable
		}

		ast.Inspect(node, func(n ast.Node) bool {
			fn, ok := n.(*ast.FuncDecl)
			if !ok {
				return true
			}

			callsBufferMetric := false
			callsRedactInterfacePII := false

			ast.Inspect(fn.Body, func(bodyNode ast.Node) bool {
				callExpr, ok := bodyNode.(*ast.CallExpr)
				if !ok {
					return true
				}

				if ident, ok := callExpr.Fun.(*ast.Ident); ok {
					if ident.Name == "BufferMetricFunc" {
						callsBufferMetric = true
					}
					if ident.Name == "RedactInterfacePII" || ident.Name == "RedactPII" {
						callsRedactInterfacePII = true
					}
				}

				if sel, ok := callExpr.Fun.(*ast.SelectorExpr); ok {
					if ident, ok := sel.X.(*ast.Ident); ok && ident.Name == "telemetry" && sel.Sel.Name == "BufferMetricFunc" {
						callsBufferMetric = true
					}
					if ident, ok := sel.X.(*ast.Ident); ok && ident.Name == "telemetry" && (sel.Sel.Name == "RedactInterfacePII" || sel.Sel.Name == "RedactPII") {
						callsRedactInterfacePII = true
					}
				}
				return true
			})

			if callsBufferMetric && !callsRedactInterfacePII {
				t.Errorf("File %s, Function %s calls BufferMetricFunc but misses RedactInterfacePII/RedactPII", path, fn.Name.Name)
			}

			return true
		})
		return nil
	})

	if err != nil {
		t.Fatalf("Failed to walk files: %v", err)
	}
}
