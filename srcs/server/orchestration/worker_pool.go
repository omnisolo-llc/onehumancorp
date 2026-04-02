package orchestration

import (
	"context"
	"fmt"
	"log/slog"
	"sync"
	"time"
)

type WorkerPool struct {
	orchestrator TaskOrchestrator
	agentID      string
	capabilities []string
	concurrency  int
	stopCh       chan struct{}
	wg           sync.WaitGroup
}

func NewWorkerPool(orchestrator TaskOrchestrator, agentID string, capabilities []string, concurrency int) *WorkerPool {
	return &WorkerPool{
		orchestrator: orchestrator,
		agentID:      agentID,
		capabilities: capabilities,
		concurrency:  concurrency,
		stopCh:       make(chan struct{}),
	}
}

func (wp *WorkerPool) Start(ctx context.Context) {
	for i := 0; i < wp.concurrency; i++ {
		wp.wg.Add(1)
		go wp.worker(ctx)
	}
}

func (wp *WorkerPool) Stop() {
	close(wp.stopCh)
	wp.wg.Wait()
}

func (wp *WorkerPool) worker(ctx context.Context) {
	defer wp.wg.Done()
	ticker := time.NewTicker(2 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-wp.stopCh:
			return
		case <-ticker.C:
			wp.processTasks(ctx)
		}
	}
}

func (wp *WorkerPool) processTasks(ctx context.Context) {
	// Attempt to acquire a task
	task, err := wp.orchestrator.AcquireReadyTask(ctx, wp.agentID, wp.capabilities)
	if err != nil {
		slog.Error("Worker failed to acquire task", "error", err)
		return
	}
	if task == nil {
		return // No tasks ready
	}

	// Simulate processing time
	time.Sleep(100 * time.Millisecond)

	// Complete task
	result := fmt.Sprintf("Completed by %s", wp.agentID)
	err = wp.orchestrator.CompleteReadyTask(ctx, task.ID, result)
	if err != nil {
		slog.Error("Worker failed to complete task", "error", err)
	}
}
