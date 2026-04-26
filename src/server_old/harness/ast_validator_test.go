package harness

import (
	"context"
	"testing"
)

func TestASTValidator(t *testing.T) {
	validator := NewASTValidator()

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
			name:    "command substitution",
			command: `echo "su"$(echo "do")`,
			wantErr: true,
		},
		{
			name:    "file redirection out",
			command: `ls -la > /tmp/out`,
			wantErr: true,
		},
		{
			name:    "file redirection in",
			command: `cat < /etc/passwd`,
			wantErr: true,
		},
        {
            name:    "backticks",
            command: "echo `whoami`",
            wantErr: true,
        },
        {
            name:    "subshell",
            command: "( rm -rf / )",
            wantErr: true,
        },
		{
			name:    "IFS injection",
			command: "IFS=:; ls",
			wantErr: true,
		},
		{
			name:    "zmodload",
			command: "zmodload zsh/net/tcp",
			wantErr: true,
		},
		{
			name:    "sudo",
			command: "sudo apt-get install",
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := validator.Validate(context.Background(), tt.command)
			if (err != nil) != tt.wantErr {
				t.Errorf("ASTValidator.Validate() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}
