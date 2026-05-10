package orchestration

import (
    "testing"
)

func TestHarnessResolver(t *testing.T) {
    resolver := NewHarnessResolver()
    harness, err := resolver.Resolve("test-agent")
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }

    res, err := harness.RunAttempt("ls")
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if res.Stdout != "mock stdout" {
        t.Errorf("expected 'mock stdout', got %s", res.Stdout)
    }
}
