package validation

import (
    "testing"

    "github.com/stretchr/testify/assert"
)

func TestASTValidator(t *testing.T) {
    config := Config{
        BlockedCommands: []string{"sudo", "su", "zmodload", "chmod"},
    }
    validator := NewASTValidator(config)

    tests := []struct {
        name    string
        cmd     string
        wantErr bool
    }{
        {
            name:    "safe command",
            cmd:     "ls -la",
            wantErr: false,
        },
        {
            name:    "blocked command",
            cmd:     "sudo ls -la",
            wantErr: true,
        },
        {
            name:    "blocked command with semicolon",
            cmd:     "echo hello; su",
            wantErr: true,
        },
        {
            name:    "subshell evasion simple",
            cmd:     "$(echo su)",
            wantErr: true, // dynamic execution for command name is blocked
        },
        {
            name:    "piped blocked command",
            cmd:     "cat file | sudo sh",
            wantErr: true,
        },
        {
            name:    "quotes evasion",
            cmd:     "s'u'do",
            wantErr: true,
        },
        {
            name:    "double quotes evasion",
            cmd:     "s\"u\"do",
            wantErr: true,
        },
        {
            name:    "variable evasion",
            cmd:     "x=su; $x",
            wantErr: true, // dynamic execution for command name is blocked
        },
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            err := validator.Validate(tt.cmd)
            if tt.wantErr {
                assert.Error(t, err)
            } else {
                assert.NoError(t, err)
            }
        })
    }
}
