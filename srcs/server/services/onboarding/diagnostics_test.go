package onboarding

import (
	"os"
	"path/filepath"
	"testing"
)

func TestRunDiagnostics(t *testing.T) {
	// Create temporary directories to simulate the agent-task structure
	tempDir := t.TempDir()
	t.Setenv("OHC_RUNTIME_DIR", filepath.Join(tempDir, ".ohc", "runtime"))
	t.Setenv("OHC_MEMORY_DIR", filepath.Join(tempDir, ".ohc", "runtime", "memory"))
	t.Setenv("OHC_STATUS_DIR", filepath.Join(tempDir, ".ohc", "runtime", "status"))

	// Scenario 1: All paths are missing
	res := RunDiagnostics()
	if res.Passed {
		t.Errorf("expected diagnostics to fail when paths are missing")
	}
	if len(res.Details) != 3 {
		t.Errorf("expected 3 details, got %d", len(res.Details))
	}

	// Create required paths
	requiredPaths := []string{
		filepath.Join(tempDir, ".ohc", "runtime"),
		filepath.Join(tempDir, ".ohc", "runtime", "memory"),
		filepath.Join(tempDir, ".ohc", "runtime", "status"),
	}

	for _, p := range requiredPaths {
		if err := os.MkdirAll(p, 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
	}

	// Scenario 2: All paths exist
	res = RunDiagnostics()
	if !res.Passed {
		t.Errorf("expected diagnostics to pass when all paths exist")
	}
	if len(res.Details) != 3 {
		t.Errorf("expected 3 details, got %d", len(res.Details))
	}
}
