package orchestration

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"sync"
	"time"

	"onehumancorp/srcs/server/orchestration/harness"
)

// WorkerPool manages a pool of workers that pull tasks from a TaskQueue and execute them
type WorkerPool struct {
	queue          TaskQueue
	maxWorkers     int
	pollInterval   time.Duration
	wg             sync.WaitGroup
	cancel         context.CancelFunc
	harnessFactory func() (harness.AgentHarness, error)
}

// NewWorkerPool creates a new WorkerPool
func NewWorkerPool(queue TaskQueue, maxWorkers int, pollInterval time.Duration) *WorkerPool {
	return &WorkerPool{
		queue:        queue,
		maxWorkers:   maxWorkers,
		pollInterval: pollInterval,
		harnessFactory: func() (harness.AgentHarness, error) {
			// Default logic to use NewAssistantAgentHarness
			return harness.NewAssistantAgentHarness("")
		},
	}
}

// SetHarnessFactory allows overriding the harness creation logic (useful for testing)
func (p *WorkerPool) SetHarnessFactory(factory func() (harness.AgentHarness, error)) {
	p.harnessFactory = factory
}

// Start begins polling the queue for tasks
func (p *WorkerPool) Start(ctx context.Context) {
	poolCtx, cancel := context.WithCancel(ctx)
	p.cancel = cancel

	for i := 0; i < p.maxWorkers; i++ {
		p.wg.Add(1)
		go p.workerLoop(poolCtx, i)
	}
}

// Stop gracefully shuts down the worker pool, waiting for active tasks to finish
func (p *WorkerPool) Stop() {
	if p.cancel != nil {
		p.cancel()
	}
	p.wg.Wait()
}

func (p *WorkerPool) workerLoop(ctx context.Context, workerID int) {
	defer p.wg.Done()

	for {
		// Continuously drain the queue without static delays if tasks exist
		processed, err := p.processNextTask(ctx)
		if err != nil {
			if errors.Is(err, context.Canceled) {
				return
			}
			log.Printf("WorkerPool %d: error processing task: %v", workerID, err)
			// Small backoff on error
			select {
			case <-ctx.Done():
				return
			case <-time.After(p.pollInterval):
			}
			continue
		}

		if !processed {
			// Backoff if no tasks found
			select {
			case <-ctx.Done():
				return
			case <-time.After(p.pollInterval):
			}
		}
	}
}

// processNextTask returns true if a task was processed, false otherwise
func (p *WorkerPool) processNextTask(ctx context.Context) (bool, error) {
	task, err := p.queue.Dequeue(ctx)
	if err != nil {
		return false, err
	}

	if task == nil {
		// No tasks available
		return false, nil
	}

	err = p.executeTask(ctx, task)
	if err != nil {
		log.Printf("WorkerPool: failed to execute task %s: %v", task.ID, err)
		// Mark task as FAILED to prevent zombie running tasks
		failErr := p.queue.FailTask(ctx, task.ID)
		if failErr != nil {
			log.Printf("WorkerPool: failed to mark task %s as failed: %v", task.ID, failErr)
		}
		// We process the task (attempted) so return true to continue polling
		return true, nil
	}

	err = p.queue.Acknowledge(ctx, task.ID)
	if err != nil {
		log.Printf("WorkerPool: failed to acknowledge task %s: %v", task.ID, err)
	}

	return true, nil
}

func (p *WorkerPool) executeTask(ctx context.Context, task *Task) error {
	agentHarness, err := p.harnessFactory()
	if err != nil {
		return fmt.Errorf("failed to create harness: %w", err)
	}
	defer agentHarness.Reset()

	// Extract command from payload if present, otherwise default to "echo Hello" for now
	var cmdStr = "echo Hello"
	if len(task.Payload) > 0 {
		var payload map[string]interface{}
		if err := json.Unmarshal(task.Payload, &payload); err == nil {
			if cmd, ok := payload["command"].(string); ok {
				cmdStr = cmd
			}
		}
	}

	result, err := agentHarness.RunAttempt(cmdStr)
	if err != nil {
		return fmt.Errorf("harness run attempt failed: %w", err)
	}

	if result.ExitCode != 0 {
		return fmt.Errorf("harness returned non-zero exit code %d: %s", result.ExitCode, result.Stderr)
	}

	return nil
}

// ActiveWorkers is deprecated since pool scales statically by maxWorkers goroutines
func (p *WorkerPool) ActiveWorkers() int {
	return p.maxWorkers
}
