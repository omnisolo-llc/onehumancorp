package harness

import (
	"context"
	"strings"
	"testing"
)

func TestViolations(t *testing.T) {
	store := NewViolationStore()
	store.RecordViolation(context.Background(), "ls", "permission denied")
	v := store.GetViolations()
	if len(v) != 1 || v[0].Command != "ls" {
		t.Errorf("expected 1 violation for 'ls', got %v", v)
	}
	annotated := AnnotateStderrWithSandboxFailures("err", v)
	if !strings.Contains(annotated, "<sandbox_violations>") || !strings.Contains(annotated, "ls: permission denied") {
		t.Errorf("annotated output is incorrect: %s", annotated)
	}
}
