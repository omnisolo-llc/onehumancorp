package queue

import (
	"context"
	"sync/atomic"
	"testing"
	"time"
)

func TestInMemJobQueue_WorkerPool(t *testing.T) {
	q := NewInMemJobQueue()
	testWorkerPoolWithQueue(t, q)
}

func testWorkerPoolWithQueue(t *testing.T, q JobQueue) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	var processedCount int32
	handler := func(ctx context.Context, payload []byte) error {
		atomic.AddInt32(&processedCount, 1)
		return nil
	}

	pool := NewWorkerPool(q, "test-topic", 3, handler)
	pool.Start(ctx)

	// Push some jobs
	for i := 0; i < 10; i++ {
		err := q.Push(ctx, "test-topic", []byte("hello"))
		if err != nil {
			t.Fatalf("Failed to push: %v", err)
		}
	}

	// Give workers time to process
	time.Sleep(100 * time.Millisecond)

	count := atomic.LoadInt32(&processedCount)
	if count != 10 {
		t.Fatalf("Expected 10 jobs processed, got %d", count)
	}

	cancel()
	pool.Wait()
}
