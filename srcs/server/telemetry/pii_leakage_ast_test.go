package telemetry

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"testing"
)

// List of sensitive PII keys we shouldn't log
var sensitiveKeys = []string{
	"tenant_id",
	"organization_id",
	"org_id",
	"session_data",
	"session_id",
	"token",
	"email",
	"password",
	"pii",
	"api_key",
	"secret_key",
	"credit",
	"card",
	"cvv",
	"dob",
	"birth",
	"passport",
	"bank",
	"account",
	"stripe",
	"billing",
	"ip_address",
	"mac_address",
	"geolocation",
}

func isLoggingCall(sel *ast.SelectorExpr) bool {
	if ident, ok := sel.X.(*ast.Ident); ok {
		pkg := ident.Name
		fn := sel.Sel.Name
		return (pkg == "log" && (fn == "Printf" || fn == "Println" || fn == "Print" || fn == "Fatalf" || fn == "Fatalln" || fn == "Fatal")) ||
			(pkg == "fmt" && (fn == "Printf" || fn == "Println" || fn == "Print" || fn == "Errorf")) ||
			(pkg == "tracing" && (fn == "Info" || fn == "Error" || fn == "Warn" || fn == "Debug" || fn == "Trace"))
	}
	return false
}

// isViolation determines if a logging string contains a sensitive key in an unsafe way
// e.g. "tenant_id: %s" is unsafe, but "failed to fetch tenant usage" is safe if we don't ban valid english words.
func isViolation(val, key string) bool {
	// Simple substrings can be too aggressive (e.g. "token" in "broken").
	// We want to avoid naive regex that bans standard terms or explicitly safe strings like "[REDACTED]".

	if strings.Contains(val, "[redacted]") {
		return false
	}

	// Create a word boundary check for the key
	pattern := fmt.Sprintf(`\b%s\b`, regexp.QuoteMeta(key))
	matched, _ := regexp.MatchString(pattern, val)
	if !matched {
		return false
	}

	hasFormat := strings.Contains(val, "%") || strings.Contains(val, "{}")
	if matched && hasFormat {
		valueInjectionPattern := fmt.Sprintf(`\b%s\b\s*[:=]\s*(%%|{})`, regexp.QuoteMeta(key))
		valueInjectionMatched, _ := regexp.MatchString(valueInjectionPattern, val)
		return valueInjectionMatched
	}

	return false
}

func TestNoPIILoggingStatements(t *testing.T) {
	// Root directory to scan (assuming tests run in srcs/server or its subpackages)
	rootDir := ".." // From srcs/server/telemetry to srcs/server

	// Fallback if not run from telemetry dir
	if _, err := os.Stat("main.go"); err == nil {
		rootDir = "."
	}

	fset := token.NewFileSet()
	var violations []string

	err := filepath.Walk(rootDir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() || !strings.HasSuffix(path, ".go") {
			return nil
		}

		// Skip this test file itself to avoid self-detection
		if strings.HasSuffix(path, "pii_leakage_ast_test.go") {
			return nil
		}

		node, err := parser.ParseFile(fset, path, nil, 0)
		if err != nil {
			return err // Not a fatal error for the test suite, could just be unparseable code, but we report it.
		}

		ast.Inspect(node, func(n ast.Node) bool {
			if call, ok := n.(*ast.CallExpr); ok {
				if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
					if isLoggingCall(sel) {
						// Check the arguments
						for _, arg := range call.Args {
							if basicLit, ok := arg.(*ast.BasicLit); ok && basicLit.Kind == token.STRING {
								val := strings.ToLower(basicLit.Value)
								for _, key := range sensitiveKeys {
									if isViolation(val, key) {
										pos := fset.Position(call.Pos())
										// Adjust the filename relative to the workspace to match standard Bazel expectation or keep absolute
										relPath := pos.Filename
										if pwd, err := os.Getwd(); err == nil {
											if rel, err := filepath.Rel(pwd, pos.Filename); err == nil {
												relPath = rel
											}
										}
										violations = append(violations, fmt.Sprintf("%s:%d: Logging call contains PII key '%s'", relPath, pos.Line, key))
										break // Only report one violation per logging call
									}
								}
							}
						}
					}
				}
			}
			return true
		})

		return nil
	})

	if err != nil {
		t.Fatalf("Failed to walk directory: %v", err)
	}

	if len(violations) > 0 {
		t.Errorf("Found %d PII logging violations:\n%s", len(violations), strings.Join(violations, "\n"))
	}
}
