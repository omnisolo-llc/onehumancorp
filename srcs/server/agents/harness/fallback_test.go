//go:build !linux && !darwin

package harness

import (
	"context"
	"strings"
	"testing"
)

func TestFallbackHarness_Coverage(t *testing.T) {
	h := NewIsolationHarness()

	ctx := context.Background()
	execCtx := ExecutionContext{
		Command:      []string{"echo", "test"},
		AllowedPaths: []string{"/tmp"},
	}

	out, err := h.Execute(ctx, execCtx)

	if err != nil {
		t.Logf("fallback command failed: %v", err)
	} else {
		if !strings.Contains(string(out), "test") {
			t.Errorf("expected 'test' in output, got: %s", string(out))
		}
	}
}
