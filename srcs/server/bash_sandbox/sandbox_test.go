package bash_sandbox

import (
	"context"
	"strings"
	"testing"
)

func TestSandboxValidation(t *testing.T) {
	sandbox := NewSandbox()

	tests := []struct {
		name    string
		command string
		wantErr bool
	}{
		{"safe command", "ls -la", false},
		{"echo command", "echo 'hello world'", false},
		{"sudo command", "sudo rm -rf /", true},
		{"rm root", "rm -rf /", true},
		{"chown command", "chown root:root file.txt", true},
		{"chmod command", "chmod 777 file.txt", true},
		{"process substitution read", "cat <(ls)", true},
		{"process substitution write", "echo 'hi' >(cat)", true},
		{"zsh expansion", "cat =(ls)", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := sandbox.ValidateContext(context.Background(), tt.command)
			if (err != nil) != tt.wantErr {
				t.Errorf("Validate() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}

func TestSandboxExecution(t *testing.T) {
	sandbox := NewSandbox()
	ctx := context.Background()

	out, err := sandbox.ExecuteContext(ctx, "echo 'test execution'", "")
	if err != nil {
		t.Fatalf("ExecuteContext failed: %v", err)
	}

	if !strings.Contains(out, "test execution") {
		t.Errorf("ExecuteContext output = %v, want it to contain 'test execution'", out)
	}
}

func TestSandboxExecutionViolation(t *testing.T) {
	sandbox := NewSandbox()
	ctx := context.Background()

	_, err := sandbox.ExecuteContext(ctx, "sudo echo 'test execution'", "")
	if err == nil {
		t.Fatalf("ExecuteContext expected error for violation, got nil")
	}

	if !strings.Contains(err.Error(), "security policy") {
		t.Errorf("ExecuteContext error = %v, want it to contain 'security policy'", err)
	}
}

func TestSandboxExecutionExplicitDeny(t *testing.T) {
	sandbox := NewSandbox()
	ctx := context.Background()

	// Simulate an operation that hits a sandbox boundary (e.g., trying to write to a protected file)
	// We'll use a mock command that returns a "Permission denied" error
	out, err := sandbox.ExecuteContext(ctx, `bash -c 'echo test > /root/protected.txt 2>&1 || (echo "Permission denied" >&2; bash -c "exit 1")'`, "")
	if err == nil {
		t.Fatalf("ExecuteContext expected error for boundary drop, got nil")
	}

	if !strings.Contains(out, "<sandbox_violations>") {
		t.Errorf("ExecuteContext output = %v, want it to contain '<sandbox_violations>' block for sandbox drop", out)
	}
}
