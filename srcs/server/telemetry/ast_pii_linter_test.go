package telemetry_test

import (
	"go/ast"
	"go/parser"
	"go/token"
	"path/filepath"
	"testing"
)

func TestPIIRedactionEnforcement(t *testing.T) {
	fset := token.NewFileSet()
	path := filepath.Join(".", "telemetry.go")
	node, err := parser.ParseFile(fset, path, nil, 0)
	if err != nil {
		t.Fatalf("Failed to parse telemetry.go: %v", err)
	}

	ast.Inspect(node, func(n ast.Node) bool {
		fn, ok := n.(*ast.FuncDecl)
		if !ok {
			return true
		}

		callsBufferMetric := false
		callsJsonMarshal := false
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
				if ident, ok := sel.X.(*ast.Ident); ok && ident.Name == "json" && sel.Sel.Name == "Marshal" {
					callsJsonMarshal = true
				}
			}
			return true
		})

		if callsBufferMetric && callsJsonMarshal && !callsRedactInterfacePII {
			t.Errorf("Function %s calls BufferMetricFunc and json.Marshal but misses RedactInterfacePII/RedactPII", fn.Name.Name)
		}

		return true
	})
}
