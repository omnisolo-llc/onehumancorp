package validation

import (
	"context"
	"testing"
	"sync"

	"github.com/stretchr/testify/assert"
)

type mockStore struct {
	mu         sync.Mutex
	violations []string
}

func (m *mockStore) RecordViolation(ctx context.Context, cmd string, errDetails string) {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.violations = append(m.violations, errDetails)
}

func (m *mockStore) GetViolations() []string {
	m.mu.Lock()
	defer m.mu.Unlock()
	res := make([]string, len(m.violations))
	copy(res, m.violations)
	return res
}

func TestBashASTValidator(t *testing.T) {
	store := &mockStore{}
	validator := NewBashASTValidator(store)
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
			name:    "blocked builtin eval",
			command: "eval 'ls -la'",
			wantErr: "blocked command: eval",
		},
		{
			name:    "blocked builtin exec",
			command: "exec bash",
			wantErr: "blocked command: exec",
		},
		{
			name:    "blocked alias",
			command: "alias rm='rm -rf /'",
			wantErr: "blocked command: alias",
		},
		{
			name:    "blocked zmodload",
			command: "zmodload zsh/net/tcp",
			wantErr: "blocked command: zmodload",
		},
		{
			name:    "blocked rm -rf /",
			command: "rm -rf /",
			wantErr: "blocked dangerous command: rm -rf /",
		},
		{
			name:    "blocked rm -rf /*",
			command: "rm -rf /*",
			wantErr: "blocked dangerous command: rm -rf /",
		},
		{
			name:    "blocked sudo",
			command: "sudo ls",
			wantErr: "blocked command: sudo",
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
			name:    "blocked network redirect tcp",
			command: "echo hi > /dev/tcp/1.2.3.4/8080",
			wantErr: "blocked network redirection to /dev/tcp/1.2.3.4/8080",
		},
		{
			name:    "blocked network redirect udp",
			command: "cat < /dev/udp/1.2.3.4/53",
			wantErr: "blocked network redirection to /dev/udp/1.2.3.4/53",
		},
		{
			name:    "nested dangerous command",
			command: "if true; then sudo rm -rf /; fi",
			wantErr: "blocked command: sudo",
		},
		{
			name:    "command substitution",
			command: "echo $(eval ls)",
			wantErr: "blocked command: eval",
		},
		{
			name:    "safe command with similar word",
			command: "echo eval",
			wantErr: "",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := validator.Validate(ctx, tt.command)
			if tt.wantErr == "" {
				assert.NoError(t, err)
			} else {
				assert.Error(t, err)
				assert.Contains(t, err.Error(), tt.wantErr)
			}
		})
	}

	// Verify store
	violations := store.GetViolations()
	assert.GreaterOrEqual(t, len(violations), 1)
}
