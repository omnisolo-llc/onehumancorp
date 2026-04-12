package orchestration

import (
	"context"
	"path/filepath"
	"testing"
	"time"
)

// TestSIPDB_ChaosPanic simulates an abrupt panic during a database operation
// to ensure the standalone semaphore is properly released and doesn't cause a livelock.
func TestSIPDB_ChaosPanic(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")
	defer ClearSemaphore()

	tmpDir := t.TempDir()
	dbPath := filepath.Join(tmpDir, "chaos_panic.db")

	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create SIPDB: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	// Use withSipRetry with a panic to simulate an abrupt failure
	var paniced bool
	func() {
		defer func() {
			if r := recover(); r != nil {
				paniced = true
			}
		}()
		_ = withSipRetry(ctx, func() error {
			panic("simulated abrupt failure")
			return nil
		})
	}()

	if !paniced {
		t.Fatalf("Expected panic to occur")
	}

	// Now try another operation to ensure the semaphore was released
	errCh := make(chan error, 1)
	go func() {
		errCh <- withSipRetry(ctx, func() error {
			return nil
		})
	}()

	select {
	case err := <-errCh:
		if err != nil {
			t.Fatalf("Operation after panic failed: %v", err)
		}
		t.Log("Successfully verified semaphore release after panic")
	case <-time.After(1 * time.Second):
		t.Fatalf("Operation after panic timed out, indicating a livelock/leaked semaphore")
	}
}
