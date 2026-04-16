package perf

import (
	"context"
	"sync/atomic"
	"testing"
)

func TestCoordinator(t *testing.T) {
	c := NewCoordinator(4)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	c.Start(ctx)

	var count int32
	numTasks := 100

	for i := 0; i < numTasks; i++ {
		c.Submit(func(ctx context.Context) error {
			atomic.AddInt32(&count, 1)
			return nil
		})
	}

	c.Stop()

	if int(count) != numTasks {
		t.Errorf("expected %d tasks completed, got %d", numTasks, count)
	}
}

func BenchmarkCoordinator(b *testing.B) {
    c := NewCoordinator(4)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	c.Start(ctx)

    b.ResetTimer()
    for i := 0; i < b.N; i++ {
        c.Submit(func(ctx context.Context) error {
			return nil
		})
    }

    c.Stop()
}
