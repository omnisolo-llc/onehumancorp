package sandbox

import (
	"context"
	"strings"
	"testing"
	"time"
)

// added for issue 5417
func TestSandboxManager_EnvAndDir(t *testing.T) {
	sm, err := NewSandboxManager("test-session-1")
	if err != nil {
		t.Fatalf("Failed to create SandboxManager: %v", err)
	}
	defer sm.Cleanup()

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	out, err := sm.Execute(ctx, "echo $TMPDIR")
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	expectedTmpdir := sm.SandboxDir
	actualTmpdir := strings.TrimSpace(out)
	if actualTmpdir != expectedTmpdir {
		t.Errorf("Expected TMPDIR '%s', got '%s'", expectedTmpdir, actualTmpdir)
	}

	out, err = sm.Execute(ctx, "pwd")
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	actualPwd := strings.TrimSpace(out)
	// pwd might return path starting with /private on macos, resolve it if necessary or just check suffix
	if !strings.HasSuffix(actualPwd, expectedTmpdir) && !strings.HasSuffix(expectedTmpdir, actualPwd) {
		t.Errorf("Expected dir '%s', got '%s'", expectedTmpdir, actualPwd)
	}
}

func TestSandboxManager_Shopt(t *testing.T) {
	sm, err := NewSandboxManager("test-session-2")
	if err != nil {
		t.Fatalf("Failed to create SandboxManager: %v", err)
	}
	defer sm.Cleanup()

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	out, err := sm.Execute(ctx, "shopt extglob")
	// usually output is "extglob        	off"
	if err != nil && !strings.Contains(err.Error(), "exit status 1") { // shopt extglob might return 1 if disabled
		// ignore status 1, check output
	}
	if !strings.Contains(out, "off") {
		t.Errorf("Expected extglob to be off, got: %s", out)
	}
}
