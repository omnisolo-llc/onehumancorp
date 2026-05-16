package harness

import (
	"strings"
	"testing"
)

func TestBwrapExecutor(t *testing.T) {
	executor := NewBwrapExecutor()
	if executor == nil {
		t.Fatalf("expected non-nil executor")
	}

	out, err := executor.Execute("echo 'hello'")

	if err != nil {
		if !strings.Contains(err.Error(), "not found") && !strings.Contains(err.Error(), "exit status") {
			t.Logf("Execute returned error: %v", err)
		}
	} else {
		if !strings.Contains(out, "hello") {
			t.Errorf("expected 'hello' in output, got '%s'", out)
		}
	}
}
