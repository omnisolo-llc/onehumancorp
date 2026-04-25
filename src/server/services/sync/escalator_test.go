package sync

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/orchestration"
)

func TestSyncEscalator(t *testing.T) {
	hub := orchestration.NewHub()
	defer hub.Close()

	escalator := NewSyncEscalator(hub)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	escalator.Start(ctx, 10*time.Millisecond)

	// Let it run for a bit to ensure no panics
	time.Sleep(50 * time.Millisecond)
}
