package telemetry

import (
	"context"
	"errors"
	"testing"
	"time"
)

import "sync/atomic"

func TestSyncWorker_Start(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	var callCount atomic.Int32
	syncFunc := func(c context.Context, endpoint string) (int, error) {
		val := callCount.Add(1)
		if val == 1 {
			return 0, errors.New("simulated error")
		}
		return 5, nil
	}

	worker := NewSyncWorker(syncFunc, "http://localhost/sync", 10*time.Millisecond)

	go worker.Start(ctx)

	// Wait for a few ticks
	time.Sleep(50 * time.Millisecond)
	cancel()

	if callCount.Load() == 0 {
		t.Errorf("expected syncFunc to be called at least once")
	}
}
