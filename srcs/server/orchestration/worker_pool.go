package orchestration

import (
	"context"
	"fmt"
	"sync"
	"time"
)

// TaskHandler is a function type for processing tasks.
type TaskHandler func(ctx context.Context, task *Task) error

// WorkerPool manages a pool of goroutines that process tasks from a TaskQueue.
type WorkerPool struct {
	queue       TaskQueue
	workerCount int
	handler     TaskHandler
	wg          sync.WaitGroup
	cancel      context.CancelFunc
}

// NewWorkerPool creates a new WorkerPool instance.
func NewWorkerPool(queue TaskQueue, workerCount int, handler TaskHandler) *WorkerPool {
	return &WorkerPool{
		queue:       queue,
		workerCount: workerCount,
		handler:     handler,
	}
}

// Start begins processing tasks from the queue using the specified number of workers.
func (wp *WorkerPool) Start(ctx context.Context) {
	poolCtx, cancel := context.WithCancel(ctx)
	wp.cancel = cancel

	for i := 0; i < wp.workerCount; i++ {
		wp.wg.Add(1)
		go wp.workerLoop(poolCtx, fmt.Sprintf("worker-%d", i))
	}
}

func (wp *WorkerPool) workerLoop(ctx context.Context, workerID string) {
	defer wp.wg.Done()
	ticker := time.NewTicker(100 * time.Millisecond) // Polling interval
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			// Continuously process tasks as long as they are available
			for {
				if ctx.Err() != nil {
					return
				}

				task, err := wp.queue.Dequeue(ctx, workerID)
				if err != nil {
					// Log error in real system, but continue polling
					break
				}
				if task == nil {
					// Queue empty, wait for next tick
					break
				}

				// Process the task
				err = wp.handler(ctx, task)
				if err == nil {
					// Acknowledge on success
					_ = wp.queue.Acknowledge(ctx, task.ID)
				} else {
					// In a full implementation we might retry or fail the task.
					// For this test/spec, we just acknowledge or log.
				}
			}
		}
	}
}

// Stop gracefully shuts down the worker pool and waits for active workers to finish.
func (wp *WorkerPool) Stop() {
	if wp.cancel != nil {
		wp.cancel()
	}
	wp.wg.Wait()
}
