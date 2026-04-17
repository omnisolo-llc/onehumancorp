package orchestration

import (
	"context"
	"strings"
	"testing"
)

func TestBashASTValidator_SPIFFE(t *testing.T) {
	v := NewBashASTValidator()

	err := v.ValidateContext(context.Background(), "echo hello")
	if err == nil || !strings.Contains(err.Error(), "valid SPIFFE identity required") {
		t.Fatalf("Expected SPIFFE identity error, got: %v", err)
	}

	ctx := context.WithValue(context.Background(), SPIFFEContextKey{}, "spiffe://example.org/workload")
	err = v.ValidateContext(ctx, "echo hello")
	if err != nil {
		t.Fatalf("Did not expect error with valid SPIFFE identity, got: %v", err)
	}
}

func TestBashASTValidator_CommandValidation(t *testing.T) {
	v := NewBashASTValidator()
	ctx := context.WithValue(context.Background(), SPIFFEContextKey{}, "spiffe://example.org/workload")

	tests := []struct {
		name        string
		command     string
		wantErr     bool
		errContains string
	}{
		{
			name:    "Safe command",
			command: "echo hello > out.txt",
			wantErr: false,
		},
		{
			name:        "Prohibited redirect /etc",
			command:     "echo foo > /etc/passwd",
			wantErr:     true,
			errContains: "prohibited redirect target",
		},
		{
			name:        "Prohibited redirect /",
			command:     "echo foo > /",
			wantErr:     true,
			errContains: "prohibited redirect target",
		},
		{
			name:        "Dangerous command sudo",
			command:     "sudo ls",
			wantErr:     true,
			errContains: "prohibited command usage",
		},
		{
			name:        "Dangerous command chown",
			command:     "chown root file",
			wantErr:     true,
			errContains: "prohibited command usage",
		},
		{
			name:        "Dangerous rm target /",
			command:     "rm -rf /",
			wantErr:     true,
			errContains: "prohibited rm target",
		},
		{
			name:        "Dangerous rm target /*",
			command:     "rm -rf /*",
			wantErr:     true,
			errContains: "prohibited rm target",
		},
		{
			name:        "Process substitution output",
			command:     "echo >(ls)",
			wantErr:     true,
			errContains: "process substitution is prohibited",
		},
		{
			name:        "Process substitution input",
			command:     "cat <(ls)",
			wantErr:     true,
			errContains: "process substitution is prohibited",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := v.ValidateContext(ctx, tt.command)
			if (err != nil) != tt.wantErr {
				t.Errorf("ValidateContext() error = %v, wantErr %v", err, tt.wantErr)
			}
			if tt.wantErr && err != nil {
				if !strings.Contains(err.Error(), tt.errContains) {
					t.Errorf("Expected error to contain %q, got %v", tt.errContains, err)
				}
			}
		})
	}
}

func TestBashASTValidator_ExecuteContext(t *testing.T) {
	v := NewBashASTValidator()
	ctx := context.WithValue(context.Background(), SPIFFEContextKey{}, "spiffe://example.org/workload")

	res, err := v.ExecuteContext(ctx, "echo hello", "")
	if err != nil {
		t.Fatalf("Unexpected error from ExecuteContext: %v", err)
	}
	if res != "Execution Simulated" {
		t.Fatalf("Expected output 'Execution Simulated', got: %v", res)
	}

	_, err = v.ExecuteContext(ctx, "sudo echo", "")
	if err == nil {
		t.Fatalf("Expected error from ExecuteContext on dangerous command")
	}
}
