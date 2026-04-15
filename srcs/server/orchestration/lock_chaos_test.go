package orchestration

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"testing"
)

func TestLock_ContentionResilience(t *testing.T) {
	tmpDir := t.TempDir()
	lockDir := filepath.Join(tmpDir, ".agent-lock")
	err := os.MkdirAll(lockDir, 0755)
	if err != nil {
		t.Fatalf("Failed to create lock dir: %v", err)
	}

	t.Run("Retry on Lock Contention", func(t *testing.T) {
		// Mock operation that fails with "database is locked" once then succeeds
		calls := 0
		op := func() error {
			calls++
			if calls == 1 {
				return fmt.Errorf("database is locked")
			}
			return nil
		}

		err := withSipRetry(context.Background(), op)
		if err != nil {
			t.Errorf("withSipRetry failed to handle transient lock: %v", err)
		}
		if calls != 2 {
			t.Errorf("Expected 2 calls due to retry, got %d", calls)
		}
	})

	t.Run("Simulate .agent-lock File Contention", func(t *testing.T) {
		lockFile := filepath.Join(lockDir, "mission_1.lock")
		err := os.WriteFile(lockFile, []byte("locked"), 0644)
		if err != nil {
			t.Fatalf("Failed to create lock file: %v", err)
		}

		// Make it un-deletable/un-writable to simulate another process holding it strictly
		err = os.Chmod(lockFile, 0400)
		if err != nil {
			t.Fatalf("Failed to chmod lock file: %v", err)
		}
		defer os.Chmod(lockFile, 0644)

		// Test logic that would attempt to acquire this lock
		// For now we just verify that we can detect the "busy" state
		_, err = os.OpenFile(lockFile, os.O_WRONLY, 0666)
		if err == nil {
			t.Errorf("Expected error opening read-only lock file for writing, got nil")
		} else {
			t.Logf("Detected lock contention: %v", err)
		}
	})
}
