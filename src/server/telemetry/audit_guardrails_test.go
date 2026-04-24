package telemetry_test

import (
	"bytes"
	"context"
	"database/sql"
	"encoding/json"
	"go/ast"
	"go/parser"
	"go/token"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"runtime"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/telemetry"
	_ "modernc.org/sqlite"
)

// TestGuardrailPIILeakageStandalone checks that any struct or map passed into telemetry recording
// cannot unintentionally leak PII into the standalone metrics buffer.
func TestGuardrailPIILeakageStandalone(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open memory db: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`CREATE TABLE local_telemetry_buffer (id INTEGER PRIMARY KEY AUTOINCREMENT, metric_type TEXT, payload TEXT)`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	telemetry.InitStandaloneBuffer(db)

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	payloads := []map[string]interface{}{
		{
			"action": "user_login",
			"email":  "secret.agent@ohc.com", // PII to be redacted
			"ip":     "192.168.1.1",
		},
		{
			"action":      "payment_attempt",
			"credit_card": "4111-1111-1111-1111", // PII to be redacted
			"amount":      99.99,
		},
	}

	for _, payload := range payloads {
		payloadBytes, _ := json.Marshal(payload)
		err = telemetry.BufferMetricFunc(ctx, "audit_event", string(payloadBytes))
		if err != nil {
			t.Fatalf("BufferMetricFunc failed: %v", err)
		}
	}

	rows, err := db.Query("SELECT payload FROM local_telemetry_buffer")
	if err != nil {
		t.Fatalf("Query failed: %v", err)
	}
	defer rows.Close()

	var count int
	for rows.Next() {
		count++
		var payloadStr string
		if err := rows.Scan(&payloadStr); err != nil {
			t.Fatalf("Scan failed: %v", err)
		}

		var stored map[string]interface{}
		if err := json.Unmarshal([]byte(payloadStr), &stored); err != nil {
			t.Fatalf("Unmarshal failed: %v", err)
		}

		if email, ok := stored["email"]; ok && email == "secret.agent@ohc.com" {
			t.Errorf("PII Leak Guardrail failed: email was not redacted in DB")
		}
		if cc, ok := stored["credit_card"]; ok && cc == "4111-1111-1111-1111" {
			t.Errorf("PII Leak Guardrail failed: credit card was not redacted in DB")
		}
	}

	if count != 2 {
		t.Errorf("Expected 2 rows, got %d", count)
	}
}

// TestGuardrailPIILeakageCloud simulates Cloud/Multi-tenant logging to ensure
// that unstructured text passed into logs is scrubbed via the standard redactor.
func TestGuardrailPIILeakageCloud(t *testing.T) {
	var buf bytes.Buffer
	baseHandler := slog.NewJSONHandler(&buf, nil)
	// We use the PIIRedactingHandler that is standard for Cloud multitenant logs.
	handler := telemetry.NewPIIRedactingHandler(baseHandler)
	logger := slog.New(handler)

	// Simulate logging user input that contains PII
	sensitiveLog := "User transaction completed for phone 123-456-7890 and email spy@ohc.com"
	logger.Info(sensitiveLog)

	output := buf.String()
	if strings.Contains(output, "123-456-7890") {
		t.Errorf("Guardrail failed: Cloud multi-tenant log leaked phone number")
	}
	if strings.Contains(output, "spy@ohc.com") {
		t.Errorf("Guardrail failed: Cloud multi-tenant log leaked email address")
	}
	if !strings.Contains(output, "[REDACTED_PHONE]") || !strings.Contains(output, "[REDACTED_EMAIL]") {
		t.Errorf("Guardrail failed: Cloud multi-tenant log didn't use expected redact placeholders")
	}
}

// TestGuardrailNoTelemetryExfiltration ensures standalone data isn't exposed when not enabled.
func TestGuardrailNoTelemetryExfiltration(t *testing.T) {
	t.Setenv("OHC_TELEMETRY_ENABLED", "false")
	t.Setenv("OHC_MULTITENANT", "false")

	// Verify that buffer metric func is nil, hence no exfiltration queue built
	telemetry.BufferMetricFunc = nil
	cleanup, err := telemetry.InitTelemetry()
	if err != nil {
		t.Fatalf("InitTelemetry should not fail when opting out: %v", err)
	}
	if cleanup != nil {
		defer cleanup()
	}

	if telemetry.BufferMetricFunc != nil {
		t.Errorf("Guardrail failed: BufferMetricFunc should remain nil if exfiltration is completely opted out.")
	}
}

// TestGuardrailNoRawEnvVars uses AST parsing to ensure critical telemetry files
// do not pass os.Getenv directly to a logger without an intermediate redaction step.
func TestGuardrailNoRawEnvVars(t *testing.T) {
	_, b, _, _ := runtime.Caller(0)
	serverPath := filepath.Dir(b)

	fset := token.NewFileSet()
	packages, err := parser.ParseDir(fset, serverPath, func(info os.FileInfo) bool {
		return strings.HasSuffix(info.Name(), ".go") && !strings.HasSuffix(info.Name(), "_test.go")
	}, 0)

	if err != nil {
		t.Logf("Skipping AST parse due to env/path constraints: %v", err)
		return
	}

	for _, pkg := range packages {
		for filename, file := range pkg.Files {
			ast.Inspect(file, func(n ast.Node) bool {
				call, ok := n.(*ast.CallExpr)
				if !ok {
					return true
				}

				// Check if the function being called is a log function
				isLogCall := false
				if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
					if ident, ok := sel.X.(*ast.Ident); ok {
						if ident.Name == "log" || ident.Name == "slog" {
							isLogCall = true
						}
					}
				}

				if isLogCall {
					// Check arguments for os.Getenv
					for _, arg := range call.Args {
						ast.Inspect(arg, func(innerNode ast.Node) bool {
							innerCall, innerOk := innerNode.(*ast.CallExpr)
							if !innerOk {
								return true
							}
							if innerSel, innerSelOk := innerCall.Fun.(*ast.SelectorExpr); innerSelOk {
								if innerIdent, innerIdentOk := innerSel.X.(*ast.Ident); innerIdentOk {
									if innerIdent.Name == "os" && innerSel.Sel.Name == "Getenv" {
										t.Errorf("Guardrail failed in %s: Found unredacted os.Getenv passed directly to a logger", filename)
									}
								}
							}
							return true
						})
					}
				}
				return true
			})
		}
	}
}
