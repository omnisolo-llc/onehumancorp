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
	sm, err := NewSandboxManager()
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

	out, err := sm.Execute(ctx, "sudo echo \"test execution\"")
	if err == nil {
		t.Fatalf("Execute expected error for violation, got nil")
	}

	if !strings.Contains(out, "<sandbox_violations>") {
		t.Errorf("Execute output = %v, want it to contain \"<sandbox_violations>\"", out)
	}
}
