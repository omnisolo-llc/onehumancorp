package harness

import (
	"context"
	"testing"
)

type MockHarness struct{}

func (m *MockHarness) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	return []byte("success"), nil
}

func TestPermissionInterceptor(t *testing.T) {
	mockHarness := &MockHarness{}
	interceptor := NewPermissionInterceptor(mockHarness)
	ctx := context.Background()

	tests := []struct {
		name    string
		command []string
		wantErr bool
	}{
		{
			name:    "Safe command",
			command: []string{"echo", "hello"},
			wantErr: false,
		},
		{
			name:    "Sudo blocked",
			command: []string{"sudo", "ls"},
			wantErr: true,
		},
		{
			name:    "Sudo blocked inside bash -c",
			command: []string{"bash", "-c", "sudo ls"},
			wantErr: true,
		},
		{
			name:    "Subshell blocked",
			command: []string{"echo", "$(ls)"},
			wantErr: true,
		},
		{
			name:    "Redirection blocked",
			command: []string{"echo", "hello", ">", "file.txt"},
			wantErr: true,
		},
		{
			name:    "IFS blocked",
			command: []string{"IFS=:", "echo", "hello"},
			wantErr: true,
		},
		{
			name:    "zmodload blocked",
			command: []string{"zmodload", "zsh/net/tcp"},
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, err := interceptor.Execute(ctx, ExecutionContext{Command: tt.command})
			if (err != nil) != tt.wantErr {
				t.Errorf("Execute() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}
