package orchestration

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"testing"
	"time"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestWithSipRetry_ExponentialBackoff(t *testing.T) {
	t.Run("Exponential backoff delays correctly", func(t *testing.T) {
		calls := 0
		op := func() error {
			calls++
			if calls < 3 {
				return fmt.Errorf("database is locked")
			}
			return nil
		}

		start := time.Now()
		err := withSipRetry(context.Background(), op)
		duration := time.Since(start)

		if err != nil {
			t.Errorf("Expected success, got %v", err)
		}
		if calls != 3 {
			t.Errorf("Expected 3 calls, got %d", calls)
		}

		// Expected delays:
		// Attempt 1 fails -> wait 10ms
		// Attempt 2 fails -> wait 20ms
		// Attempt 3 succeeds
		// Minimum expected duration: 10ms + 20ms = 30ms.
		// Maximum expected duration: 30ms + generous buffer for test execution overhead (e.g. 50ms)
		if duration < 30*time.Millisecond {
			t.Errorf("Expected duration to be at least 30ms, got %v", duration)
		}
	})
}

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
	t.Run("Retry with Telemetry", func(t *testing.T) {
		called := 0
		telemetry.BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
			if metricType == "sqlite_retry_event" {
				called++
			}
			return nil
		}
		defer func() { telemetry.BufferMetricFunc = nil }()

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
			t.Errorf("withSipRetry failed: %v", err)
		}
		if called != 1 {
			t.Errorf("Expected 1 call to telemetry, got %d", called)
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
