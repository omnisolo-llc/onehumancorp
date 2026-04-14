package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

func TestAutoDreamSyncDaemon_StartStop(t *testing.T) {
	daemon := NewAutoDreamSyncDaemon(nil, 10*time.Millisecond)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	daemon.Start(ctx)

	time.Sleep(50 * time.Millisecond)

	daemon.Stop()

	// Wait for the goroutine to exit
	time.Sleep(20 * time.Millisecond)

	// Test is just to ensure Start and Stop don't panic or block
	assert.NotNil(t, daemon)
}
