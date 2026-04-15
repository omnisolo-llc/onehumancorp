package resilience

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"sync"
	"testing"
	"time"
)

func TestMeshFallback_Contention(t *testing.T) {
	tmpDir := t.TempDir()
	lockDir := filepath.Join(tmpDir, ".agent-lock")
	if err := os.MkdirAll(lockDir, 0755); err != nil {
		t.Fatalf("Failed to create lock dir: %v", err)
	}
	lockFile := filepath.Join(lockDir, "mesh.lock")

	var wg sync.WaitGroup
	successes := 0
	var mu sync.Mutex

	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
			defer cancel()

			err := WithRetry(ctx, 3, 10*time.Millisecond, func(c context.Context) error {
				// Simulate some work that requires a lock
				f, err := os.OpenFile(lockFile, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0666)
				if err != nil {
					return fmt.Errorf("lock contention")
				}
				defer f.Close()
				defer os.Remove(lockFile)

				time.Sleep(5 * time.Millisecond) // Hold lock
				return nil
			})

			if err == nil {
				mu.Lock()
				successes++
				mu.Unlock()
			}
		}(i)
	}

	wg.Wait()
	if successes == 0 {
		t.Error("Expected at least some successes under contention")
	}
}

func TestMeshFallback_MaxRetries(t *testing.T) {
	ctx := context.Background()
	attempts := 0
	err := WithRetry(ctx, 3, 10*time.Millisecond, func(c context.Context) error {
		attempts++
		return errors.New("always fail")
	})

	if err == nil {
		t.Error("Expected error after max retries")
	}
	if attempts != 4 { // Initial + 3 retries
		t.Errorf("Expected 4 attempts, got %d", attempts)
	}
}

func TestMeshFallback_StandaloneParity(t *testing.T) {
	// Standalone mode test parity
	ctx := context.Background()
	attempts := 0

	// Simulate an operation that initially fails (e.g., due to local SQLite lock contention)
	// but succeeds on a subsequent retry, verifying WithRetry recovers it gracefully.
	err := WithRetry(ctx, 3, 10*time.Millisecond, func(c context.Context) error {
		attempts++
		if attempts < 2 {
			return errors.New("database is locked (simulated SQLite)")
		}
		return nil
	})

	if err != nil {
		t.Errorf("Expected success after retry, got: %v", err)
	}
	if attempts != 2 {
		t.Errorf("Expected 2 attempts, got %d", attempts)
	}
}

func TestMeshFallback_ZeroBackoff(t *testing.T) {
	ctx := context.Background()
	err := WithRetry(ctx, 1, 0, func(c context.Context) error {
		return errors.New("always fail")
	})
	if err == nil {
		t.Error("Expected error but got nil")
	}
}

func TestMeshFallback_ContextCancelled(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 50*time.Millisecond)
	err := WithRetry(ctx, 5, 100*time.Millisecond, func(c context.Context) error {
		cancel()
		return errors.New("failing callback")
	})
	if err == nil {
		t.Error("Expected error but got nil")
	} else if !errors.Is(err, context.Canceled) {
		t.Errorf("Expected context.Canceled, got %v", err)
	}
}

func TestMeshFallback_ZeroJitter(t *testing.T) {
	ctx := context.Background()
	err := WithRetry(ctx, 1, 1*time.Nanosecond, func(c context.Context) error {
		return errors.New("always fail")
	})
	if err == nil {
		t.Error("Expected error but got nil")
	}
}
