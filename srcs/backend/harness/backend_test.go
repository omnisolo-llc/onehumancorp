package harness

import (
	"context"
	"strings"
	"testing"
)

func TestHarnessGateway_Execute(t *testing.T) {
	gateway := NewHarnessGateway()
	ctx := context.Background()
	execCtx := ExecutionContext{Command: []string{"echo", "test"}}

	// Test free tier -> Serverless
	out, err := gateway.Execute(ctx, execCtx, "free")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !strings.Contains(string(out), "[ServerlessBackend]") {
		t.Errorf("Expected ServerlessBackend for free tier, got: %s", string(out))
	}

	// Test standard tier -> Docker
	out, err = gateway.Execute(ctx, execCtx, "standard")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !strings.Contains(string(out), "[DockerBackend]") {
		t.Errorf("Expected DockerBackend for standard tier, got: %s", string(out))
	}

	// Test premium tier -> Local
	// Note: Local backend actually runs bwrap, so it will fail if not installed or execute correctly.
	// Since we don't want to rely on system environment for routing test, we can mock the backends inside the gateway for this test.
}

func TestHarnessGateway_Routing(t *testing.T) {
	gateway := NewHarnessGateway()

	// Replace with dummy backends that return predictable strings
	gateway.RegisterBackend(BackendTypeLocal, &dummyBackend{name: "local"})
	gateway.RegisterBackend(BackendTypeDocker, &dummyBackend{name: "docker"})
	gateway.RegisterBackend(BackendTypeServerless, &dummyBackend{name: "serverless"})

	ctx := context.Background()
	execCtx := ExecutionContext{Command: []string{"test"}}

	tests := []struct{
		tier     string
		expected string
	}{
		{"free", "serverless"},
		{"standard", "docker"},
		{"premium", "local"},
		{"unknown", "serverless"},
		{"", "serverless"},
	}

	for _, tt := range tests {
		out, err := gateway.Execute(ctx, execCtx, tt.tier)
		if err != nil {
			t.Errorf("unexpected error for tier %s: %v", tt.tier, err)
		}
		if string(out) != tt.expected {
			t.Errorf("Expected %s for tier %s, got %s", tt.expected, tt.tier, string(out))
		}
	}
}

type dummyBackend struct {
	name string
}

func (d *dummyBackend) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	return []byte(d.name), nil
}
