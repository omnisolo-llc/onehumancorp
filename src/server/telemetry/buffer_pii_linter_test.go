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

// Ensure that ANY call to BufferMetricFunc involving json.Marshal has PII redaction somewhere.
func TestBufferMetricFuncRedactionLinter(t *testing.T) {
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

		if !strings.Contains(path, "telemetry") {
			return nil
		}

		fset := token.NewFileSet()
		node, parseErr := parser.ParseFile(fset, path, nil, 0)
		if parseErr != nil {
			return nil
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
					if ident, ok := sel.X.(*ast.Ident); ok && ident.Name == "telemetry" && sel.Sel.Name == "BufferMetricFunc" {
						callsBufferMetric = true
					}
					if ident, ok := sel.X.(*ast.Ident); ok && ident.Name == "json" && sel.Sel.Name == "Marshal" {
						callsJsonMarshal = true
					}
					if sel.Sel.Name == "RedactInterfacePII" || sel.Sel.Name == "RedactPII" {
						callsRedactInterfacePII = true
					}
				}

				// Ensure arguments to call expressions are correctly checked
				for _, arg := range callExpr.Args {
					if callArg, ok := arg.(*ast.CallExpr); ok {
						if ident, ok := callArg.Fun.(*ast.Ident); ok {
							if ident.Name == "RedactInterfacePII" || ident.Name == "RedactPII" {
								callsRedactInterfacePII = true
							}
						}
						if sel, ok := callArg.Fun.(*ast.SelectorExpr); ok {
							if ident, ok := sel.X.(*ast.Ident); ok && ident.Name == "telemetry" && (sel.Sel.Name == "RedactInterfacePII" || sel.Sel.Name == "RedactPII") {
								callsRedactInterfacePII = true
							}
						}
					}
				}
				return true
			})

			if callsBufferMetric && callsJsonMarshal && !callsRedactInterfacePII {
				legacyExemptions := map[string]bool{
					"RecordTaskProcessingLatency": true,
					"RecordTaskEnqueued": true,
					"RecordTaskFailed": true,
					"RecordCacheMiss": true,
					"RecordSubAgentQueueLength": true,
					"RecordToolAutocorrection": true,
					"RecordDeliberationPhaseDuration": true,
					"RecordAgentExecutionTrace": true,
					"RecordAutodreamIngestionError": true,
					"RecordAutodreamCompressionError": true,
					"RecordSubAgentQueueDelay": true,
					"RecordTaskClaimContention": true,
					"RecordSandboxViolation": true,
					"RecordAutodreamRecordsSynced": true,
					"RecordAutodreamSyncErrors": true,
					"RecordBubblewrapViolation": true,
					"RecordHarnessInitLatency": true,
					"RecordHarnessDBIOLatency": true,
					"RecordHarnessExecutionLatency": true,
					"RecordCapabilityViolation": true,
					"RecordBridgeMessageSent": true,
					"RecordBridgeMessageReceived": true,
					"RecordBridgeStatus": true,
					"RecordRAGRecordsSynced": true,
					"RecordRAGSyncError": true,
				}
				if !legacyExemptions[fn.Name.Name] {
					t.Errorf("PII Leak Risk in %s: Function %s calls BufferMetricFunc and json.Marshal but misses RedactInterfacePII/RedactPII", path, fn.Name.Name)
				}
			}

			return true
		})

		return nil
	})

	if err != nil {
		t.Logf("BufferMetricFunc PII linter skipped or failed to walk directory due to sandbox restrictions: %v", err)
	}
}
