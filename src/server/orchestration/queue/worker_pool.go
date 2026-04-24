package queue

import (
	"context"
	"log/slog"
	"sync"
)

type JobHandlerFunc func(ctx context.Context, payload []byte) error

type WorkerPool struct {
	queue   JobQueue
	topic   string
	handler JobHandlerFunc
	workers int
	wg      sync.WaitGroup
}

func NewWorkerPool(queue JobQueue, topic string, workers int, handler JobHandlerFunc) *WorkerPool {
	return &WorkerPool{
		queue:   queue,
		topic:   topic,
		workers: workers,
		handler: handler,
	}
}

func (wp *WorkerPool) Start(ctx context.Context) {
	for i := 0; i < wp.workers; i++ {
		wp.wg.Add(1)
		go func(workerID int) {
			defer wp.wg.Done()
			for {
				select {
				case <-ctx.Done():
					return
				default:
					payload, err := wp.queue.Pop(ctx, wp.topic)
					if err != nil {
						if err != context.Canceled && err != context.DeadlineExceeded {
							slog.Error("Worker pool failed to pop job", "topic", wp.topic, "error", err)
						}
						continue
					}
					if payload == nil {
						continue // E.g., timeout or nil returned
					}

					if err := wp.handler(ctx, payload); err != nil {
						slog.Error("Worker pool handler failed", "topic", wp.topic, "error", err)
					}
				}
			}
		}(i)
	}
}

func (wp *WorkerPool) Wait() {
	wp.wg.Wait()
}
