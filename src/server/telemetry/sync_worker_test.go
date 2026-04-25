package telemetry

import (
	"context"
	"errors"
	"sync"
	"testing"
	"time"
)

func TestStartSyncDaemon(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())

	syncCount := 0
	syncFunc := func(c context.Context, endpoint string, batchSize int) (int, error) {
		syncCount++
		if syncCount == 1 {
			return 500, nil // First call simulates a full batch
		}
		if syncCount == 2 {
			return 100, nil // Second call simulates a partial batch
		}
		return 0, errors.New("simulated error") // Stop testing
	}

	// Ensure metrics are initialized
	mockM := &mockMeter{}
	_ = InitWithMeter(mockM)

	StartSyncDaemon(ctx, syncFunc, "http://localhost:8080/api/telemetry/sync", 10*time.Millisecond)

	time.Sleep(50 * time.Millisecond)
	cancel()
	time.Sleep(20 * time.Millisecond) // Give the goroutine time to exit

	if syncCount < 2 {
		t.Errorf("Expected syncFunc to be called at least 2 times, got %d", syncCount)
	}
}

func TestStartSyncDaemon_ContextCancel(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())

	syncCount := 0
	syncFunc := func(c context.Context, endpoint string, batchSize int) (int, error) {
		syncCount++
		return 10, nil
	}

	cancel() // Cancel immediately

	StartSyncDaemon(ctx, syncFunc, "http://localhost", 10*time.Millisecond)

	time.Sleep(50 * time.Millisecond)

	if syncCount > 0 {
		t.Errorf("Expected syncFunc to not be called after context cancel, got %d", syncCount)
	}
}

func TestStartSyncDaemon_AIMD(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())

	var mu sync.Mutex
	callCount := 0
	var requestedBatchSizes []int

	done := make(chan struct{})

	syncFunc := func(c context.Context, endpoint string, batchSize int) (int, error) {
		mu.Lock()
		defer mu.Unlock()

		callCount++
		requestedBatchSizes = append(requestedBatchSizes, batchSize)

		if callCount == 1 {
			return batchSize, nil // First call success
		}
		if callCount == 2 {
			return 0, errors.New("simulated error to trigger decrease") // Error triggers decrease to 250
		}
		if callCount >= 3 {
			if callCount == 4 {
				close(done)
			}
			return batchSize, nil // Success
		}

		return 0, nil
	}

	mockM := &mockMeter{}
	_ = InitWithMeter(mockM)

	StartSyncDaemon(ctx, syncFunc, "http://localhost:8080", 1*time.Millisecond)

	select {
	case <-done:
	case <-time.After(3 * time.Second):
		t.Fatal("Test timed out waiting for sync calls")
	}

	cancel()

	mu.Lock()
	defer mu.Unlock()

	if len(requestedBatchSizes) < 4 {
		t.Fatalf("Expected at least 4 calls to syncFunc, got %d", len(requestedBatchSizes))
	}

	if requestedBatchSizes[0] != 500 {
		t.Errorf("Expected first batch size 500, got %d", requestedBatchSizes[0])
	}
	if requestedBatchSizes[1] != 500 {
		t.Errorf("Expected second batch size 500, got %d", requestedBatchSizes[1])
	}
	if requestedBatchSizes[2] != 250 {
		t.Errorf("Expected third batch size 250 (after error), got %d", requestedBatchSizes[2])
	}
	if requestedBatchSizes[3] != 300 {
		t.Errorf("Expected fourth batch size 300 (after success), got %d", requestedBatchSizes[3])
	}
}