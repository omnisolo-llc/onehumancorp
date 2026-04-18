//go:build darwin

package harness

import (
	"context"
	"strings"
	"testing"
)

func TestSandboxHarness_Coverage(t *testing.T) {
	h := NewIsolationHarness()

	ctx := context.Background()
	execCtx := ExecutionContext{
		Command:      []string{"echo", "test"},
		AllowedPaths: []string{"/tmp"},
	}

	out, err := h.Execute(ctx, execCtx)

	if err != nil {
		if strings.Contains(err.Error(), "not found") {
			t.Logf("sandbox-exec not found, skipping deep validation: %v", err)
			return
		}
	} else {
		if !strings.Contains(string(out), "test") {
			t.Errorf("expected 'test' in output, got: %s", string(out))
		}
	}
}
