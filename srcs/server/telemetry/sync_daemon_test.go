package telemetry

import (
	"context"
	"testing"
	"time"
)

func TestSyncDaemonLoop(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	mockM := &mockMeter{}
	_ = InitWithMeter(mockM)

	calledCount := 0
	syncFunc := func(ctx context.Context, endpoint string) (int, error) {
		calledCount++
		return 0, nil
	}

	StartSyncDaemon(ctx, syncFunc, "http://localhost:1234/sync", 10*time.Millisecond)

	time.Sleep(50 * time.Millisecond)
	cancel()

	if calledCount == 0 {
		t.Errorf("expected sync daemon to invoke syncFunc")
	}
}
