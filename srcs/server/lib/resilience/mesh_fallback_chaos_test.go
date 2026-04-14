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
	numAgents := 10
	successCount := 0
	var mu sync.Mutex

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	for i := 0; i < numAgents; i++ {
		wg.Add(1)
		go func(agentID int) {
			defer wg.Done()

			err := WithRetry(ctx, 15, 50*time.Millisecond, func(c context.Context) error {
				f, err := os.OpenFile(lockFile, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0666)
				if err != nil {
					return fmt.Errorf("agent %d failed to acquire lock: %w", agentID, err)
				}

				time.Sleep(10 * time.Millisecond)
				f.Close()
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

func TestMeshFallback_MaxRetries(t *testing.T) {
	tmpDir := t.TempDir()
	lockDir := filepath.Join(tmpDir, ".agent-lock")
	if err := os.MkdirAll(lockDir, 0755); err != nil {
		t.Fatalf("Failed to create lock dir: %v", err)
	}
	lockFile := filepath.Join(lockDir, "mesh.lock")

	f, err := os.OpenFile(lockFile, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0666)
	if err != nil {
		t.Fatalf("Failed to setup initial lock: %v", err)
	}
	defer func() {
		f.Close()
		os.Remove(lockFile)
	}()

	ctx := context.Background()
	err = WithRetry(ctx, 3, 10*time.Millisecond, func(c context.Context) error {
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
	ctx, cancel := context.WithCancel(context.Background())
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
