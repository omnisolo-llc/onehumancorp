package agents

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/src/server/interop"
)

func TestProviderGetCredentials(t *testing.T) {
	creds := Credentials{APIKey: "test-key"}

	tests := []struct {
		name     string
		provider Provider
	}{
		{"GeminiProvider", &GeminiProvider{}},
		{"OpenCodeProvider", &OpenCodeProvider{}},
		{"OpenClawProvider", &OpenClawProvider{}},
		{"IronClawProvider", &IronClawProvider{}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_ = tt.provider.Authenticate(creds)
			if tt.provider.GetCredentials().APIKey != "test-key" {
				t.Errorf("Expected test-key for %s GetCredentials", tt.name)
			}
		})
	}
}

func TestBuiltinProvider_RunInIsolation(t *testing.T) {
	// Create a temporary data dir for the test
	tempDir := t.TempDir()
	os.Setenv("OHC_DATA_DIR", tempDir)
	defer os.Unsetenv("OHC_DATA_DIR")

	// Create a dummy worktree and transport
	ctx := context.Background()
	worktree := filepath.Join(tempDir, "worktree")

	// Set a mock gRPC server to avoid real network
	address := "127.0.0.1:0"
	os.Setenv("OHC_AGENT_ADDRESS", address)
	defer os.Unsetenv("OHC_AGENT_ADDRESS")

	// Set handoff dir properly by passing the base dir manually
	// Or verify `store` using interop.NewFileHandoffStore() since it relies on OHC_DATA_DIR

	provider := &BuiltinProvider{}
	err := provider.RunInIsolation(ctx, worktree, nil)

	// It should fail with connection refused or missing dialing
	if err == nil {
		t.Errorf("Expected RunInIsolation to fail connecting to dummy address")
	}

	// But the handoff file should be created!
	store, _ := interop.NewFileHandoffStore() // Uses OHC_DATA_DIR automatically
	taskIDs, _ := store.ListHandoffs(ctx)
	if len(taskIDs) == 0 {
		t.Errorf("Expected handoff file to be created")
	}
}
