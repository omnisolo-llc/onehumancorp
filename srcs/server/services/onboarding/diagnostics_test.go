package onboarding

import (
	"os"
	"path/filepath"
	"testing"
)

func TestRunDiagnostics(t *testing.T) {
	// Create temporary directories to simulate the agent-task structure
	cwd, err := os.Getwd()
	if err != nil {
		t.Fatalf("failed to get cwd: %v", err)
	}

	tempDir := t.TempDir()

	// Change working directory to tempDir so that relative paths work
	if err := os.Chdir(tempDir); err != nil {
		t.Fatalf("failed to change directory to tempDir: %v", err)
	}

	// Ensure we restore the working directory after the test
	defer func() {
		if err := os.Chdir(cwd); err != nil {
			t.Fatalf("failed to restore working directory: %v", err)
		}
	}()

	// Scenario 1: All paths are missing
	res := RunDiagnostics(false) // standalone mode
	if res.Passed {
		t.Errorf("expected diagnostics to fail when paths are missing")
	}
	if len(res.Details) != 3 {
		t.Errorf("expected 3 details, got %d", len(res.Details))
	}

	// Create required paths for standalone mode
	requiredPaths := []string{
		filepath.Join(".ohc-local-data", "db"),
		filepath.Join(".ohc-local-data", "blob"),
		filepath.Join(".ohc-local-data", "config"),
	}

	for _, p := range requiredPaths {
		if err := os.MkdirAll(p, 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
	}

	// Scenario 2: All paths exist for standalone mode
	res = RunDiagnostics(false)
	if !res.Passed {
		t.Errorf("expected diagnostics to pass when all paths exist")
	}
	if len(res.Details) != 3 {
		t.Errorf("expected 3 details, got %d", len(res.Details))
	}

    // Scenario 3: All paths missing for cloud mode
    res = RunDiagnostics(true)
	if res.Passed {
		t.Errorf("expected diagnostics to fail when paths are missing for cloud")
	}

    // Create required paths for cloud mode
	cloudPaths := []string{
		filepath.Join(".ohc-cloud-data", "db"),
		filepath.Join(".ohc-cloud-data", "blob"),
		filepath.Join(".ohc-cloud-data", "config"),
	}

	for _, p := range cloudPaths {
		if err := os.MkdirAll(p, 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
	}

    // Scenario 4: All paths exist for cloud mode
	res = RunDiagnostics(true)
	if !res.Passed {
		t.Errorf("expected diagnostics to pass when all paths exist for cloud")
	}
}
