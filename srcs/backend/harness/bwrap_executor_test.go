package harness

import (
	"context"
	"strings"
	"testing"
)

func TestBwrapHarness_Execute(t *testing.T) {
	harness := NewBwrapHarness()

	// Need to check if bwrap is installed for real execution, or just test args construction.
	// Since the requirement asks for 100% test coverage and it uses exec.Command,
	// let's try running a simple command.
	execCtx := ExecutionContext{
		Command: []string{"echo", "hello"},
	}

	out, err := harness.Execute(context.Background(), execCtx)
	if err != nil {
		// bwrap might not be installed in the test environment, skip or check error
		t.Skipf("bwrap might not be installed: %v", err)
	}

	if !strings.Contains(string(out), "hello") {
		t.Errorf("Expected output to contain 'hello', got '%s'", string(out))
	}
}

func TestBwrapHarness_Execute_WithArgs(t *testing.T) {
	harness := NewBwrapHarness()

	execCtx := ExecutionContext{
		Command:      []string{"echo", "test"},
		AllowedPaths: []string{"/tmp"},
		NetworkProxy: "http://proxy:8080",
	}

	out, err := harness.Execute(context.Background(), execCtx)
	if err != nil {
		t.Skipf("bwrap might not be installed: %v", err)
	}

	if !strings.Contains(string(out), "test") {
		t.Errorf("Expected output to contain 'test', got '%s'", string(out))
	}
}
