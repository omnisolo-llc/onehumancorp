package builtin

import (
	"context"
	"testing"
)

func TestASTCommandValidator(t *testing.T) {
	validator := NewASTCommandValidator()
	ctx := context.Background()

	tests := []struct {
		name    string
		command string
		wantErr string
	}{
		{
			name:    "safe command",
			command: "echo hello",
			wantErr: "",
		},
		{
			name:    "blocked zmodload",
			command: "zmodload zsh/net/tcp",
			wantErr: "blocked command: zmodload",
		},
		{
			name:    "blocked emulate",
			command: "emulate -R zsh",
			wantErr: "blocked command: emulate",
		},
		{
			name:    "blocked process substitution read",
			command: "cat <(ls)",
			wantErr: "blocked process substitution",
		},
		{
			name:    "blocked process substitution write",
			command: "echo hi >(cat)",
			wantErr: "blocked process substitution",
		},
		{
			name:    "blocked zsh expansion",
			command: "cmd =(ls)",
			wantErr: "blocked process substitution",
		},
		{
			name:    "access sip db",
			command: "cat /tmp/sip.db",
			wantErr: "attempted access to OHC internal sip.db state files",
		},
		{
			name:    "access sip db quotes",
			command: "cat '/tmp/sip.db'",
			wantErr: "attempted access to OHC internal sip.db state files",
		},
		{
			name:    "nested dangerous command",
			command: "if true; then zmodload zsh/net/tcp; fi",
			wantErr: "blocked command: zmodload",
		},
		{
			name:    "command substitution",
			command: "echo $(zmodload)",
			wantErr: "blocked command: zmodload",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := validator.Validate(ctx, tt.command)
			if tt.wantErr == "" {
				if err != nil {
					t.Errorf("Validate() error = %v, wantErr %v", err, tt.wantErr)
				}
			} else {
				if err == nil || err.Error() != tt.wantErr {
					t.Errorf("Validate() error = %v, wantErr %v", err, tt.wantErr)
				}
			}
		})
	}
}
