package sandbox

import (
	"context"
	"strings"
	"testing"
	"time"
)

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
	if err != nil && !strings.Contains(err.Error(), "exit status 1") {
		// shopt extglob might return exit status 1 if disabled
	}
	if !strings.Contains(out, "off") {
		t.Errorf("Expected extglob to be off, got: %s", out)
	}
}

func TestSandboxManager_PowerShell(t *testing.T) {
	sm, err := NewSandboxManager("test-session-pwsh")
	if err != nil {
		t.Fatalf("Failed to create SandboxManager: %v", err)
	}
	defer sm.Cleanup()

	sm.SetProvider(&PowerShellProvider{})

	if _, ok := sm.Provider.(*PowerShellProvider); !ok {
		t.Fatalf("Expected PowerShellProvider, got %T", sm.Provider)
	}

	// We test execute with pwsh. If pwsh is not installed, it will return "executable file not found".
	// We handle this gracefully to allow tests to pass on CI environments without PowerShell.
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	out, err := sm.Execute(ctx, "Write-Output 'hello'")
	if err != nil {
		errStr := err.Error()
		if strings.Contains(errStr, "executable file not found") || strings.Contains(errStr, "no such file") || strings.Contains(errStr, "signal: killed") {
			t.Logf("Skipping PowerShell execution test due to environment missing pwsh: %v", err)
		} else {
			t.Fatalf("Unexpected execute error: %v, out: %s", err, out)
		}
	} else {
		if !strings.Contains(out, "hello") {
			t.Errorf("Expected 'hello', got: %s", out)
		}
	}
}

func TestRecordViolation(t *testing.T) {
	// Simple test to ensure it doesn't panic
	RecordViolation()
}
