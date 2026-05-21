package sandbox

import (
	"context"
	"strings"
	"testing"
	"time"
)

func TestSandboxManager_Execute_TMPDIR(t *testing.T) {
	sm, err := NewSandboxManager()
	if err != nil {
		t.Fatalf("Failed to create sandbox manager: %v", err)
	}
	defer sm.Cleanup()

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	output, err := sm.Execute(ctx, "echo $TMPDIR")
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	result := strings.TrimSpace(string(output))
	if result != sm.Dir {
		t.Errorf("Expected TMPDIR to be %s, got %s", sm.Dir, result)
	}
}

func TestSandboxManager_Execute_Shopt(t *testing.T) {
	sm, err := NewSandboxManager()
	if err != nil {
		t.Fatalf("Failed to create sandbox manager: %v", err)
	}
	defer sm.Cleanup()

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	output, err := sm.Execute(ctx, "shopt | grep extglob")
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	result := string(output)
	if !strings.Contains(result, "extglob        	off") {
		t.Errorf("Expected extglob to be off, got output: %s", result)
	}
}

func TestSandboxManager_Execute_Timeout(t *testing.T) {
	sm, err := NewSandboxManager()
	if err != nil {
		t.Fatalf("Failed to create sandbox manager: %v", err)
	}
	defer sm.Cleanup()

	ctx, cancel := context.WithTimeout(context.Background(), 1*time.Millisecond)
	defer cancel()

	_, err = sm.Execute(ctx, "sleep 1")
	if err == nil {
		t.Error("Expected error due to timeout, got nil")
	}
}
