package terminal

import (
	"context"
	"testing"
)

func TestASTValidator_Validate(t *testing.T) {
	ctx := context.Background()
	validator := NewASTValidator(DefaultValidatorConfig())

	tests := []struct {
		name    string
		command string
		wantErr bool
	}{
		{
			name:    "safe command",
			command: "ls -la /tmp",
			wantErr: false,
		},
		{
			name:    "blocked command zmodload",
			command: "zmodload zsh/net/tcp",
			wantErr: true,
		},
		{
			name:    "blocked command in pipeline",
			command: "cat file.txt | grep foo | nc localhost 8080",
			wantErr: true,
		},
		{
			name:    "destructive rm -rf",
			command: "rm -rf /tmp/foo",
			wantErr: true,
		},
		{
			name:    "destructive rm -r -f",
			command: "rm -r -f /tmp/foo",
			wantErr: true,
		},
		{
			name:    "destructive rm -fr",
			command: "rm -fr /tmp/foo",
			wantErr: true,
		},
		{
			name:    "safe rm",
			command: "rm file.txt",
			wantErr: false,
		},
		{
			name:    "safe rm -r",
			command: "rm -r dir",
			wantErr: false,
		},
		{
			name:    "obfuscated blocked command",
			command: "z\\mod\\load zsh/net/tcp",
			wantErr: true,
		},
		{
			name:    "obfuscated blocked command with quotes",
			command: "\"zmodload\" zsh/net/tcp",
			wantErr: true,
		},
		{
			name:    "obfuscated blocked command with single quotes",
			command: "'zmodload' zsh/net/tcp",
			wantErr: true,
		},
		{
			name:    "destructive rm -rf with quotes",
			command: "rm \"-rf\" /tmp/foo",
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := validator.Validate(ctx, tt.command)
			if (err != nil) != tt.wantErr {
				t.Errorf("Validate() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}
