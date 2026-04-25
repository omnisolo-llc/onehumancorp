package telemetry

import (
	"context"
	"errors"
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

	StartSyncDaemon(ctx, syncFunc, nil, "http://localhost:8080/api/telemetry/sync", 10*time.Millisecond)

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

	StartSyncDaemon(ctx, syncFunc, nil, "http://localhost", 10*time.Millisecond)

	time.Sleep(50 * time.Millisecond)

	if syncCount > 0 {
		t.Errorf("Expected syncFunc to not be called after context cancel, got %d", syncCount)
	}
}


func TestStartSyncDaemon_AIMD(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())

	syncCount := 0
	syncFunc := func(c context.Context, endpoint string, batchSize int) (int, error) {
		syncCount++
		if syncCount == 1 {
			// Success -> should increase batch size
			if batchSize != 500 {
				t.Errorf("Expected initial batch size 500, got %d", batchSize)
			}
			return 100, nil
		}
		if syncCount == 2 {
			// Simulated error -> should halve batch size
			if batchSize != 600 {
				t.Errorf("Expected increased batch size 600, got %d", batchSize)
			}
			return 0, errors.New("timeout: simulated network error")
		}
		if syncCount == 3 {
			// After error -> should be halved
			if batchSize != 300 {
				t.Errorf("Expected halved batch size 300, got %d", batchSize)
			}
			return 100, nil
		}
		return 0, errors.New("simulated error") // Stop testing
	}

	depthFunc := func(c context.Context) (int, error) {
		return 42, nil
	}

	mockM := &mockMeter{}
	_ = InitWithMeter(mockM)

	StartSyncDaemon(ctx, syncFunc, depthFunc, "http://localhost:8080/api/telemetry/sync", 5*time.Millisecond)

	time.Sleep(2100 * time.Millisecond)
	cancel()
	time.Sleep(20 * time.Millisecond) // Give the goroutine time to exit

	if syncCount < 3 {
		t.Errorf("Expected syncFunc to be called at least 3 times, got %d", syncCount)
	}
}
