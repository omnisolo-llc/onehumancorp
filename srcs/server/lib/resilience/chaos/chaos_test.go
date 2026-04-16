package chaos

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"
)

func TestInjectNetworkLatency(t *testing.T) {
	injector := NewDefaultInjector()
	ctx := context.Background()

	start := time.Now()
	err := injector.InjectNetworkLatency(ctx, 100*time.Millisecond)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	duration := time.Since(start)
	if duration < 100*time.Millisecond {
		t.Fatalf("expected at least 100ms latency, got %v", duration)
	}
}

func TestDropDatabaseConnections(t *testing.T) {
	injector := NewDefaultInjector()
	ctx := context.Background()

	// Test 0% drop
	err := injector.DropDatabaseConnections(ctx, 0.0)
	if err != nil {
		t.Fatalf("expected no drop at 0%%, got: %v", err)
	}

	// Test 100% drop
	err = injector.DropDatabaseConnections(ctx, 1.0)
	if err == nil {
		t.Fatalf("expected connection drop at 100%%, got nil")
	}

	// Test invalid percentage
	err = injector.DropDatabaseConnections(ctx, 1.5)
	if err == nil {
		t.Fatalf("expected error for invalid percentage, got nil")
	}
}

func TestSimulateResourceExhaustion(t *testing.T) {
	injector := NewDefaultInjector()
	ctx := context.Background()

	start := time.Now()
	err := injector.SimulateResourceExhaustion(ctx)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// The simulated exhaustion should take at least 500ms
	duration := time.Since(start)
	if duration < 500*time.Millisecond {
		t.Fatalf("expected exhaustion to take ~500ms, took %v", duration)
	}
}

func TestCorruptStateFiles(t *testing.T) {
	injector := NewDefaultInjector()
	ctx := context.Background()

	// Create a temporary directory and file
	tmpDir := t.TempDir()
	testFile := filepath.Join(tmpDir, ".agent-lock")

	// Write initial content
	err := os.WriteFile(testFile, []byte("valid_lock_data"), 0644)
	if err != nil {
		t.Fatalf("failed to create test file: %v", err)
	}

	// Run corruption injection
	err = injector.CorruptStateFiles(ctx, testFile)
	if err != nil {
		t.Fatalf("unexpected error during corruption: %v", err)
	}

	// Read corrupted content
	content, err := os.ReadFile(testFile)
	if err != nil {
		t.Fatalf("failed to read test file: %v", err)
	}

	// Verify corruption string exists
	if string(content) != "valid_lock_data\n<chaos>corrupted_by_chaos_injector</chaos>\n" {
		t.Fatalf("file corruption failed, got content: %s", content)
	}
}

func TestCorruptStateFiles_NotExists(t *testing.T) {
	injector := NewDefaultInjector()
	ctx := context.Background()

	err := injector.CorruptStateFiles(ctx, "/path/to/nonexistent/file")
	if err == nil {
		t.Fatalf("expected error when corrupting non-existent file, got nil")
	}
}
