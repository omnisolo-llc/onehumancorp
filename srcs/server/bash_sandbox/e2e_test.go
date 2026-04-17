package bash_sandbox

import (
	"context"
	"strings"
	"testing"
)

func TestE2E_GitHookProtection(t *testing.T) {
	sandbox := NewSandbox()
	ctx := context.Background()

	tests := []struct {
		name    string
		command string
		wantViolation bool
	}{
		{
			name:    "Happy Path: Safe Git Command",
			command: "git status",
			wantViolation: false,
		},
		{
			name:    "Happy Path: Safe File Write",
			command: "echo 'hello' > hello.txt",
			wantViolation: false,
		},
		{
			name:    "Attack: Git Hook Injection via redirection",
			command: "echo 'evil' > .git/hooks/pre-commit",
			wantViolation: true,
		},
		{
			name:    "Attack: Git Hook Injection via multi-command",
			command: "mkdir -p .git/hooks && echo 'evil' > .git/hooks/pre-commit && git status",
			wantViolation: true,
		},
		{
			name:    "Attack: Git HEAD modification",
			command: "echo 'main' > .git/HEAD",
			wantViolation: true,
		},
		{
			name:    "Attack: Git Object deletion",
			command: "rm -rf .git/objects",
			wantViolation: true,
		},
		{
			name:    "Attack: Git Hook bypass with dots",
			command: "echo 'evil' > ./.git/hooks/pre-push",
			wantViolation: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			out, err := sandbox.ExecuteContext(ctx, tt.command, "")
			if tt.wantViolation {
				if err == nil {
					t.Errorf("expected violation error for command %q, got nil", tt.command)
				}
				if !strings.Contains(out, "<sandbox_violations>") {
					t.Errorf("expected <sandbox_violations> in output for command %q, got: %s", tt.command, out)
				}
				if !strings.Contains(out, "detected suspicious access to git-internal paths") {
					t.Errorf("expected specific security error message, got: %s", out)
				}
			} else {
				if err != nil {
					t.Errorf("unexpected error for safe command %q: %v", tt.command, err)
				}
				if strings.Contains(out, "<sandbox_violations>") {
					t.Errorf("unexpected <sandbox_violations> in output for safe command %q", tt.command)
				}
			}
		})
	}
}
