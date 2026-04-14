package tests

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/lib/resilience"
)

// TestMeshFallback_Contention simulates multiple agents attempting to write
// to the same .agent-lock directory, demonstrating WithRetry handles contention.
func TestMeshFallback_Contention(t *testing.T) {
	tmpDir := t.TempDir()
	lockDir := filepath.Join(tmpDir, ".agent-lock")
	if err := os.MkdirAll(lockDir, 0755); err != nil {
		t.Fatalf("Failed to create lock dir: %v", err)
	}
	lockFile := filepath.Join(lockDir, "mesh.lock")

	var wg sync.WaitGroup
	numAgents := 10
	successCount := 0
	var mu sync.Mutex

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	// Simulate agents
	for i := 0; i < numAgents; i++ {
		wg.Add(1)
		go func(agentID int) {
			defer wg.Done()

			err := resilience.WithRetry(ctx, 15, 50*time.Millisecond, func(c context.Context) error {
				// Attempt to create the lock file (simulate exclusive access)
				f, err := os.OpenFile(lockFile, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0666)
				if err != nil {
					return fmt.Errorf("agent %d failed to acquire lock: %w", agentID, err)
				}

				// Simulate some work while holding the lock
				time.Sleep(10 * time.Millisecond)

				// Properly close before removing
				f.Close()

				// Release the lock
				return os.Remove(lockFile)
			})

			if err == nil {
				mu.Lock()
				successCount++
				mu.Unlock()
			} else {
				t.Logf("Agent %d failed: %v", agentID, err)
			}
		}(i)
	}

	wg.Wait()

	if successCount != numAgents {
		t.Errorf("Expected %d agents to succeed, but got %d", numAgents, successCount)
	} else {
		t.Logf("All %d agents successfully acquired and released the lock via retry", numAgents)
	}
}

// TestMeshFallback_MaxRetries ensures that we eventually fail if the lock is held indefinitely.
func TestMeshFallback_MaxRetries(t *testing.T) {
	tmpDir := t.TempDir()
	lockDir := filepath.Join(tmpDir, ".agent-lock")
	if err := os.MkdirAll(lockDir, 0755); err != nil {
		t.Fatalf("Failed to create lock dir: %v", err)
	}
	lockFile := filepath.Join(lockDir, "mesh.lock")

	// Pre-create the lock file and hold it
	f, err := os.OpenFile(lockFile, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0666)
	if err != nil {
		t.Fatalf("Failed to setup initial lock: %v", err)
	}
	defer func() {
		f.Close()
		os.Remove(lockFile)
	}()

	ctx := context.Background()
	err = resilience.WithRetry(ctx, 3, 10*time.Millisecond, func(c context.Context) error {
		f2, err := os.OpenFile(lockFile, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0666)
		if err != nil {
			return err
		}
		f2.Close()
		return errors.New("should not reach here")
	})

	if err == nil {
		t.Error("Expected failure due to max retries, but succeeded")
	} else {
		t.Logf("Successfully failed after max retries: %v", err)
	}
}

// TestMeshFallback_ZeroBackoff tests behavior when initialBackoff is zero or negative
func TestMeshFallback_ZeroBackoff(t *testing.T) {
	ctx := context.Background()
	err := resilience.WithRetry(ctx, 1, 0, func(c context.Context) error {
		return errors.New("always fail")
	})
	if err == nil {
		t.Error("Expected error but got nil")
	}
}

// TestMeshFallback_Corruption simulates file corruption within the .agent-lock directory.
func TestMeshFallback_Corruption(t *testing.T) {
	tmpDir := t.TempDir()
	lockDir := filepath.Join(tmpDir, ".agent-lock")
	if err := os.MkdirAll(lockDir, 0755); err != nil {
		t.Fatalf("Failed to create lock dir: %v", err)
	}
	lockFile := filepath.Join(lockDir, "mesh.lock")

	// Corrupt the lock file
	if err := os.Mkdir(lockFile, 0755); err != nil {
		t.Fatalf("Failed to corrupt lock file: %v", err)
	}

	ctx := context.Background()
	err := resilience.WithRetry(ctx, 3, 10*time.Millisecond, func(c context.Context) error {
		f, err := os.OpenFile(lockFile, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0666)
		if err != nil {
			return err
		}
		f.Close()
		return nil
	})

	if err == nil {
		t.Error("Expected failure due to corrupted lock file (is a directory), but succeeded")
	} else {
		t.Logf("Successfully caught corruption failure: %v", err)
	}
}

// TestMeshFallback_ContextCancelled tests behavior when context is cancelled during retries
func TestMeshFallback_ContextCancelled(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	err := resilience.WithRetry(ctx, 5, 100*time.Millisecond, func(c context.Context) error {
		cancel() // cancel immediately so the sleep select catches it
		return errors.New("failing callback")
	})
	if err == nil {
		t.Error("Expected error but got nil")
	} else if !errors.Is(err, context.Canceled) {
		t.Errorf("Expected context.Canceled, got %v", err)
	}
}

// TestMeshFallback_ZeroJitter tests behavior when jitterVal is <= 0
func TestMeshFallback_ZeroJitter(t *testing.T) {
	ctx := context.Background()
	// initialBackoff of 1ns means jitterVal will be 1/2 = 0
	err := resilience.WithRetry(ctx, 1, 1*time.Nanosecond, func(c context.Context) error {
		return errors.New("always fail")
	})
	if err == nil {
		t.Error("Expected error but got nil")
	}
}
