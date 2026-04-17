package chaos

import (
	"context"
	"os"
	"testing"
)

func TestCorruptAgentLock(t *testing.T) {
	// Create a temporary directory to act as the working directory
	tempDir, err := os.MkdirTemp("", "chaos_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	// Change working directory to temp dir
	originalWd, err := os.Getwd()
	if err != nil {
		t.Fatalf("failed to get current working directory: %v", err)
	}
	defer os.Chdir(originalWd)

	err = os.Chdir(tempDir)
	if err != nil {
		t.Fatalf("failed to change working directory: %v", err)
	}

	// Create the .agent-lock directory
	lockPath := ".agent-lock/"
	err = os.Mkdir(lockPath, 0755)
	if err != nil {
		t.Fatalf("failed to create .agent-lock directory: %v", err)
	}

	inj := NewInjector(CorruptAgentLock, 1)
	err = inj.Inject(context.Background())

	if err == nil {
		t.Fatal("expected error, got nil")
	}

	if e, ok := err.(*ChaosError); !ok || e.Message != "chaos: simulated agent lock corruption" {
		t.Fatalf("expected ChaosError with correct message, got: %v", err)
	}

	// Verify that the file was created
	content, err := os.ReadFile(lockPath + "corrupt.lock")
	if err != nil {
		t.Fatalf("failed to read corrupt.lock: %v", err)
	}

	if string(content) != "chaos corrupted this lock" {
		t.Fatalf("expected 'chaos corrupted this lock', got %s", string(content))
	}
}

func TestCorruptAgentLock_NoDir(t *testing.T) {
	// Create a temporary directory to act as the working directory
	tempDir, err := os.MkdirTemp("", "chaos_test_no_dir")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	// Change working directory to temp dir
	originalWd, err := os.Getwd()
	if err != nil {
		t.Fatalf("failed to get current working directory: %v", err)
	}
	defer os.Chdir(originalWd)

	err = os.Chdir(tempDir)
	if err != nil {
		t.Fatalf("failed to change working directory: %v", err)
	}

	inj := NewInjector(CorruptAgentLock, 1)
	err = inj.Inject(context.Background())

	if err == nil {
		t.Fatal("expected error, got nil")
	}

	if e, ok := err.(*ChaosError); !ok || e.Message != "chaos: simulated agent lock corruption" {
		t.Fatalf("expected ChaosError with correct message, got: %v", err)
	}

	// Verify that the file was NOT created since the directory doesn't exist
	_, err = os.Stat(".agent-lock/corrupt.lock")
	if !os.IsNotExist(err) {
		t.Fatalf("expected file to not exist, got err: %v", err)
	}
}
