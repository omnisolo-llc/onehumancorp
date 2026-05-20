package harness

import (
	"context"
	"strings"
	"testing"
)

func TestExecutorAllowed(t *testing.T) {
	executor := NewLocalShellTask()
	ctx := context.Background()

	out, err := executor.Execute(ctx, "echo 'hello'")
	// bwrap might not be installed in CI or local environment.
	if err != nil {
		if !strings.Contains(err.Error(), "bwrap: command not found") && !strings.Contains(err.Error(), "executable file not found") {
			t.Fatalf("Expected no error or bwrap not found, got: %v", err)
		}
	} else {
		if !strings.Contains(out, "hello") {
			t.Errorf("Expected output to contain 'hello', got: %s", out)
		}
	}
}

func TestExecutorDenied(t *testing.T) {
	executor := NewLocalShellTask()
	ctx := context.Background()

	_, err := executor.Execute(ctx, "rm -rf /")
	if err == nil {
		t.Fatalf("Expected error, got nil")
	}

	if !strings.Contains(err.Error(), "SANDBOX_FAILURE") {
		t.Errorf("Expected SANDBOX_FAILURE, got: %v", err)
	}
}
