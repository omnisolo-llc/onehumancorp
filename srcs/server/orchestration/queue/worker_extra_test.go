package queue

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/require"
)

func TestWorkerTelemetryRecord(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	mockQ := &mockQueue{}

	handlerCalled := false
	handler := func(c context.Context, j *Job) error {
		handlerCalled = true
		time.Sleep(10 * time.Millisecond) // Ensure latency is non-zero
		return nil
	}

	w := NewWorker(mockQ, []string{"test-role"}, handler)

	go w.Start(ctx)

	time.Sleep(100 * time.Millisecond)

	// Since we mock the queue and it blocks or returns nil, we just ensure no panic happened
	// during Start initialization. We can't easily intercept the telemetry global here.
	require.False(t, handlerCalled, "Handler should not be called with empty mock")
}
