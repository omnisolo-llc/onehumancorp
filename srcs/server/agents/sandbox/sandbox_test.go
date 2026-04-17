package sandbox

import (
	"context"
	"strings"
	"testing"
	"time"
)

func TestSandboxManager_EnvAndDir(t *testing.T) {
	sm, err := NewSandboxManager()
	if err != nil {
		t.Fatalf("Failed to create SandboxManager: %v", err)
	}
	defer sm.Cleanup()

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	out, err := sm.Execute(ctx, "echo $TMPDIR", "")
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	expectedTmpdir := sm.SandboxDir
	actualTmpdir := strings.TrimSpace(out)
	if actualTmpdir != expectedTmpdir {
		t.Errorf("Expected TMPDIR '%s', got '%s'", expectedTmpdir, actualTmpdir)
	}

	out, err = sm.Execute(ctx, "pwd", "")
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	actualPwd := strings.TrimSpace(out)
	if !strings.HasSuffix(actualPwd, expectedTmpdir) && !strings.HasSuffix(expectedTmpdir, actualPwd) {
		t.Errorf("Expected dir '%s', got '%s'", expectedTmpdir, actualPwd)
	}
}

func TestSandboxManager_Shopt(t *testing.T) {
	sm, err := NewSandboxManager()
	if err != nil {
		t.Fatalf("Failed to create SandboxManager: %v", err)
	}
	defer sm.Cleanup()

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	out, err := sm.Execute(ctx, "shopt extglob", "")
	if err != nil && !strings.Contains(err.Error(), "exit status 1") {
	}
	if !strings.Contains(out, "off") {
		t.Errorf("Expected extglob to be off, got: %s", out)
	}
}

func TestSandboxManager_Validation(t *testing.T) {
	sm, err := NewSandboxManager()
	if err != nil {
		t.Fatalf("Failed to create SandboxManager: %v", err)
	}
	defer sm.Cleanup()

	tests := []struct {
		name    string
		command string
		wantErr bool
	}{
		{"safe command", "ls -la", false},
		{"sudo command", "sudo rm -rf /", true},
		{"rm root", "rm -rf /", true},
		{"chown command", "chown root:root file.txt", true},
		{"process substitution read", "cat <(ls)", true},
		{"git hooks injection", "echo 'rm -rf /' > .git/hooks/pre-commit", true},
		{"git config injection", "echo '[core]' > .git/config", true},
	}

	ctx := context.Background()
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := sm.ValidateContext(ctx, tt.command)
			if (err != nil) != tt.wantErr {
				t.Errorf("ValidateContext() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}

func TestSandboxManager_ExecutionViolation_Output(t *testing.T) {
	sm, err := NewSandboxManager()
	if err != nil {
		t.Fatalf("Failed to create SandboxManager: %v", err)
	}
	defer sm.Cleanup()

	ctx := context.Background()

	out, err := sm.Execute(ctx, "sudo echo \"test execution\"", "")
	if err == nil {
		t.Fatalf("Execute expected error for violation, got nil")
	}

	if !strings.Contains(out, "<sandbox_violations>") {
		t.Errorf("Execute output = %v, want it to contain \"<sandbox_violations>\"", out)
	}
}

func TestSandboxManager_Timeout(t *testing.T) {
	sm, err := NewSandboxManager()
	if err != nil {
		t.Fatalf("Failed to create SandboxManager: %v", err)
	}
	defer sm.Cleanup()

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Millisecond)
	defer cancel()

	_, err = sm.Execute(ctx, "sleep 1", "")
	if err == nil {
		t.Fatalf("Expected timeout error, got nil")
	}
}

func TestSandboxManager_EnvironmentScrubbing(t *testing.T) {
	sm, err := NewSandboxManager()
	if err != nil {
		t.Fatalf("Failed to create SandboxManager: %v", err)
	}
	defer sm.Cleanup()
	ctx := context.Background()

	out, err := sm.Execute(ctx, "env", "")
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	if !strings.Contains(out, "TMPDIR="+sm.SandboxDir) {
		t.Errorf("ExecuteContext output did not override TMPDIR, output: %v", out)
	}
}
