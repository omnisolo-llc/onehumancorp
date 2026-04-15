package bash_sandbox

import (
	"context"
	"path/filepath"
	"strings"
	"testing"
)

func TestValidateBashCommand(t *testing.T) {
	tests := []struct {
		name    string
		cmd     string
		wantErr bool
	}{
		{"Valid command", "echo 'hello'", false},
		{"Zsh expansion", "echo ${var#prefix}", true},
		{"Process substitution", "diff <(ls) <(ls -a)", true},
		{"Legacy expansion", "echo $[1+1]", true},
		{"zmodload", "zmodload zsh/net/tcp", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if err := ValidateBashCommand(tt.cmd); (err != nil) != tt.wantErr {
				t.Errorf("ValidateBashCommand() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}

func TestRunSandboxed(t *testing.T) {
	ctx := context.Background()

	// Set env isolation for tests
	t.Setenv("TEST_ENV", "isolated")

	tmpDir := t.TempDir()

	tests := []struct {
		name    string
		cmd     string
		policy  SandboxPolicy
		wantErr bool
		wantOut string
	}{
		{
			name:    "Valid execution",
			cmd:     "echo 'hello'",
			policy:  SandboxPolicy{},
			wantErr: false,
			wantOut: "hello\n",
		},
		{
			name:    "Validation failure",
			cmd:     "echo $[1+1]",
			policy:  SandboxPolicy{},
			wantErr: true,
			wantOut: "",
		},
		{
			name:    "Directory isolation",
			cmd:     "pwd",
			policy:  SandboxPolicy{WriteRestriction: []string{tmpDir}},
			wantErr: false,
			wantOut: tmpDir + "\n",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			wantOut := tt.wantOut
			// Resolve symlinks for temp dir on macOS
			if tt.name == "Directory isolation" {
				realTempDir, _ := filepath.EvalSymlinks(tmpDir)
				wantOut = realTempDir + "\n"
			}
			res, err := RunSandboxed(ctx, tt.cmd, tt.policy)
			if (err != nil) != tt.wantErr {
				t.Errorf("RunSandboxed() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if !tt.wantErr && strings.TrimSpace(res.Stdout) != strings.TrimSpace(wantOut) {
				t.Errorf("RunSandboxed() stdout = %v, want %v", res.Stdout, wantOut)
			}
		})
	}
}
