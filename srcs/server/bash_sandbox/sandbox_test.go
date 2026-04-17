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
		{"echo command", "echo \"hello world\"", false},
		{"sudo command", "sudo rm -rf /", true},
		{"rm root", "rm -rf /", true},
		{"chown command", "chown root:root file.txt", true},
		{"chmod command", "chmod 777 file.txt", true},
		{"process substitution read", "cat <(ls)", true},
		{"process substitution write", "echo \"hi\" >(cat)", true},
		{"zsh expansion", "cat =(ls)", true},
		{"git hooks escape", "mkdir -p .git/hooks && echo \"evil\" > .git/hooks/pre-commit && git status", true},
		{"git hooks direct write", "echo \"evil\" > .git/hooks/pre-push", true},
		{"git HEAD direct write", "echo \"main\" > .git/HEAD", true},
		{"git safe command", "git status", false},
		{"git log safe", "git log -n 5", false},
		{"git internal path mentioned but no git command", "echo .git/hooks is a sensitive directory", false},
		{"bypass attempt with env var", "P=.git; echo evil > $P/hooks/pre-commit", false},
		{"bypass attempt with alternative path", "echo evil > ./.git/hooks/pre-push", true},
		{"multi-command attack", "echo evil > .git/hooks/pre-commit; git status", true},
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

	out, err := sandbox.ExecuteContext(ctx, "echo \"test execution\"", "")
	if err != nil {
		t.Fatalf("ExecuteContext failed: %v", err)
	}

	if !strings.Contains(out, "test execution") {
		t.Errorf("ExecuteContext output = %v, want it to contain \"test execution\"", out)
	}
}

func TestSandboxExecutionViolation_Output(t *testing.T) {
	sandbox := NewSandbox()
	ctx := context.Background()

	out, err := sandbox.ExecuteContext(ctx, "sudo echo \"test execution\"", "")
	if err == nil {
		t.Fatalf("ExecuteContext expected error for violation, got nil")
	}

	if !strings.Contains(out, "<sandbox_violations>") {
		t.Errorf("ExecuteContext output = %v, want it to contain \"<sandbox_violations>\"", out)
	}
}

func TestSandboxExecution_EnvironmentScrubbing(t *testing.T) {
	sandbox := NewSandbox()
	ctx := context.Background()

	t.Setenv("GITHUB_TOKEN", "secret123")
	t.Setenv("OTEL_EXPORTER_OTLP_HEADERS", "header123")

	out, err := sandbox.ExecuteContext(ctx, "env", "")
	if err != nil {
		t.Fatalf("ExecuteContext failed: %v", err)
	}

	if strings.Contains(out, "GITHUB_TOKEN=secret123") {
		t.Errorf("ExecuteContext output leaked GITHUB_TOKEN, output: %v", out)
	}
	if strings.Contains(out, "OTEL_EXPORTER_OTLP_HEADERS=header123") {
		t.Errorf("ExecuteContext output leaked OTEL_EXPORTER_OTLP_HEADERS, output: %v", out)
	}

	if !strings.Contains(out, "HOME=.agent-home/") {
		t.Errorf("ExecuteContext output did not override HOME, output: %v", out)
	}
}
