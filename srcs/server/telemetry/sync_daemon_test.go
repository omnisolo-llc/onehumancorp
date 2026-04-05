package telemetry

import (
	"context"
	"testing"
	"time"
)

func TestSyncDaemon(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	callCount := 0
	syncFunc := func(ctx context.Context, endpoint string) (int, error) {
		callCount++
		if callCount == 1 {
			return 500, nil // triggers a second loop
		}
		return 0, nil
	}

	daemon := NewSyncDaemon(1*time.Millisecond, "http://localhost", syncFunc)
	daemon.Start(ctx)

	time.Sleep(10 * time.Millisecond)
	daemon.Stop()

	if callCount < 2 {
		t.Errorf("Expected at least 2 calls to syncFunc, got %d", callCount)
	}
}
