package builtin

import (
	"context"
	"testing"
)

func TestASTCommandValidator(t *testing.T) {
	validator := NewASTCommandValidator()

	tests := []struct {
		name    string
		command string
		wantErr bool
	}{
		{
			name:    "safe command",
			command: `echo "hello world"`,
			wantErr: false,
		},
		{
			name:    "zmodload",
			command: "zmodload zsh/net/tcp",
			wantErr: true,
		},
		{
			name:    "emulate",
			command: "emulate sh",
			wantErr: true,
		},
		{
			name:    "process substitution read",
			command: "cat <(ls)",
			wantErr: true,
		},
		{
			name:    "process substitution write",
			command: "echo hi >(cat)",
			wantErr: true,
		},
		{
			name:    "zsh process substitution",
			command: "cat =ls",
			wantErr: true,
		},
		{
			name:    "sip.db access",
			command: "cat /var/lib/sip.db",
			wantErr: true,
		},
		{
			name:    "subshell",
			command: "( rm -rf / )",
			wantErr: true,
		},
		{
			name:    "command substitution",
			command: "echo $(whoami)",
			wantErr: true,
		},
		{
			name:    "command substitution backticks",
			command: "echo `whoami`",
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := validator.Validate(context.Background(), tt.command)
			if (err != nil) != tt.wantErr {
				t.Errorf("Validate() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}
