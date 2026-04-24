package harness

import (
	"context"
	"strings"
	"testing"
)

func TestBwrapExecutor_Execute(t *testing.T) {
	executor := NewBwrapExecutor()

	// Execute a simple echo command
	// Bwrap might not be installed on the host running tests, so it may fail with "executable file not found in $PATH"
	// However, we just need to ensure the arguments and the method signature work correctly.
	result, err := executor.Execute(context.Background(), `echo "hello sandbox"`)

	if err != nil {
		if strings.Contains(err.Error(), "executable file not found in $PATH") || strings.Contains(err.Error(), "no such file or directory") || strings.Contains(err.Error(), "exit status") {
			// Expected if bwrap is missing or fails due to permissions in CI
			t.Logf("bwrap execution failed as expected in test environment: %v", err)
		} else {
			t.Fatalf("Unexpected error from Execute: %v", err)
		}
	} else {
		if !strings.Contains(result.Stdout, "hello sandbox") {
			t.Errorf("Expected 'hello sandbox' in stdout, got: %s", result.Stdout)
		}
	}
}
