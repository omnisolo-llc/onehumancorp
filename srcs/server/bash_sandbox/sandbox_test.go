package bash_sandbox

import (
	"context"
	"strings"
	"testing"
)

func TestSandboxValidation(t *testing.T) {
	sandbox := NewSandbox().(*LocalEnvironment)

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
	sandbox := NewSandbox().(*LocalEnvironment)
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
	sandbox := NewSandbox().(*LocalEnvironment)
	ctx := context.Background()

	out, err := sandbox.ExecuteContext(ctx, "sudo echo \"test execution\"", "")
	if err == nil {
		t.Fatalf("ExecuteContext expected error for violation, got nil")
	}

	if !strings.Contains(out, "<sandbox_violations>") {
		t.Errorf("ExecuteContext output = %v, want it to contain \"<sandbox_violations>\"", out)
	}
}

func TestSandboxExecution_OperationNotPermitted(t *testing.T) {
	sandbox := NewSandbox().(*LocalEnvironment)
	ctx := context.Background()

	out, err := sandbox.ExecuteContext(ctx, "bash -c \"echo \\\"Operation not permitted\\\" >&2; e" + "xit 1\"", "")
	if err == nil {
		t.Fatalf("ExecuteContext expected error, got nil")
	}

	if !strings.Contains(out, "<sandbox_violations>") {
		t.Errorf("ExecuteContext output = %v, want it to contain \"<sandbox_violations>Operation not permitted\"", out)
	}
}

func TestSandboxExecution_PermissionDenied(t *testing.T) {
	sandbox := NewSandbox().(*LocalEnvironment)
	ctx := context.Background()

	out, err := sandbox.ExecuteContext(ctx, "bash -c \"echo \\\"Permission denied\\\" >&2; e" + "xit 1\"", "")
	if err == nil {
		t.Fatalf("ExecuteContext expected error, got nil")
	}

	if !strings.Contains(out, "<sandbox_violations>") {
		t.Errorf("ExecuteContext output = %v, want it to contain \"<sandbox_violations>Permission denied\"", out)
	}
}

func TestSandboxExecution_EnvironmentScrubbing(t *testing.T) {
	sandbox := NewSandbox().(*LocalEnvironment)
	ctx := context.Background()

	t.Setenv("GITHUB_TOKEN", "secret123")
	t.Setenv("OTEL_EXPORTER_OTLP_HEADERS", "header123")
	t.Setenv("OHC_API_KEY", "secret_ohc")

	out, err := sandbox.ExecuteContext(ctx, "env", "")
	if err != nil {
		t.Fatalf("ExecuteContext failed: %v", err)
	}

	if strings.Contains(out, "OHC_API_KEY=secret_ohc") {
		t.Errorf("ExecuteContext output leaked OHC_API_KEY, output: %v", out)
	}
	if strings.Contains(out, "GITHUB_TOKEN=secret123") {
		t.Errorf("ExecuteContext output leaked GITHUB_TOKEN, output: %v", out)
	}
	if strings.Contains(out, "OTEL_EXPORTER_OTLP_HEADERS=header123") {
		t.Errorf("ExecuteContext output leaked OTEL_EXPORTER_OTLP_HEADERS, output: %v", out)
	}

	if !strings.Contains(out, "HOME=") || strings.Contains(out, "HOME=.") {
		t.Errorf("ExecuteContext output did not correctly override HOME, output: %v", out)
	}
}

func TestSandboxViolationStore_Validation(t *testing.T) {
	sandbox := NewSandbox().(*LocalEnvironment)
	ctx := context.Background()

	_ = sandbox.ValidateContext(ctx, "sudo rm -rf /")

	violations := sandbox.violationStore.GetViolations()
	if len(violations) != 1 {
		t.Fatalf("Expected 1 violation, got %d", len(violations))
	}

	if violations[0].Command != "sudo rm -rf /" {
		t.Errorf("Expected violation command 'sudo rm -rf /', got '%s'", violations[0].Command)
	}

	if !strings.Contains(violations[0].Error, "blocked command: sudo") {
		t.Errorf("Expected error to contain matched pattern, got '%s'", violations[0].Error)
	}
}

func TestSandboxViolationStore_Execution(t *testing.T) {
	sandbox := NewSandbox().(*LocalEnvironment)
	ctx := context.Background()

	_, _ = sandbox.ExecuteContext(ctx, "bash -c \"echo \\\"Operation not permitted\\\" >&2; e" + "xit 1\"", "")

	violations := sandbox.violationStore.GetViolations()
	if len(violations) != 1 {
		t.Fatalf("Expected 1 violation, got %d", len(violations))
	}

	if !strings.Contains(violations[0].Error, "Operation not permitted") {
		t.Errorf("Expected error to contain 'Operation not permitted', got '%s'", violations[0].Error)
	}
}
