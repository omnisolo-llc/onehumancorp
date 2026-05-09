package telemetry

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestNoPIILoggingStatements(t *testing.T) {
	var checkedFiles int
	var violations []string

	// When running via bazel, the files will be in runfiles directory under workspace root
	// The bazel BUILD file exposes srcs/server/**/*.go via the all_go_files filegroup
	searchRoot := "srcs/server"
	if workspaceDir := os.Getenv("BUILD_WORKSPACE_DIRECTORY"); workspaceDir != "" {
		searchRoot = filepath.Join(workspaceDir, "srcs/server")
	} else if runfilesDir := os.Getenv("RUNFILES_DIR"); runfilesDir != "" {
		searchRoot = filepath.Join(runfilesDir, "ohc/srcs/server")
		if _, err := os.Stat(searchRoot); os.IsNotExist(err) {
			searchRoot = filepath.Join(runfilesDir, "_main/srcs/server")
		}
	} else if _, err := os.Stat("../../server"); err == nil {
		searchRoot = "../../server"
	}

	err := filepath.Walk(searchRoot, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if info.IsDir() {
			return nil
		}
		ext := filepath.Ext(path)
		if ext != ".go" {
			return nil
		}
		// Exclude test files testing this specifically, or known files where we do redaction and we expect "pii" keyword in docs
		if strings.Contains(path, "pii_log_test.go") || strings.Contains(path, "pii_test.go") || strings.Contains(path, "pii.go") || strings.Contains(path, "buffer_integration.go") || strings.Contains(path, "local_buffer_test.go") || strings.Contains(path, "mcp/client.go") || strings.Contains(path, "telemetry_test.rs") {
			return nil
		}

		contentBytes, err := os.ReadFile(path)
		if err != nil {
			return nil
		}

		lines := strings.Split(string(contentBytes), "\n")
		for i, line := range lines {
			lowerLine := strings.ToLower(line)
			if strings.Contains(lowerLine, "log.print") ||
				strings.Contains(lowerLine, "fmt.errorf") ||
				strings.Contains(lowerLine, "fmt.error") ||
				strings.Contains(lowerLine, "log.printf") ||
				strings.Contains(lowerLine, "fmt.print") {

				if strings.Contains(lowerLine, "tenant_id") ||
					strings.Contains(lowerLine, "organization_id") ||
					strings.Contains(lowerLine, "org_id") ||
					strings.Contains(lowerLine, "session_data") ||
					strings.Contains(lowerLine, "session_id") ||
					strings.Contains(lowerLine, "payload") ||
					strings.Contains(lowerLine, "email") ||
					strings.Contains(lowerLine, "password") ||
					strings.Contains(lowerLine, "pii") ||
					strings.Contains(lowerLine, "api_key") ||
					strings.Contains(lowerLine, "secret_key") ||
					strings.Contains(lowerLine, "credit") ||
					strings.Contains(lowerLine, "card") ||
					strings.Contains(lowerLine, "cvv") ||
					strings.Contains(lowerLine, "dob") ||
					strings.Contains(lowerLine, "birth") ||
					strings.Contains(lowerLine, "passport") ||
					strings.Contains(lowerLine, "bank") ||
					strings.Contains(lowerLine, "account") ||
					strings.Contains(lowerLine, "stripe") ||
					strings.Contains(lowerLine, "billing") ||
					strings.Contains(lowerLine, "ip_address") ||
					strings.Contains(lowerLine, "mac_address") ||
					strings.Contains(lowerLine, "geolocation") {
					violations = append(violations, fmt.Sprintf("%s:%d: %s", path, i+1, strings.TrimSpace(line)))
				}
			}
		}
		checkedFiles++
		return nil
	})

	if err != nil {
		t.Fatalf("Failed to walk codebase: %v", err)
	}

	if checkedFiles < 5 {
		t.Fatalf("Could not find enough .go files to run PII leakage test. Checked: %d", checkedFiles)
	}

	if len(violations) > 0 {
		t.Fatalf("Found PII logging violations in the following lines:\n%v", strings.Join(violations, "\n"))
	}
}
