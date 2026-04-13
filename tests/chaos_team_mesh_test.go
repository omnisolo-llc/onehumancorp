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
