package onboarding

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

func TestSetupAuditService(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "audit_test_*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	requiredFiles := []string{"config.yml", "profile.json"}
	service := NewSetupAuditService(requiredFiles)
	ctx := context.Background()

	// Test: Files missing
	success, err := service.VerifySetup(ctx, tempDir)
	if success || err == nil {
		t.Errorf("Expected failure for missing files, got success=%v, err=%v", success, err)
	}

	// Test: Absolute path restriction
	_, err = service.VerifySetup(ctx, "/etc")
	if err == nil {
		t.Errorf("Expected error for absolute path, got nil")
	}

	// Create required files
	for _, f := range requiredFiles {
		err := os.WriteFile(filepath.Join(tempDir, f), []byte("test"), 0644)
		if err != nil {
			t.Fatalf("failed to write test file: %v", err)
		}
	}

	// Test: Files present, note we must pass '.' since we are bounded to relative paths for safety,
	// so let's change to the tempDir
	originalWD, _ := os.Getwd()
	os.Chdir(tempDir)
	defer os.Chdir(originalWD)

	success, err = service.VerifySetup(ctx, ".")
	if !success || err != nil {
		t.Errorf("Expected success, got success=%v, err=%v", success, err)
	}

	// Test: Path escapes boundary
	_, err = service.VerifySetup(ctx, "../outside_dir")
	if err == nil {
		t.Errorf("Expected error for path escaping boundary, got nil")
	}
}
