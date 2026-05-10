package telemetry

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestCheckForPIILock(t *testing.T) {
	safePayload := AuditPayload{
		"event_name": "click",
		"author":     "home",
		"discard":    "yes",
		"keyboard":   "abc",
	}
	if err := CheckForPIILock(safePayload); err != nil {
		t.Errorf("expected no error for safe payload, got: %v", err)
	}

	unsafePayload := AuditPayload{
		"user_email": "test@test.com",
	}
	if err := CheckForPIILock(unsafePayload); err == nil {
		t.Errorf("expected error for unsafe payload, got nil")
	}
}

func TestCheckMultiTenantContext(t *testing.T) {
	ctx := context.WithValue(context.Background(), TenantContextKey, "tenant-a")
	if err := CheckMultiTenantContext(ctx, "tenant-a"); err != nil {
		t.Errorf("expected no error when tenants match, got: %v", err)
	}

	if err := CheckMultiTenantContext(ctx, "tenant-b"); err == nil {
		t.Errorf("expected error for cross-tenant mismatch, got nil")
	}
}

func TestStandaloneWrapperAudit(t *testing.T) {
	// Locate file under Bazel test sandbox
	scriptPath := os.Getenv("TEST_WORKSPACE")
	if scriptPath == "" {
		// fallback to relative for native go testing
		scriptPath = filepath.Join("..", "..", "..", "deploy", "scripts", "ohc-standalone.sh")
	} else {
		scriptPath = filepath.Join(os.Getenv("RUNFILES_DIR"), scriptPath, "deploy", "scripts", "ohc-standalone.sh")
	}

	contentBytes, err := os.ReadFile(scriptPath)
	if err != nil {
		t.Skipf("Failed to read ohc-standalone.sh at path %s: %v", scriptPath, err)
	}
	content := string(contentBytes)

	// In test, look for specific logic rather than exact multiline string formatting
	if !strings.Contains(content, "export OHC_TELEMETRY_ENABLED") {
		t.Errorf("Local Sovereignty violation: ohc-standalone.sh does not properly strictly enforce OHC_TELEMETRY_ENABLED opt-in boundary.")
	}
	if !strings.Contains(content, "OHC_TELEMETRY_ENABLED=false") {
		t.Errorf("Local Sovereignty violation: ohc-standalone.sh does not properly strictly enforce OHC_TELEMETRY_ENABLED opt-in boundary.")
	}
}
