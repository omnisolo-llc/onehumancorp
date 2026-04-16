package queue

import (
	"context"
	"fmt"
	"sync"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

// SubAgentHarness defines the interface for running a command.
type SubAgentHarness interface {
	Run(ctx context.Context, cmd string, args []string) error
}

// WorkerPool manages concurrent workers polling the TaskQueue.
type WorkerPool struct {
	queue   TaskQueue
	harness SubAgentHarness
	workers int
	wg      sync.WaitGroup
	quit    chan struct{}

	meter   metric.Meter
	counter metric.Int64Counter
	duration metric.Float64Histogram
}

// NewWorkerPool creates a new worker pool.
func NewWorkerPool(queue TaskQueue, harness SubAgentHarness, workers int) (*WorkerPool, error) {
	meter := otel.Meter("ohc_orchestration_queue")
	counter, err := meter.Int64Counter("ohc_sub_agent_queue_length",
		metric.WithDescription("Length of the sub agent queue"),
	)
	if err != nil {
		return nil, fmt.Errorf("failed to create metric counter: %w", err)
	}

	duration, err := meter.Float64Histogram("ohc_sub_agent_execution_duration_seconds",
		metric.WithDescription("Execution duration of sub agents"),
	)
	if err != nil {
		return nil, fmt.Errorf("failed to create metric histogram: %w", err)
	}

	return &WorkerPool{
		queue:    queue,
		harness:  harness,
		workers:  workers,
		quit:     make(chan struct{}),
		meter:    meter,
		counter:  counter,
		duration: duration,
	}, nil
}

// Start begins processing tasks from the queue.
func (p *WorkerPool) Start(ctx context.Context) {
	for i := 0; i < p.workers; i++ {
		p.wg.Add(1)
		go p.worker(ctx)
	}
}

// Stop gracefully stops the worker pool.
func (p *WorkerPool) Stop() {
	close(p.quit)
	p.wg.Wait()
}

func (p *WorkerPool) worker(ctx context.Context) {
	defer p.wg.Done()
	for {
		select {
		case <-p.quit:
			return
		case <-ctx.Done():
			return
		default:
			task, err := p.queue.Acquire(ctx)
			if err != nil {
				// Backoff on error or empty queue
				time.Sleep(1 * time.Second)
				continue
			}
			if task == nil {
				time.Sleep(1 * time.Second)
				continue
			}

			start := time.Now()
			err = p.harness.Run(ctx, task.Command, task.Args)
			duration := time.Since(start).Seconds()
			p.duration.Record(ctx, duration)

			if err != nil {
				// Exponential backoff strategy: 2^attempts seconds
				retryAfter := time.Duration(1<<task.Attempts) * time.Second
				_ = p.queue.Fail(ctx, task.ID, retryAfter)
			} else {
				_ = p.queue.Complete(ctx, task.ID)
			}
		}
	}
}
